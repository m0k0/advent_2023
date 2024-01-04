use std::env;

use grid::Matrix;
use input::input_as_grid;

mod input;
mod grid;

const CHAR__EMPTY_SPACE: char = '.';
const CHAR__GALAXY: char = '#';


fn main() {
    
    env::set_var("PRINT_DEBUG", "TRUE");

    let mut grid = input_as_grid();

    // input grid
    grid.print_grid();

}

fn expand_space(space_grid: Matrix<char>) -> Matrix<char>{ 

    let mut expanded_space = Matrix::new();

    for y in 0..space_grid.height()  {
        
        let mut is_empty_space = true;
        
        let row  = space_grid.get_row_values(y);
 
        for cell in row {
        
            if !cell.is_some_and(|x| *x == CHAR__EMPTY_SPACE) {
                is_empty_space = false;
                break;
            }
        }

        expanded_space.insert_row(y, row);
        
    }

    return expand_space(expanded_space);

}
