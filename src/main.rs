use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};
use std::{thread, time};

const NUM_ROWS: usize = 6;
const NUM_COLS: usize = 6;

struct Bounds {
    lower: usize,
    upper: usize,
}

struct Grid {
    bit_array: [bool; NUM_ROWS * NUM_COLS],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            bit_array: [false; NUM_ROWS * NUM_COLS],
        }
    }

    fn get_cell(&self, row: usize, col: usize) -> bool {
        self.bit_array[row * NUM_COLS + col]
    }

    fn set_cell(&mut self, row: usize, col: usize, is_alive: bool) {
        self.bit_array[row * NUM_COLS + col] = is_alive;
    }

    fn get_bounds(&self, val: usize, limit: usize) -> Bounds {
        let lower = if val > 0 { val - 1 } else { val };
        let upper = if val + 1 < limit { val + 2 } else { val + 1 };
        Bounds { lower, upper }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut live_neighbors = 0;

        // Make sure we stay in the bounds of the grid
        let row_bounds = self.get_bounds(row, NUM_ROWS);
        let col_bounds = self.get_bounds(col, NUM_COLS);

        // Iterate through all 8 adjacent cells
        for i in row_bounds.lower..row_bounds.upper {
            for j in col_bounds.lower..col_bounds.upper {
                // Make sure to skip the cell at (row, col)
                if i == row && j == col {
                    continue;
                }

                // If there's a cell at (i, j), increment live_neighbors
                if self.get_cell(i, j) {
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

        let mut updated_bit_array = [false; NUM_ROWS * NUM_COLS];

        for i in 0..NUM_ROWS {
            for j in 0..NUM_COLS {
                updated_bit_array[i * NUM_COLS + j] = self.bit_array[i * NUM_COLS + j];
                let live_neighbors = self.count_neighbors(i, j);
                // If the cell is alive
                if self.get_cell(i, j) {
                    // Set the cell to dead if it doesn't have fewer than 2 or more than 3 live
                    // neighbors
                    if !(2..=3).contains(&live_neighbors) {
                        updated_bit_array[i * NUM_COLS + j] = false;
                    }
                }
                // If the cell is dead
                else {
                    // Set the cell to alive if it has exactly 3 live neighbors
                    if live_neighbors == 3 {
                        updated_bit_array[i * NUM_COLS + j] = true;
                    }
                }
            }
        }

        self.bit_array = updated_bit_array;
    }
}

fn draw_grid(grid: &Grid) {
    println!();
    for i in 0..NUM_ROWS {
        for j in 0..NUM_COLS {
            let is_alive = grid.get_cell(i, j);
            if is_alive {
                print!(" X ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }
}

fn main() {
    let mut stdout = stdout();

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

        stdout.queue(cursor::SavePosition).unwrap();
        draw_grid(&grid);
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();

        thread::sleep(time::Duration::from_millis(500));
    }
}
