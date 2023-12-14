use std::{env, ops::Range, slice::Iter};
use crate::input::iterate_input;

mod input;
struct SeedMapEntry {
    coverage_range: Range<i64>,
    dest_offset: i64
}

impl SeedMapEntry {



    // parse a map definition line 
    // e.g. 50 98 2
    pub fn parse(definition_line: &str) -> Option<SeedMapEntry> {
        let params: Vec<&str> = definition_line.split(" ").collect();

        if params.len() != 3 {
            return None;
        }

        let param_dest = params[0].parse::<i64>();
        let param_source = params[1].parse::<i64>();
        let param_length = params[2].parse::<i64>();

        if param_dest.is_err() || param_source.is_err() || param_length.is_err() {
            return None; 
        }


        let param_dest = param_dest.unwrap();
        let param_source = param_source.unwrap();
        let param_length = param_length.unwrap();


        let entry = Self {
            dest_offset: param_dest - param_source,
            coverage_range: param_source..(param_source + param_length - 1)
        };



        /*
           debug!("parsed: {}", definition_line);
           debug!("\toffset: {}", entry.dest_offset);
           debug!("\trange:");
           debug!("\t{}", entry.coverage_range.start);
           debug!("\t{}", entry.coverage_range.end);
           */
        return Some(entry);
    }



}

struct SeedMap {
    entries: Vec<SeedMapEntry>,
}


impl SeedMap {

    pub fn new() -> Self {
        return Self {
            entries: Vec::new()
        };
    }

    pub fn add_entry(&mut self, entry: SeedMapEntry) {
        self.entries.push(entry);
    }

    pub fn iterate_entries(&self) -> Iter<SeedMapEntry> {
        return self.entries.iter(); 
    }

    pub fn map_value(&self, source_value: i64) -> i64 {


        for map_entry in &self.entries {
            if !map_entry.coverage_range.contains(&source_value){
                continue;
            }

            return source_value + map_entry.dest_offset as i64;
        }

        return source_value;

    }
}

fn parse_seeds(seed_line: &str) -> Vec<i64> {

    let mut seeds = Vec::new();
    let seed_line_segments = seed_line.split(" ");
    for seed_line_segments in seed_line_segments {

        let seed = seed_line_segments.parse::<i64>();
        if seed.is_ok() {
            seeds.push(seed.unwrap());
        }
    }

    return seeds;
}


fn parse_seeds_part_two(seed_line: &str) -> Vec<Range<i64>> {

    let mut seeds = Vec::new();
    let seed_line_segments = seed_line.split(" ");

    let mut current_seed_start = 0;
    let mut current_seed_length = 0;

    for (ix, seed_line_segments) in seed_line_segments.enumerate() {


        let number = match seed_line_segments.parse::<i64>() {
            Ok(v) => v,
            Err(_) => { continue }//{ panic!("Invalid seed value: {}", e) }
        };

        let is_seed_start = ix % 2 == 1; // first number is ix == 1


        if is_seed_start {
            current_seed_start = number;
        } else {
            current_seed_length = number;

            seeds.push(
                current_seed_start..(current_seed_start + current_seed_length-1)
                );
        }

    }

    return seeds;
}


fn main() {

    //env::set_var("PRINT_DEBUG", "true"); 


    // parse input into data structures
    let mut seeds: Vec<i64> = Vec::new();
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    let mut maps: Vec<SeedMap> = Vec::new();

    let mut is_first = true;
    let mut current_map = SeedMap::new();

    for line in iterate_input() {

        // seed definition
        if line.starts_with("seeds:") {
            seeds = parse_seeds(line.as_str());
            seed_ranges = parse_seeds_part_two(line.as_str());
            continue;
        }

        // map definition
        if line.ends_with("map:") {

            if !is_first {
                // commit completed map
                maps.push(current_map);
                current_map = SeedMap::new();

            }
            is_first = false;

            continue;
        }

        // try parse map entry 
        if let Some(entry) = SeedMapEntry::parse(line.as_str()) {
            current_map.add_entry(entry);
        }
    }
    maps.push(current_map);


    print_parsed_input(&seeds, &maps);



    // process seeds

    let solution_part1 = find_lowest_location(&seeds, &maps);
    println!("Solution 1: {}", solution_part1);

    let solution_part2 = find_lowest_location_range(&seed_ranges, &maps);
    println!("Solution 2: {}", solution_part2);

}

fn find_lowest_location_range(
    seed_ranges: &Vec<Range<i64>>, 
    maps: &Vec<SeedMap>) -> i64 {

    let mut lowest_location = 0;

    for range in seed_ranges {
        println!("Starting seed range: {}", range.start);

        for seed in range.start..range.end {

            let mapping_result = map_seed_to_location(&seed, maps);

            if lowest_location == 0 || mapping_result < lowest_location {
                lowest_location = mapping_result;
            }
        }
    }

    return lowest_location;
}

fn find_lowest_location(seeds: &Vec<i64>, maps: &Vec<SeedMap>)
    -> i64 {

        let mut lowest_location = 0;
        for seed in seeds {

            let mapping_result = map_seed_to_location(seed, maps);

            if lowest_location == 0 || mapping_result < lowest_location {
                lowest_location = mapping_result;
            }

        }
        return lowest_location;

    }

fn map_seed_to_location(
    seed: &i64, 
    maps: &Vec<SeedMap>
    ) -> i64 {


    let mut mapping_result = *seed;

    for map in maps {

        mapping_result = map.map_value(mapping_result);

    }

    return mapping_result;
}


fn print_parsed_input(seeds: &Vec<i64>, maps: &Vec<SeedMap>) {

    debug!("seeds:");
    for s in seeds {
        debug!(s);
    }

    for (ix, map) in maps.iter().enumerate() {

        debug!("map [{}]:", ix);

        for entry in map.iterate_entries() {

            debug!("\toffset: {}", entry.dest_offset);
            debug!("\trange:");
            debug!("\t{}", entry.coverage_range.start);
            debug!("\t{}", entry.coverage_range.end);
        }

    }
}


