use std::{fs::File, env::{self}, io::{BufReader,BufRead, Lines}};

use crate::grid::Matrix;

const ARGS_IX_INPUT : usize = 1;

#[macro_export]
macro_rules! debug {
    ($a:expr) => {
        if env::var("PRINT_DEBUG").is_ok() {
            println!("{}",$a);
        }
    };
    
    ($a:expr, $b:expr) => {
        if env::var("PRINT_DEBUG").is_ok() {
           println!($a, $b);
        }

    };
}




pub fn iterate_input() -> impl Iterator<Item = String>  {

    let input_file_path = get_input_path_from_args();
    
    let iterator = read_input_lines(input_file_path)
        .filter_map(|x| match x {
            Ok(v) => Some(v),
            Err(_) => None
        });

    return iterator;

}


pub fn get_input_path_from_args() -> String {

    if let Some(input_file_path) = env::args().nth(ARGS_IX_INPUT) { 
        return input_file_path;
    }
    
    panic!("No input file specified");
}

pub fn read_input_lines(input_file_path: String) -> Lines<BufReader<File>> {
    let input_file = File::open(&input_file_path);
    
    if input_file.is_err() {
        panic!("Error opening file: {}", &input_file_path);
    }

    let input_file = input_file.unwrap();
    let input_file = BufReader::new(input_file);

    return input_file.lines();

}


pub fn input_as_grid() -> Matrix<char> { 
    let mut grid = Matrix::new();
    for (y, line) in iterate_input().enumerate() {

        for (x, c) in line.chars().enumerate() {
            
            grid.set_value(x, y, c);
        }
    }
    return grid;

}


