export default function AIController(gameManager, wasm) {
  this.gameManager = gameManager;
  this.wasm = wasm;

  return this;
}

AIController.prototype.start = function () {
  let self = this;
  this.moving = setInterval(function () {
    const move = self.wasm.get_next_move(
      self.gameManager.gridCellsToInt(self.gameManager.grid.cells)
    );
    self.make_move(move);
  }, 1);
};

AIController.prototype.pause = function () {
	clearInterval(this.moving);
}

AIController.prototype.make_move = function (move) {
  if (this.gameManager) {
	this.gameManager.move(move);
  }
};
