use std::{env, fmt::format};

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


struct Coord2D {
    x: usize,
    y: usize
}
impl std::fmt::Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!("x: {}, y: {}", self.x, self.y);
    }
}


struct Matrix<T> {
    data: Vec<Vec<Option<T>>>,
    width: usize,
    height: usize
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


        let value = match cell {
            Some(v) => v,
            None => return None
        };

        return Some(&value);
    }

    fn set_value(&mut self, x: usize, y: usize, value: T) {

        while self.data.len() <= y {
            self.data.push(Vec::new());
        }

        let row = match self.data.get_mut(y) {
            Some(v) => v,
            None => panic!("Out of bounds")
        };
         
        while row.len() <= x {
            row.push(None);
        }

        if self.width < row.len() {
            self.width = row.len();
        }
        if self.height < self.data.len() {
            self.height = self.data.len();
        }
        
        self.data[y][x] = Some(value); 
    }

    fn new() -> Self {
        return Self {
            data: Vec::new(),
            width: 0,
            height: 0
        }
    }
}


fn main() {
    env::set_var("PRINT_DEBUG", "true"); 

    let mut grid: Matrix<char> = Matrix::new();
    let mut animal_coord: Coord2D;

    for (row_ix, line) in iterate_input().enumerate() {
        for (col_ix, c) in line.chars().enumerate() {
            grid.set_value(col_ix, row_ix, c);
            if c == PIPE_ANIMAL {
                animal_coord = Coord2D { x: col_ix, y: row_ix };
            }
        }
    }
    print_grid(&grid);
    
    debug!("The animal is at {}", animal_coord);
}

fn print_grid(grid: &Matrix<char>) {

    for y in 0..grid.height {
        for x in 0..grid.width {

            if let Some(v) = grid.get_value(x, y) {
                print!("{}",v);
            } else {
                print!(" ");
            }
        }

        print!("\n");
    }
}
