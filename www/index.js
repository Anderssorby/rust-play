import { Universe, Cell } from "rust-play";
import { memory } from "rust-play/rust_play_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR_1 = "#0000FF";
const ALIVE_COLOR_2 = "#00FF00";
const ALIVE_COLOR_3 = "#00FF00";


const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  const alivePtr = universe.alive_neighbours();
  const aliveNeighbours = new Uint8Array(memory.buffer, alivePtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      const alive = aliveNeighbours[idx];

      if (cells[idx] === Cell.Dead) {
        ctx.fillStyle = DEAD_COLOR;
      } else if (alive < 2) {
        ctx.fillStyle = ALIVE_COLOR_1;
      } else if (alive <= 3) {
        ctx.fillStyle = ALIVE_COLOR_2;
      } else {
        ctx.fillStyle = ALIVE_COLOR_3;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);

//wasm.greet("Anders");
//const Http = new XMLHttpRequest();
//const url='http://localhost:5000';
//Http.open("GET", url);
//Http.send();
//
//Http.onreadystatechange = (e) => {
//  console.log(Http.responseText)
//}
