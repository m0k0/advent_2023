use std::{fs::File, env::{self}, io::{BufReader, BufRead, Lines}, collections::HashMap};

const ARGS_IX_INPUT : usize = 1;
const DEBUG : bool = true;

macro_rules! debug {
    ($a:expr) => {
        if DEBUG {
            println!("{}",$a);
        }
    };
    
    ($a:expr, $b:expr) => {
        if DEBUG {
           println!($a, $b);
        }

    };
}



fn main(){
    let input_file_path = env::args().nth(ARGS_IX_INPUT);

    if input_file_path.is_none() { 
        panic!("No input file specified");
    }
    let input_file_path = input_file_path.unwrap();

    let mut answer = 0;

    let mut limits: HashMap<&str,usize> = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    for line in read_input_lines(input_file_path) {
        
        if let Some(game_id) = process_game(line.unwrap(), &limits) {
            answer = answer + game_id;
        }
    }
    
    println!("The answer is {}", answer);
}

fn read_input_lines(input_file_path: String) -> Lines<BufReader<File>> {
    let input_file = File::open(&input_file_path);
    
    if input_file.is_err() {
        panic!("Error opening file: {}", &input_file_path);
    }

    let input_file = input_file.unwrap();
    let input_file = BufReader::new(input_file);

    return input_file.lines();

}

// takes a game string and 
// tries to return the game number
fn process_game(line: String, limits: &HashMap<&str, usize>) -> Option<usize> {


    let game_data: Vec<&str> = line.split(":").collect();

    let game_id_str = game_data[0].replace("Game ", "");

    let game_id: Option<usize> = match game_id_str.parse() {
       Ok(id) => Some(id),
       Err(_) => None
    };








    return game_id;

}



