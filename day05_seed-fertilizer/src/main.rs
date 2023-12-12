use std::{env, ops::Range};
use crate::input::iterate_input;

mod input;
struct SeedMapEntry {
    coverage_range: Range<usize>,
    dest_offset: i32
}

impl SeedMapEntry {

    

    // parse a map definition line 
    // e.g. 50 98 2
    pub fn parse(definition_line: &str) -> Option<SeedMapEntry> {
        let params: Vec<&str> = definition_line.split(" ").collect();

     
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
            dest_offset: param_dest as i32 - param_source as i32,
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

struct SeedMap<'a> {
    name: &'a str,
    entries: Vec<SeedMapEntry>,
}


impl<'a> SeedMap<'a> {

    pub fn new(name: &'a str) -> Self {
        return Self {
            name,
            entries: Vec::new()
        };
    }

    pub fn add_entry(&mut self, entry: SeedMapEntry) {
        self.entries.push(entry);
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


fn main() {
    
    env::set_var("PRINT_DEBUG", "true"); 

    for line in iterate_input() {
    //    debug!(line);
    }
    
    let mut map = SeedMap::new("test");
    
    if let Some(entry) = SeedMapEntry::parse("52 50 48") {
        map.add_entry(entry);
    }
    let dest_value = map.map_value(79);
   
    debug!("dest value: {}", dest_value);


}
