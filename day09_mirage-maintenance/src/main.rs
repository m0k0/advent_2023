use std::env;

use input::iterate_input;

mod input;

fn main() {

    env::set_var("PRINT_DEBUG", "true");
 
    let mut solution = 0;

    for (line_ix, line) in iterate_input().enumerate() {

        let mut line_sequence = Vec::new();
        for segment in line.split(" ") {
            
            let parsed_segment: isize = match segment.parse() {
                Ok(v) => v,
                Err(e) => { panic!("Error parsing input {} on line {}", e, line_ix) }
            };

            line_sequence.push(parsed_segment);
        }

        let next_value = calc_next_value(&line_sequence);
        solution += next_value; 
        debug!(line);
        debug!("next: {}", next_value);
    }

    println!("Solution: {}", solution);
}


fn calc_next_value(sequence: &Vec<isize>) -> isize {
    
    let reduced_sequence = calc_reduced_sequence(sequence);
    
    let mut is_all_zero = true;
    for num in reduced_sequence.iter() {
        
        if *num != 0 {
            is_all_zero = false;
            break;
        }
    }

    let last_value = match sequence.last() {
        Some(v) => *v,
        None => 0
    };

    let mut last_value_reduced = match reduced_sequence.last() {
        Some(v) => *v,
        None => 0
    };

    if !is_all_zero {
        last_value_reduced = calc_next_value(&reduced_sequence);
    }

    let next_value = last_value + last_value_reduced;

    return next_value;

}

fn calc_reduced_sequence(sequence: &Vec<isize>) -> Vec<isize> {

    let mut reduced_sequence = Vec::new();

    let mut sequence_iterator = sequence.iter();

    let mut next_num = match sequence_iterator.next() {
        Some(v) => v,
        None => { return reduced_sequence }
    };
    
    loop {
        
        let current_num = next_num;
        next_num = match sequence_iterator.next() {
            Some(v) => v,
            None => {return reduced_sequence }
        };

        let diff: isize = next_num - current_num;
        reduced_sequence.push(diff);
    }
    
}
