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
        
        let line = line.unwrap();

        debug!(line);

        if let Some(game_result) = process_game_part2(line) {
            debug!("result: {}", game_result);
            answer = answer + game_result;
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
fn process_game_part1(line: String, limits: &HashMap<&str, usize>) -> Option<usize> {


    let game_data: Vec<&str> = line.split(":").collect();

    let game_id_str = game_data[0].replace("Game ", "");

    let game_id: Option<usize> = match game_id_str.parse() {
       Ok(id) => Some(id),
       Err(_) => None
    };

    
    let set_data = game_data[1].split(";");
    
    for set in set_data { 
        if !is_set_possible(set, limits) {
            return None;
        }
    }

    return game_id;

}

// takes a game string and 
// returns its "power" 
fn process_game_part2(line: String) -> Option<usize> {

    let game_data: Vec<&str> = line.split(":").collect();

    let game_id_str = game_data[0].replace("Game ", "");

    let game_id: Option<usize> = match game_id_str.parse() {
       Ok(id) => Some(id),
       Err(_) => None
    };

    
    let set_data = game_data[1].split(";");
    
    let mut minimum_store: HashMap<&str, usize> = HashMap::new();

    for set in set_data { 
        apply_minimum_cubes(set, &mut minimum_store);
    }

    let mut power: usize = 1;
    for (colour, minimum) in minimum_store {
        
        debug!(format!("min {colour}: {minimum}"));

        power = power * minimum;

    }

    return Some(power);

}

fn apply_minimum_cubes<'a>(set_spec: &'a str, minimum_store: &mut HashMap<&'a str,usize>){

    
    let set_data = set_spec.split(",");

    debug!("\t {}",set_spec);

    for set_part in set_data {
    
        //debug!("set part: {}", set_part);
        let set_part_components: Vec<&str> = set_part.trim().split(" ").collect();

        let amount: usize = match set_part_components[0].parse() {
            Ok(value) => value,
            Err(_) => {return} // invalid amount
        };

        //debug!("set amount: {}", amount);

        let colour = set_part_components[1];
       
        
        minimum_store.entry(colour)
            .and_modify(|e| { 
                if amount > *e {
                    *e = amount;
                }
            })
            .or_insert(amount);
        
    }

    return;

}



fn is_set_possible(set_spec: &str, limits: &HashMap<&str, usize>) -> bool {
    
    let set_data = set_spec.split(",");
    debug!("\t {}",set_spec);

    for set_part in set_data {
    
        //debug!("set part: {}", set_part);
        let set_part_components: Vec<&str> = set_part.trim().split(" ").collect();

        let amount: usize = match set_part_components[0].parse() {
            Ok(value) => value,
            Err(_) => {return false } // invalid amount
        };

        //debug!("set amount: {}", amount);

        let colour = set_part_components[1];

        
        let colour_limit = match limits.get(colour) {
            Some(value) => value,
            None => {return false } // colour limit not defined
        };

        if amount > *colour_limit {
        
            debug!("\t- limit for {} exceeded", colour);
            return false;
        }
    }

    return true;
}

