use std::env;

use input::iterate_input;

mod input;

fn main() {

    env::set_var("PRINT_DEBUG", "true");


    let solution_part1 = get_solution_part1();
    let solution_part2 = get_solution_part2();

    println!("------------------------------------");
    println!("Solution - Part 1: {}", solution_part1);
    println!("Solution - Part 2: {}", solution_part2);

}

fn get_solution_part2() -> usize {
    return 0;
}

fn get_solution_part1() -> usize {
    let mut solution = 0;

    for line in iterate_input(){
        debug!(line);

        if let Some(num_data) = line.split(":").last() {

            let num_series: Vec<&str> = num_data.split("|").collect();

            let winning_nums = parse_number_series(num_series[0]);
            let scratched_nums = parse_number_series(num_series[1]);

            let mut card_value = 0;
            for scratched_num in scratched_nums.iter() {
                for winning_num in winning_nums.iter() {

                    if scratched_num != winning_num {
                        continue;
                    }

                    card_value = increment_card_value(card_value);

                    debug!("\twinning number: {}", scratched_num);
                }
            }
            debug!("- card_value: {}", card_value);

            solution = solution + card_value;
        }
    }

    return solution; 

}


fn increment_card_value(
    current_value: usize)
    -> usize 
{

    let mut card_value = current_value;
    if card_value == 0 {
        card_value = 1;
    } else {
        card_value = card_value * 2;
    }
    return card_value

}

fn parse_number_series(
    series: &str) -> Vec<usize>{

    let mut results = Vec::new();

    for num_string in series.split(" ") {
        let num = num_string.parse::<usize>();

        if num.is_err(){
            continue;
        }

        results.push(num.unwrap());
    }

    return results;

}

