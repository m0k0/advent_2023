use std::env;

use input::iterate_input;

mod input;

fn main() {

    env::set_var("PRINT_DEBUG", "true");
 
    let mut solution = 0;
    let mut solution_part_two = 0;

    for (line_ix, line) in iterate_input().enumerate() {

        let mut line_sequence = Vec::new();
        for segment in line.split(" ") {
            
            let parsed_segment: isize = match segment.parse() {
                Ok(v) => v,
                Err(e) => { panic!("Error parsing input {} on line {}", e, line_ix) }
            };

            line_sequence.push(parsed_segment);
        }

        let next_value = calc_next_value(&line_sequence, false);
        solution += next_value; 

        let next_value_part_two = calc_next_value(&line_sequence, true);
        solution_part_two += next_value_part_two;

        debug!(line);
        debug!("nvp2: {}", next_value_part_two);
    }

    println!("Solution: {}", solution);
    println!("Solution: {}", solution_part_two);

}

fn calc_next_value(sequence: &Vec<isize>, left_side: bool) -> isize {
    
    let reduced_sequence = calc_reduced_sequence(sequence);
    
    let mut is_all_zero = true;
    for num in reduced_sequence.iter() {
        
        if *num != 0 {
            is_all_zero = false;
            break;
        }
    }


    let sequence_value;
    let reduced_sequence_value;
    if left_side {
        sequence_value = sequence.first();
        reduced_sequence_value = reduced_sequence.first();
    } else {
        sequence_value = sequence.last();
        reduced_sequence_value = reduced_sequence.last();
    }


    let sequence_value = match sequence_value {
        Some(v) => *v,
        None => 0
    };

    let mut reduced_sequence_value = match reduced_sequence_value {
        Some(v) => *v,
        None => 0
    };

    if !is_all_zero {
        reduced_sequence_value = calc_next_value(&reduced_sequence, left_side);
    }

    let next_value;
    if left_side {
        next_value = sequence_value - reduced_sequence_value;
    } else {
        next_value = sequence_value + reduced_sequence_value;
    }

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
