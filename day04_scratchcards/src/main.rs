use std::{env, collections::HashMap};

use input::iterate_input;

mod input;

fn main() {

    //env::set_var("PRINT_DEBUG", "true");



    let solution_part1 = get_solution(false);
    let solution_part2 = get_solution(true);

    println!("------------------------------------");
    println!("Solution - Part 1: {}", solution_part1);
    println!("Solution - Part 2: {}", solution_part2);

}


fn get_solution(is_part_two: bool) -> usize {
    let mut solution = 0;

    let mut card_count: HashMap<usize,usize> = HashMap::new();

    for (card_ix, line) in iterate_input().enumerate(){
        debug!(line);

        if let Some(num_data) = line.split(":").last() {

            let num_series: Vec<&str> = num_data.split("|").collect();

            let winning_nums = parse_number_series(num_series[0]);
            let scratched_nums = parse_number_series(num_series[1]);

            if is_part_two {


                let current_card_count = card_count.entry(card_ix)
                    .and_modify(|e| {*e += 1})
                    .or_insert(1);

                for _ in 0..*current_card_count {
                    let winning_count = get_winning_count(&winning_nums, &scratched_nums);

                    for i in 0..winning_count {
                        let next_card_ix = card_ix + i + 1;

                        card_count.entry(next_card_ix)
                            .and_modify(|e| { *e += 1 })
                            .or_insert(1);
                    }
                }


            } else {

                let winning_count = get_winning_count(&winning_nums, &scratched_nums);

                let mut card_value = 0;
                for _ in 0..winning_count {
                    card_value = increment_card_value(card_value);
                }
                debug!("- card_value: {}", card_value);

                solution = solution + card_value;
            }
        }
    }

    if is_part_two {
        for key in card_count.keys() {
            debug!("Card {}:", key + 1);
            debug!("\t{}", card_count[key]);

            solution += card_count[key];
        }
    }

    return solution; 

}

fn process_card(
    winning_nums: &Vec<usize>,
    scratched_nums: &Vec<usize>,
    count: usize
    ) -> usize {

    debug!("-dive: {}", count);
    let mut count = count;
    let winning_count = get_winning_count(&winning_nums, &scratched_nums);

    for _ in 0..winning_count {
        count = count + process_card(&winning_nums, &scratched_nums, count);
    }

    return count;
}

fn get_winning_count(
    winning_nums: &Vec<usize>,
    scratched_nums: &Vec<usize>
    ) -> usize {


    let mut count = 0;
    for scratched_num in scratched_nums.iter() {
        for winning_num in winning_nums.iter() {

            if scratched_num != winning_num {
                continue;
            }

            count = count + 1;

            debug!("\twinning number: {}", scratched_num);
        }
    }
    return count;

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

