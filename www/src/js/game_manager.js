import Grid from './grid.js';
import Tile from './tile.js';
import axios from 'axios';
import {v4 as uuidv4 } from 'uuid';

export default function GameManager(
  size,
  InputManager,
  Actuator,
  StorageManager,
  name,
  wasm,
  isTesting,
  isAi
) {
  this.size = size; // Size of the grid
  this.inputManager = new InputManager(name);
  this.storageManager = new StorageManager(name);
  this.actuator = new Actuator();
  this.isTesting = isTesting;
  this.name = name;
  if (isTesting) {
    this.wasm = wasm;
  }
  this.isAi = isAi;

  this.startTiles = 2;

  this.inputManager.on(name + '_move', this.move.bind(this));
  this.inputManager.on(name + '_restart', this.restart.bind(this));
  this.inputManager.on(name + '_keepPlaying', this.keepPlaying.bind(this));

  this.setup();

  return this;
}

GameManager.prototype.refresh = function () {
	return this;
}

// Restart the game
GameManager.prototype.restart = function () {
  this.storageManager.clearGameState();
  this.actuator.continueGame(); // Clear the game won/lost message
  this.setup();
};

// Keep playing after winning (allows going over 2048)
GameManager.prototype.keepPlaying = function () {
  this.keepPlaying = true;
  this.actuator.continueGame(); // Clear the game won/lost message
};

// Return true if the game is lost, or has won and the user hasn't kept playing
GameManager.prototype.isGameTerminated = function () {
  return this.over || (this.won && !this.keepPlaying);
};

// Set up the game
GameManager.prototype.setup = function () {
  var previousState = this.storageManager.getGameState();

  // Reload the game from a previous game if present
  if (previousState) {
    this.grid = new Grid(previousState.grid.size, previousState.grid.cells); // Reload grid
    this.score = previousState.score;
    this.over = previousState.over;
    this.won = previousState.won;
    this.keepPlaying = previousState.keepPlaying;
    if (this.isTesting) {
      this.gameString = previousState.gameString;
      this.lastMoveTime = previousState.lastMoveTime;
      this.state = previousState.state;
    }
  } else {
    this.grid = new Grid(this.size);
    this.score = 0;
    this.over = false;
    this.won = false;
    this.keepPlaying = false;
    if (this.isTesting) {
      this.gameString = '';
      this.lastMoveTime = Date.now();
    }

    // Add the initial tiles
    this.addStartTiles();
  }

  // Update the actuator
  this.actuate();

  if (this.isTesting) {
    this.state = this.gridCellsToHexString(this.grid.cells);
    this.gameString += this.gridCellsToHexString(this.grid.cells);
    this.gameString += ',';
  }
};

// Set up the initial tiles to start the game with
GameManager.prototype.addStartTiles = function () {
  for (var i = 0; i < this.startTiles; i++) {
    this.addRandomTile();
  }
};

// Adds a tile in a random position
GameManager.prototype.addRandomTile = function () {
  if (this.grid.cellsAvailable()) {
    var value = Math.random() < 0.9 ? 2 : 4;
    var tile = new Tile(this.grid.randomAvailableCell(), value);

    this.grid.insertTile(tile);
  }
};

// Sends the updated grid to the actuator
GameManager.prototype.actuate = function () {
  if (this.storageManager.getBestScore() < this.score) {
    this.storageManager.setBestScore(this.score);
  }

  // Clear the state when the game is over (game over only, not win)
  if (this.over) {
    this.storageManager.clearGameState();
  } else {
    this.storageManager.setGameState(this.serialize());
  }

  this.actuator.actuate(this.grid, {
    score: this.score,
    over: this.over,
    won: this.won,
    bestScore: this.storageManager.getBestScore(),
    terminated: this.isGameTerminated(),
  });
};

// Represent the current game as an object
GameManager.prototype.serialize = function () {
  if (this.isTesting) {
    return {
      grid: this.grid.serialize(),
      score: this.score,
      over: this.over,
      won: this.won,
      state: this.state,
      gameString: this.gameString,
      lastMoveTime: this.lastMoveTime,
      keepPlaying: this.keepPlaying,
    };
  } else {
    return {
      grid: this.grid.serialize(),
      score: this.score,
      over: this.over,
      won: this.won,
      keepPlaying: this.keepPlaying,
    };
  }
};

// Save all tile positions and remove merger info
GameManager.prototype.prepareTiles = function () {
  this.grid.eachCell(function (x, y, tile) {
    if (tile) {
      tile.mergedFrom = null;
      tile.savePosition();
    }
  });
};

// Move a tile and its representation
GameManager.prototype.moveTile = function (tile, cell) {
  this.grid.cells[tile.x][tile.y] = null;
  this.grid.cells[cell.x][cell.y] = tile;
  tile.updatePosition(cell);
};

// Move tiles on the grid in the specified direction
GameManager.prototype.move = async function (direction, noPost) {
  // 0: up, 1: right, 2: down, 3: left
  var self = this;

  if (this.isGameTerminated()) return; // Don't do anything if the game's over

  var cell, tile;

  var vector = this.getVector(direction);
  var traversals = this.buildTraversals(vector);
  var moved = false;

  // Save the current tile positions and remove merger information
  this.prepareTiles();

  // Traverse the grid in the right direction and move tiles
  traversals.x.forEach(function (x) {
    traversals.y.forEach(function (y) {
      cell = { x: x, y: y };
      tile = self.grid.cellContent(cell);

      if (tile) {
        var positions = self.findFarthestPosition(cell, vector);
        var next = self.grid.cellContent(positions.next);

        // Only one merger per row traversal?
        if (next && next.value === tile.value && !next.mergedFrom) {
          var merged = new Tile(positions.next, tile.value * 2);
          merged.mergedFrom = [tile, next];

          self.grid.insertTile(merged);
          self.grid.removeTile(tile);

          // Converge the two tiles' positions
          tile.updatePosition(positions.next);

          // Update the score
          self.score += merged.value;

          // The mighty 2048 tile
          if (merged.value === 2048 && !this.isAi) self.won = true;
        } else {
          self.moveTile(tile, positions.farthest);
        }

        if (!self.positionsEqual(cell, tile)) {
          moved = true; // The tile moved from its original cell!
        }
      }
    });
  });

  if (moved) {
    this.addRandomTile();

    if (!this.movesAvailable()) {
      this.over = true; // Game over!
    }

    let moveDirection;
    let aiMoveDirection;
    let moveTime;
    let state;
	const shouldPost = document.getElementById("ai-move") === null;
    if (this.isTesting) {
	  let uid = window.localStorage.getItem("id");
	  if (!uid) {
		  uid = window.localStorage.setItem("id", uuidv4());
	  }
      var dir_map = {
        0: 'up',
        1: 'right',
        2: 'down',
        3: 'left',
      };
      moveDirection = dir_map[direction];
      moveTime = Date.now() - this.lastMoveTime;
      state = this.state;
      aiMoveDirection = this.wasm.get_next_move(
        this.hexStringStateToInt(state)
      );
      aiMoveDirection = dir_map[aiMoveDirection];
      this.state = this.gridCellsToHexString(this.grid.cells);

      this.gameString += moveDirection;
      this.gameString += ',';
	  this.gameString += aiMoveDirection;
	  this.gameString += ',';
      this.gameString += moveTime.toString();
      this.gameString += '\n';
      this.lastMoveTime = Date.now();

      if (this.over && shouldPost) {
        try {
          await axios.post(
            'https://project-3646707934505305305.firebaseio.com/complete_games.json',
            { id: uid, game: this.gameString }
          );
        } catch (err) {
          console.error(err);
        }
      } else {
        this.gameString += state;
        this.gameString += ',';
      }
    }

    this.actuate();

    if (this.isTesting && state && moveTime && moveDirection && shouldPost) {
	  let uid = window.localStorage.getItem("id");
	  if (!uid) {
		uid = window.localStorage.setItem("id", uuidv4());
	  }
      try {
        await axios.post(
          'https://project-3646707934505305305.firebaseio.com/moves.json',
          {
			id: uid,
            state: state,
            user_direction: moveDirection,
            ai_direction: aiMoveDirection,
            time_taken: moveTime,
          }
        );
      } catch (err) {
        console.error(err);
      }
    }
  }
};

GameManager.prototype.gridCellsToInt = function (cells) {
  let int_value = BigInt(0);
  for (let x = 0; x < 4; x++) {
    for (let y = 0; y < 4; y++) {
      let cell_value = cells[x][y];
      if (cell_value == null) {
        cell_value = 0;
      } else {
        cell_value = Math.log2(cell_value.value);
      }

      // top left is at start of num
      const shift_amount = 4 * (4 * (3 - y) + (3 - x));
      int_value = int_value + BigInt(cell_value * Math.pow(2, shift_amount));
    }
  }
  return int_value;
};

GameManager.prototype.gridCellsToHexString = function (cells) {
  let hexString = this.gridCellsToInt(cells).toString(16);
  return hexString.padStart(16, '0');
};

GameManager.prototype.hexStringStateToInt = function () {
  if (!this.isTesting) {
    return;
  }
  return BigInt('0x' + this.state);
};

// Get the vector representing the chosen direction
GameManager.prototype.getVector = function (direction) {
  // Vectors representing tile movement
  var map = {
    0: { x: 0, y: -1 }, // Up
    1: { x: 1, y: 0 }, // Right
    2: { x: 0, y: 1 }, // Down
    3: { x: -1, y: 0 }, // Left
  };

  return map[direction];
};

// Build a list of positions to traverse in the right order
GameManager.prototype.buildTraversals = function (vector) {
  var traversals = { x: [], y: [] };

  for (var pos = 0; pos < this.size; pos++) {
    traversals.x.push(pos);
    traversals.y.push(pos);
  }

  // Always traverse from the farthest cell in the chosen direction
  if (vector.x === 1) traversals.x = traversals.x.reverse();
  if (vector.y === 1) traversals.y = traversals.y.reverse();

  return traversals;
};

GameManager.prototype.findFarthestPosition = function (cell, vector) {
  var previous;

  // Progress towards the vector direction until an obstacle is found
  do {
    previous = cell;
    cell = { x: previous.x + vector.x, y: previous.y + vector.y };
  } while (this.grid.withinBounds(cell) && this.grid.cellAvailable(cell));

  return {
    farthest: previous,
    next: cell, // Used to check if a merge is required
  };
};

GameManager.prototype.movesAvailable = function () {
  return this.grid.cellsAvailable() || this.tileMatchesAvailable();
};

// Check for available matches between tiles (more expensive check)
GameManager.prototype.tileMatchesAvailable = function () {
  var self = this;

  var tile;

  for (var x = 0; x < this.size; x++) {
    for (var y = 0; y < this.size; y++) {
      tile = this.grid.cellContent({ x: x, y: y });

      if (tile) {
        for (var direction = 0; direction < 4; direction++) {
          var vector = self.getVector(direction);
          var cell = { x: x + vector.x, y: y + vector.y };

          var other = self.grid.cellContent(cell);

          if (other && other.value === tile.value) {
            return true; // These two tiles can be merged
          }
        }
      }
    }
  }

  return false;
};

GameManager.prototype.positionsEqual = function (first, second) {
  return first.x === second.x && first.y === second.y;
};
