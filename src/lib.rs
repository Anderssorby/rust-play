mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let s = format!("Hello, {}!", name);
    alert(&s);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Dims {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
pub struct Universe {
    dims: Dims,
    cells: Vec<Cell>,
    alive_neighbours: Vec<u8>,
}

fn get_index(width: u32, row: u32, column: u32) -> usize {
    (row * width + column) as usize
}

fn live_neighbor_count(dims: &Dims, cells: &Vec<Cell>, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [dims.height - 1, 0, 1].iter().cloned() {
        for delta_col in [dims.width - 1, 0, 1].iter().cloned() {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbor_row = (row + delta_row) % dims.height;
            let neighbor_col = (column + delta_col) % dims.width;
            let idx = get_index(dims.width, neighbor_row, neighbor_col);
            count += cells[idx] as u8;
        }
    }

    count
}

fn live_neighbor_count_all(dims: &Dims, cells: &Vec<Cell>) -> Vec<u8> {
    let cap: usize = (dims.width * dims.height) as usize;
    let mut alive_neighbours = Vec::with_capacity(cap);
    for row in 0..dims.height {
        for col in 0..dims.width {
            let idx = get_index(dims.width, row, col);
            let live_neighbors = live_neighbor_count(&dims, &cells, row, col);
            alive_neighbours[idx] = live_neighbors;
        }
    }
    alive_neighbours
}

fn next_universe(uni: &Universe) -> Vec<Cell> {
    let mut next = uni.cells.clone();
    let dims = &uni.dims;

    for row in 0..dims.height {
        for col in 0..dims.width {
            let idx = get_index(dims.width, row, col);
            let cell = uni.cells[idx];
            let live_neighbors = uni.alive_neighbours[idx];

            let next_cell = match (cell, live_neighbors) {
                // Rule 1: Any live cell with fewer than two live neighbours
                // dies, as if caused by underpopulation.
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                // Rule 2: Any live cell with two or three live neighbours
                // lives on to the next generation.
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                // Rule 3: Any live cell with more than three live
                // neighbours dies, as if by overpopulation.
                (Cell::Alive, x) if x > 3 => Cell::Dead,
                // Rule 4: Any dead cell with exactly three live neighbours
                // becomes a live cell, as if by reproduction.
                (Cell::Dead, 3) => Cell::Alive,
                // All other cells remain in the same state.
                (otherwise, _) => otherwise,
            };

            next[idx] = next_cell;
        }
    }
    next
}

// Game og life
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let next = next_universe(&self);
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let dims = Dims { width, height };

        let cells: Vec<Cell> = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let alive_neighbours = live_neighbor_count_all(&dims, &cells);

        Universe {
            dims,
            cells,
            alive_neighbours,
        }
    }

    pub fn width(&self) -> u32 {
        self.dims.width
    }

    pub fn height(&self) -> u32 {
        self.dims.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn alive_neighbours(&self) -> *const u8 {
        self.alive_neighbours.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.dims.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
