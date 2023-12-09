use std::{fs::File, env::{Args, self}, io::{BufReader, BufRead, Error}};

const ARGS_IX_INPUT : usize = 1;

macro_rules! debug {
    ($a:expr) => {
        println!("{}",$a)
    };
}

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
    let answer = get_sum_part1(input_file);

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


