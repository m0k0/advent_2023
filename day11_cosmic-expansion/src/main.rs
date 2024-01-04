use std::env;

use input::input_as_grid;

mod input;
mod grid;

fn main() {
    
    env::set_var("PRINT_DEBUG", "TRUE");

    let grid = input_as_grid();

    grid.print_grid()
}

