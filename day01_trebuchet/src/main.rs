use std::{fs::File, env::{Args, self}, io::{BufReader, BufRead, Error}, collections::HashMap, array, ops::Index};

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
    let answer = get_sum(input_file);

    println!("The answer is {}", answer);
}

fn get_sum(input_file: File) -> usize {
    
    let digit_map = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let input_file = BufReader::new(input_file);

    let mut sum : usize = 0; 
    for line in input_file.lines() {
        sum = sum + get_line_value_part2(line.unwrap(), &digit_map);
    }

    return sum;

}



fn map_line_words(line: String, map: &[&str]) -> String {
 
    debug!("source line: {}", line);

    let mut result = String::from(line);

    for (ix, value) in map.iter().enumerate() {
        //debug!(value);        
        result = result.replace(value, ix.to_string().as_str());
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

fn map_digit_word(s : String, map: &[&str]) -> Option<usize> {
     
    for (ix, word) in map.iter().enumerate() {
        if s.contains(*word) {
            return Some(ix);
        }
    }
    return None;

}

fn get_line_value_part2(line : String, map: &[&str]) -> usize {
   
    let mut first_digit_char = '\0';
    let mut last_digit_char : char = '\0';

    let mut digit_word_buffer : String = String::new();

    debug!(line);

    let line_chars: Vec<char> = line.chars().collect();
    let mut line_ix = 0;

    while line_ix < line_chars.len() {
        
        let mut digit_char = line_chars[line_ix];

        digit_word_buffer.push(digit_char);

        //debug!("buffer: {}", digit_word_buffer);

        if let Some(digit) = map_digit_word(digit_word_buffer.to_string(), map) {
            //debug!("found word: {}", digit_word_buffer);
            digit_char = digit.to_string().chars().nth(0).unwrap();
            //debug!("converted to: {}", digit_char);
            digit_word_buffer = String::new();
            line_ix = line_ix - 1;
        }        
        if digit_char.is_numeric() {
             
            if first_digit_char == '\0' {
                first_digit_char = digit_char;
            } else {
                last_digit_char = digit_char;
            }
            digit_word_buffer = String::new();
        }

        //debug!("char: {}", digit_char);

        line_ix = line_ix + 1;

        
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


