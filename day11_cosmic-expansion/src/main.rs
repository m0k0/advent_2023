use std::env;

use input::iterate_input;

use crate::grid::Matrix;

mod input;
mod grid;

fn main() {
    
    env::set_var("PRINT_DEBUG", "TRUE");

    let grid = parse_grid();

    grid

}

fn parse_grid() -> Matrix<char> {
    
    let mut grid = Matrix::new();

    for (y, line) in iterate_input().enumerate() {

        for (x, c) in line.chars().enumerate() {
            
            grid.set_value(x, y, c);
        }
    }
    return grid;

}

