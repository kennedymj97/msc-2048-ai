import { WasmInterface } from "msc-2048-ai";

const canvas = document.getElementById("game-canvas");
const ctx = canvas.getContext("2d");

let animationId;
let numMoves = 0;
let startTime;

function main() {
    const canvasSize = 500;
    canvas.width = canvasSize;
	canvas.height = canvasSize;

    ctx.font = "bold 30px Arial";
	ctx.textAlign = "center";
	ctx.textBaseline = "middle";

    let game = WasmInterface.new();
	startTime = new Date();
	render(game);
	animationId = requestAnimationFrame(() => animate(game));
}

const animate = (game) => {
	if (game.is_game_over()) {
		let finishTime = new Date();
		let millisecondsElapsed = finishTime - startTime;
		console.log("Time taken: " + (millisecondsElapsed / 1000) + " seconds");
		console.log("Number of moves made: " + numMoves);
		alert("Game over!");
		cancelAnimationFrame(animationId);
		return;
	}

	ctx.clearRect(0, 0, canvas.width, canvas.height);
	game.make_move();
	render(game);
	numMoves++;

    animationId = requestAnimationFrame(() => animate(game));
}

const render = (game) => {
	drawGrid();
	drawTiles(game);
}

const drawGrid = () => {
	const offset = canvas.width / 4;
	for (let i = 0; i <= 4; i++) {
		ctx.beginPath();
		ctx.moveTo(offset * i, 0);
		ctx.lineTo(offset * i, canvas.height);
		ctx.stroke();
		
		ctx.beginPath();
		ctx.moveTo(0, offset * i);
		ctx.lineTo(canvas.width, offset * i);
		ctx.stroke();
	}
}

const drawTiles = (game) => {
	const offset = canvas.width / 4;
	const edgeOffset = canvas.width / 8;
	for (let i = 0; i < 4; i++) {
		for (let j = 0; j < 4; j++) {
			let tileVal = game.get_tile_val(i, j);
			if (tileVal !== 1) {
				ctx.fillText(tileVal.toString(), edgeOffset + (offset * i), edgeOffset + (offset * j));
			}
		}
	}
}

window.onload = main();
