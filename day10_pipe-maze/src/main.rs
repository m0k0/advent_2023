use std::env;

use input::iterate_input;

mod input;

const PIPE_NORTH_SOUTH: char = '|';
const PIPE_EAST_WEST: char = '-';
const PIPE_NORTH_EAST: char  = 'L';
const PIPE_NORTH_WEST: char = 'J';
const PIPE_SOUTH_WEST: char = '7';
const PIPE_SOUTH_EAST: char = 'F';
const PIPE_GROUND: char = '.';
const PIPE_ANIMAL: char = 'S';

struct Matrix<T> {
    data: Vec<Vec<Option<T>>>    
}

impl<T> Matrix<T> {

    fn get_value(&self, x: usize, y: usize) -> Option<&T> {
        let row = match self.data.get(y) {
            Some(v) => v,
            None => return None
        };

        let cell = match row.get(x) {
            Some(v) => v,
            None => return None
        };

        let value = match *cell {
            Some(v) => v,
            None => return None
        };

        return Some(&value);
    }

    fn set_value(&mut self, x: usize, y: usize, value: T) {

        while self.data.len() <= y {
            self.data.push(Vec::new());
        }

        let mut row = match self.data.get(y) {
            Some(v) => v,
            None => panic!("Out of bounds")
        };
        
        while row.len() <= x {
            row.push(None);
        }
        
        self.data[y][x] = Some(value); 
    }

    fn new() -> Self {
        return Self {
            data: Vec::new()
        }
    }
}


fn main() {
    env::set_var("PRINT_DEBUG", "true"); 

    let mut grid: Matrix<char> = Matrix::new();

    for (row_ix, line) in iterate_input().enumerate() {
        
        for (col_ix, c) in line.chars().enumerate() {
            grid.set_value(col_ix, row_ix, c);
        }
    }

}



