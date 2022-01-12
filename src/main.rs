use std::ops::Range;
use std::{thread, time};

const GRID_SIZE: usize = 6;

struct Grid {
    bit_array: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            bit_array: [[false; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, is_alive: bool) {
        self.bit_array[row][col] = is_alive;
    }

    fn get_bounds(&self, val: usize) -> Range<usize> {
        let start = if val > 0 { val - 1 } else { val };
        let end = if val + 1 < GRID_SIZE {
            val + 2
        } else {
            val + 1
        };
        Range { start, end }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut live_neighbors = 0;

        // Make sure we stay in the bounds of the grid
        let row_bounds = self.get_bounds(row);
        let col_bounds = self.get_bounds(col);

        // Iterate through all 8 adjacent cells
        for i in row_bounds.start..row_bounds.end {
            for j in col_bounds.start..col_bounds.end {
                // Make sure to skip the cell at (row, col)
                if i == row && j == col {
                    continue;
                }

                // If there's a cell at (i, j), increment live_neighbors
                if self.bit_array[i][j] {
                    live_neighbors += 1;
                }
            }
        }

        live_neighbors
    }

    fn update(&mut self) {
        // Update the grid based on rules of Conway's Game of Life
        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Any live cell with two or three live neighbours lives on to the next generation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

        let mut updated_bit_array = [[false; GRID_SIZE]; GRID_SIZE];

        for (i, row) in updated_bit_array.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = self.bit_array[i][j];
                let live_neighbors = self.count_neighbors(i, j);
                // If the cell is alive
                if self.bit_array[i][j] {
                    // Set the cell to dead if it doesn't have fewer than 2 or more than 3 live
                    // neighbors
                    if !(2..=3).contains(&live_neighbors) {
                        *col = false;
                    }
                }
                // If the cell is dead
                else {
                    // Set the cell to alive if it has exactly 3 live neighbors
                    if live_neighbors == 3 {
                        *col = true;
                    }
                }
            }
        }

        self.bit_array = updated_bit_array;
    }

    fn draw(&self) {
        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        println!();
        for row in self.bit_array.iter() {
            for col in row.iter() {
                if *col {
                    print!(" X ");
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
    }
}

fn main() {
    // Init
    let mut grid = Grid::new();

    // Seed (Toad)
    grid.set_cell(2, 2, true);
    grid.set_cell(2, 3, true);
    grid.set_cell(2, 4, true);

    grid.set_cell(3, 1, true);
    grid.set_cell(3, 2, true);
    grid.set_cell(3, 3, true);

    // Seed (Beacon)
    // grid.set_cell(1, 1, true);
    // grid.set_cell(1, 2, true);
    // grid.set_cell(2, 1, true);

    // grid.set_cell(3, 4, true);
    // grid.set_cell(4, 3, true);
    // grid.set_cell(4, 4, true);

    // Game Loop
    loop {
        grid.update();
        grid.draw();

        thread::sleep(time::Duration::from_millis(500));
    }
}
