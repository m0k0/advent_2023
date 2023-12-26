use std::{env, collections::HashMap, fmt::Display};

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
    x: isize,
    y: isize
}
impl Coord2D {
    fn new(x: isize, y: isize) -> Self {
        return Self { x, y }
    }

    fn combine(&self, other: &Coord2D) -> Coord2D {
        return Coord2D::new(self.x + other.x, self.y + other.y);
    }
}
impl Clone for Coord2D {
    
    fn clone(&self) -> Self {
        return Coord2D::new(self.x, self.y);
    }

}
impl std::fmt::Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
impl std::ops::Add<Coord2D> for Coord2D{
    type Output = Self;

    fn add(self, rhs: Coord2D) -> Self::Output {
        return Coord2D::new(self.x + rhs.x, self.y + rhs.y);
    } 
}
impl PartialEq for Coord2D {

    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
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
    let mut score_grid: Matrix<usize> = Matrix::new();

    let mut animal_coord: Option<Coord2D> = None;

    for (row_ix, line) in iterate_input().enumerate() {
        for (col_ix, c) in line.chars().enumerate() {
            grid.set_value(col_ix, row_ix, c);
            if c == PIPE_ANIMAL {
                animal_coord = Some (
                    Coord2D { x: col_ix as isize, y: row_ix as isize }
                );
            }
        }
    }
    print_grid(&grid);
 

    let animal_coord = match animal_coord {
        Some(v) => v,
        None => panic!("No animal found")
    };

    debug!("The animal is at {}", animal_coord);

    walk_path(&grid, &mut score_grid, animal_coord, 0);

    print_grid(&score_grid);

    let solution = get_highest_steps(&score_grid);

    println!("Solution: {}", solution);
}

fn get_available_moves(c: char) -> Vec<Coord2D> {
    
    let mut moves = Vec::new();

    // west
    if c == PIPE_ANIMAL || c == PIPE_EAST_WEST || c == PIPE_NORTH_WEST || c == PIPE_SOUTH_WEST {
        moves.push(Coord2D::new(-1, 0));
    }
    // south
    if c == PIPE_ANIMAL || c == PIPE_NORTH_SOUTH || c == PIPE_SOUTH_EAST || c == PIPE_SOUTH_WEST {
        moves.push(Coord2D::new(0, 1));
    }
    // east
    if c == PIPE_ANIMAL || c == PIPE_EAST_WEST || c == PIPE_NORTH_EAST || c == PIPE_SOUTH_EAST{
        moves.push(Coord2D::new(1, 0));
    }
    // north
    if c == PIPE_ANIMAL || c == PIPE_NORTH_EAST || c == PIPE_NORTH_WEST || c == PIPE_NORTH_SOUTH {
        moves.push(Coord2D::new(0, -1));  
    }

    return moves;
}

fn walk_path(
    grid: &Matrix<char>, 
    score_grid: &mut Matrix<usize>,
    position: Coord2D, 
    steps: usize) {
    

    debug!("Walking at {}", position);

    let c = match grid.get_value(position.x as usize, position.y as usize) {
        Some(v) => *v,
        None => return
    };
    
    if c == PIPE_GROUND {
        debug!("invalid; impassable");
        return;
    }

    let moves = get_available_moves(c);

    // record position
    score_grid.set_value(position.x as usize, position.y as usize, steps); 

    for m in moves {

        debug!(format!("move: {}; from: {}", m, position));

        let new_position = position.combine(&m);

        if new_position.x < 0 || new_position.y < 0 || new_position.x >= grid.width as isize || new_position.y >= grid.height as isize {
            
            debug!("invalid; out of bounds");
            continue; // invalid move; out of bounds
        }

        let new_steps = steps + 1;
       
        if let Some(recorded_steps) = score_grid.get_value(new_position.x as usize, new_position.y as usize) {
            if *recorded_steps < new_steps {
                debug!("invalid; already recorded with fewer steps");
                continue; // new position already scored
            }
        }

        debug!("steps: {}", new_steps);

        walk_path(grid, score_grid, new_position, new_steps);
    }

}

fn get_highest_steps(score_grid: &Matrix<usize>) -> usize{
    
    let mut highest_steps = 0;
    for row in score_grid.data.iter() {
        for cell in row.iter() {
            if let Some(cell_steps) = cell {
                if *cell_steps > highest_steps {
                    highest_steps = *cell_steps;
                }
            }
        }
    }
    
    return highest_steps;
}

fn print_grid<T>(grid: &Matrix<T>) where T: Display {

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
