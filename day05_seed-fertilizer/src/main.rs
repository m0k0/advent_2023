use std::{env, ops::Range, collections::HashMap, slice::Iter};
use crate::input::iterate_input;

mod input;
struct SeedMapEntry {
    coverage_range: Range<usize>,
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

        let param_dest = params[0].parse::<usize>();
        let param_source = params[1].parse::<usize>();
        let param_length = params[2].parse::<usize>();

        if param_dest.is_err() || param_source.is_err() || param_length.is_err() {
            return None; 
        }


        let param_dest = param_dest.unwrap();
        let param_source = param_source.unwrap();
        let param_length = param_length.unwrap();


        let entry = Self {
            dest_offset: (param_dest as i64) - (param_source as i64),
            coverage_range: param_source..(param_source + param_length - 1)
        };




        debug!("parsed: {}", definition_line);
        debug!("\toffset: {}", entry.dest_offset);
        debug!("\trange:");
        debug!("\t{}", entry.coverage_range.start);
        debug!("\t{}", entry.coverage_range.end);

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

    pub fn map_value(&self, source_value: usize) -> usize {


        for map_entry in &self.entries {
            if !map_entry.coverage_range.contains(&source_value){
                continue;
            }

            return source_value + map_entry.dest_offset as usize;
        }

        return source_value;

    }
}

fn parse_seeds(seed_line: &str) -> Vec<usize> {

    let mut seeds = Vec::new();
    let seed_line_segments = seed_line.split(" ");
    for seed_line_segments in seed_line_segments {

        let seed = seed_line_segments.parse::<usize>();
        if seed.is_ok() {
            seeds.push(seed.unwrap());
        }
    }

    return seeds;
}


fn main() {

    env::set_var("PRINT_DEBUG", "true"); 


    // parse input into data structures
    let mut seeds: Vec<usize> = Vec::new();
    let mut maps: HashMap<String, SeedMap> = HashMap::new();

    let mut is_first = true;
    let mut current_map_name = String::new();
    let mut current_map = SeedMap::new();

    for line in iterate_input() {

        // seed definition
        if line.starts_with("seeds:") {
            seeds = parse_seeds(line.as_str());
            continue;
        }

        // map definition
        if line.ends_with("map:") {

            if !is_first {
                // commit completed map
                maps.insert(
                    current_map_name,
                    current_map);

                current_map = SeedMap::new();

            }
            current_map_name = line.replace(" map:", "");
            is_first = false;

            continue;
        }

        // try parse map entry 
        if let Some(entry) = SeedMapEntry::parse(line.as_str()) {

            current_map.add_entry(entry);
        }
    }
    maps.insert(
        current_map_name,
        current_map);

   
    print_parsed_input(&seeds, &maps);


    // process input
    //
    
    //
}

fn print_parsed_input(seeds: &Vec<usize>, maps: &HashMap<String, SeedMap>) {
    
    debug!("seeds:");
    for s in seeds {
        debug!(s);
    }
    
    for map_key in maps.keys() {

        debug!("{} map:", map_key);

        for seed_map in maps.get(map_key) {
            for entry in seed_map.iterate_entries() {
                
                debug!("\toffset: {}", entry.dest_offset);
                debug!("\trange:");
                debug!("\t{}", entry.coverage_range.start);
                debug!("\t{}", entry.coverage_range.end);
            }
        }

    }
}


