mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// wasm_bindgen used to interface with JS
// importing JS functions to rust
// indexing into the global JS namespace/object table to find the alert function and bring it into rust/scope
#[wasm_bindgen]
extern {
    // fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// // exporting rust functions to JS
// // placing the greet function in the heap (boxing) 
// // and creating a JS class wrapper around the pointer to the function to use within JS
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

#[wasm_bindgen]
#[repr(u8)] // each cell is represented by a single byte when compiled to wasm, for memory efficiency
#[derive(Clone, Copy, Debug, PartialEq, Eq)] // derive some traits
pub enum Cell {
    Dead = 0, // optimization 
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>, // of length width * height (area)
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 32;
        let height = 32;

        let mut rng = rand::thread_rng();
        
        // fill up universe with cells, both alive and dead
        let cells = (0..width * height)
            .map(|i| {
                if i % 13 == 0 {
                    Cell::Alive
                } else {
                    let rand_num = rng.gen_range(0..2);
                    if rand_num == 1 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
            })
            .collect();

        Universe {
           width,
           height,
           cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // return a read-only pointer to a Cell type
    // raw pointer - unsafe access to memory location (unsafe Rust, bypass borrow checker)
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        // provided by implementing the Display trait
        self.to_string()
    }

    // translate a 2d coordinate into a 1d index
    fn get_index(&self, row: u32, column: u32) -> usize {
        // if row == 1 && column == 1 {
        //     log(&format!("index: {}", row * self.width + column));
        // }
        // row * width + column
        (row * self.width + column) as usize
    }

    // computes total live neighbors for a given cell
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // count the number of live neighbors
        let mut count = 0;
        // iterate over all possible neighbors
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // don't count the cell itself
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                // compute the neighbor's row and column
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                // get the index of the neighbor
                let idx = self.get_index(neighbor_row, neighbor_col);
                // increment the count if the neighbor is alive
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    // update the universe state with new cells (new generation)
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        // iterate over all cells
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col); // get the index of the cell
                let cell = self.cells[idx]; // get the cell
                let live_neighbors = self.live_neighbor_count(row, col); // get the number of live neighbors
                
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

        self.cells = next;
    }
}

use std::fmt;

// implement Display trait to convert to String for printing
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get the slice of the vector that contains the data, don't want entire growable vector
        // divide the vector of cells into lines of length the width of the universe (creating row)
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}