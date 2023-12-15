use std::env;
use std::str::FromStr;
use input::iterate_input;

mod input;

fn main() {

    std::env::set_var("PRINT_DEBUG", "true");
  
    let mut input = iterate_input();

    let mut line = input.next().unwrap();

    let times = parse_numbers::<usize>(line);
    
    line = input.next().unwrap();

    let distances = parse_numbers::<usize>(line);

    let mut solution = 1;
    for i in 0..times.len() {

        let time = times[i];
        let distance = distances[i];

        let ways_to_win = calc_ways_to_win(time, distance);
        
        debug!("time: {}", time);
        debug!("distance: {}", distance);
        debug!("ways to win : {}", ways_to_win);
    
        solution *= ways_to_win;
    }

    println!("Solution: {}", solution);
}

// calculates distance from race duration and button hold / charge time
fn race(duration: usize, charge_time: usize) -> usize {
    
    const CHARGE_ACCELERATION: usize = 1;

    let speed = charge_time * CHARGE_ACCELERATION;

    let distance = speed * (duration - charge_time);

    return distance;
}

// calculates distances for all possible races
fn all_the_races(duration: usize) -> Vec<usize> {

    let mut results = Vec::new();

    for i in 0..duration+1 {
        results.push( race(duration, i ) ); 
    }

    return results;
}

fn calc_ways_to_win(time: usize, distance_record: usize) -> usize {
  
    let race_results = all_the_races(time);

    let mut wins = 0;
    for result in race_results {
        if result > distance_record {
            wins += 1;
        }
    }

    return wins;
}


fn parse_numbers<T>(text: String) -> Vec<T>
where T: FromStr{

    let line_components = text.split(" ");
    let mut results = Vec::new(); 

    for component in line_components {
        if let Ok(component_num) = component.parse::<T>() {
            results.push(component_num);

        }
    }
 

    return results;
}
