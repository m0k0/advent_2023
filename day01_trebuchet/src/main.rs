use std::{fs::File, env::{Args, self}, io::{BufReader, BufRead, Error}, collections::HashMap};

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


    let input_file = File::open(&input_file_path);
    if input_file.is_err() {
        panic!("Error opening file: {}", &input_file_path)
    }

    let input_file = input_file.unwrap();
    let answer = get_sum_part2(input_file);

    println!("The answer is {}", answer);
}

fn get_sum_part1(input_file: File) -> usize {
    
    let input_file = BufReader::new(input_file);

    let mut sum : usize = 0; 
    for line in input_file.lines() {
        
        sum = sum + get_line_value(line.unwrap());
    }
    return sum;
}

fn get_sum_part2(input_file: File) -> usize {
    
    let mut digit_map: HashMap<&str, &str> = HashMap::new();
    digit_map.insert("one","1");
    digit_map.insert("two","2");
    digit_map.insert("three","3");
    digit_map.insert("four","4");
    digit_map.insert("five","5");
    digit_map.insert("six","6");
    digit_map.insert("seven","7");
    digit_map.insert("eight","8");
    digit_map.insert("nine","9");
    digit_map.insert("zero","0");
   
    let input_file = BufReader::new(input_file);

    let mut sum : usize = 0; 
    for line in input_file.lines() {
        
        let line = map_line_words(line.unwrap(), &digit_map);

        sum = sum + get_line_value(line);
    }

    return sum;

}

fn map_line_words(line: String, map: &HashMap<&str,&str>) -> String {
 
    debug!("source line: {}", line);

    let mut result = String::from(line);

    for key in map.keys() {
        debug!(key);        
        result = result.replace(key, map[key]);
    }
    debug!("mapped line: {}", result);
    
    return result;
}

fn get_line_value(line : String) -> usize {
   
    let mut first_digit_char : char = '\0';
    let mut last_digit_char : char = '\0';

    debug!(line);


    for c in line.chars() {

        if !c.is_numeric() {
            continue;
        }
        
        if first_digit_char == '\0' {
           first_digit_char = c;
        } else {
            last_digit_char = c;
        }
        
    }
    if first_digit_char == '\0' {
        return 0;
    } 
    let mut digits_text = String::from("");
    digits_text.push(first_digit_char);
    if last_digit_char != '\0' {
        digits_text.push(last_digit_char);
    } else {
        digits_text.push(first_digit_char);
    }
    debug!(digits_text);

    let result = match digits_text.parse::<usize>() {
        Ok(value) => value,
        Err(_) => 0
    };

    debug!(result);
    
    return result;
    
}


