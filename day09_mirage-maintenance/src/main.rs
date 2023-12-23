use std::env;

use input::iterate_input;

mod input;

fn main() {

    env::set_var("PRINT_DEBUG", "true");

    for (line_ix, line) in iterate_input().enumerate() {

        let mut line_sequence = Vec::new();
        for segment in line.split(" ") {
            
            let parsed_segment: isize = match segment.parse() {
                Ok(v) => v,
                Err(e) => { panic!("Error parsing input {} on line {}", e, line_ix) }
            };

            line_sequence.push(parsed_segment);
        }
    }
}


fn calc_reduced(sequence: Vec<isize>) {
    
    let mut reduced_sequence = calc_reduced_sequence(sequence);
    
    let mut isAllZeroes = true;
    for num in reduced_sequence.iter() {
        
        if *num != 0 {
            isAllZeroes = false;
            break;
        }
    }

    if isAllZeroes {
        // done
    }


}

fn calc_reduced_sequence(sequence: Vec<isize>) -> Vec<isize> {

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
