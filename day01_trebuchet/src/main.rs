use std::{fs::File, env::{Args, self}, io::{BufReader, BufRead}};

const ARGS_IX_INPUT : usize = 1;

fn main()  {
    let input_file_path = env::args().nth(ARGS_IX_INPUT);

    if input_file_path.is_none() { 
        panic!("No input file specified");
    }
    let input_file_path = input_file_path.unwrap();


    let input_file = File::open(&input_file_path);
    if input_file.is_err() {
        panic!("Error opening file: {}", &input_file_path)
    }

    let input_file = input_file.unwrap();
    let input_file = BufReader::new(input_file);

    
    for line in input_file.lines() {

        println!("{}",line.unwrap());
    }
}
