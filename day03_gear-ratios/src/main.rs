use std::{env, array, collections::VecDeque};

use input::{iterate_input};

mod input;


struct PartNumber {
    value: String,
    index: usize,
    length: usize,
    is_qualified: bool
}

struct Symbol {
    value: char,
    index: usize
}

fn find_parts_adjacent_to_symbols(
    symbols: &Vec<Symbol>, 
    part_ids_in_range: &Vec<usize>,
    parts_store: &Vec<PartNumber>) 
    -> Vec<usize> {

    let mut result: Vec<usize> = Vec::new();

    for symbol in symbols {
        debug!("symbol: {}", symbol.value); 

        for part_id in part_ids_in_range {

            let ref part = parts_store[*part_id];
            if is_part_adjacent(&part, symbol.index) {
                result.push(*part_id);
            }
        }
    }

    return result;
}

fn is_part_adjacent(
    part: &PartNumber,
    index: usize) 
    -> bool{
    
    let mut start = 0;
    if part.index > 0 {
        start = part.index - 1;
    }
    let end = part.index + part.length; 

    if index >= start && index <= end {

        debug!("\t   - adjacent: {}", part.value);

        return true;
    }
    
    debug!("\t ! - not adjacent: {}", part.value );
    /*
    debug!("\t\t ix: {}", index);
    debug!("\t\t pix: {}", part.index);
    debug!("\t\t s: {}", start);
    debug!("\t\t e: {}", end);
    */
    return false;
}


fn main() {
   
    env::set_var("PRINT_DEBUG", "true");

    const PADDING_CHAR: char = '.';
    const RADAR_LINE_RANGE: usize = 1;
    
    
    let mut parts_store = Vec::new();
    let mut parts_sum: usize = 0;

    let mut part_radar: VecDeque<Vec<usize>> = VecDeque::new(); 
    let mut symbol_radar: VecDeque<Vec<Symbol>> = VecDeque::new(); 
    


    for (line_count, line) in iterate_input().enumerate() {
        debug!(line);
        
        // part radar
        if part_radar.len() > (RADAR_LINE_RANGE + 1) * 2 {
            part_radar.pop_front();
        }
        
        // symbol radar
        if symbol_radar.len() > RADAR_LINE_RANGE * 2 {
           

            //debug!("\tsymbol_radar size: {}", symbol_radar.len());
            //debug!("\tpart_radar size: {}", part_radar.len());

            if let Some(popped_symbols) = symbol_radar.pop_front() {

                let mut part_ids_in_range: Vec<usize> = Vec::new();
                for i in 0..(RADAR_LINE_RANGE*2)+1 {

                    if let Some(parts_on_line) = part_radar.get(i) {
                        //debug!("part_radar index: {}", i);
                        for part_id in parts_on_line {
                           
                            //debug!("- part in range: {}", part.value);
                            part_ids_in_range.push(*part_id);
                        }
                    }
                }

                
               let adjacent_part_ids = find_parts_adjacent_to_symbols(
                    &popped_symbols, &part_ids_in_range, &parts_store);

                for part_id in adjacent_part_ids {
                    let ref mut part = parts_store[part_id];
                    part.is_qualified = true;
                }

            }


        }

        let mut line_part_radar: Vec<usize> = Vec::new();
        let mut line_symbol_radar: Vec<Symbol> = Vec::new();
        

        let mut part_buffer = String::new(); 

        for (line_pos, c) in line.chars().enumerate() {
            
            if c.is_numeric() {
                part_buffer.push(c); 

            } else {

                if !part_buffer.is_empty() {
                    // commit part number buffer to store
                    let part_buffer_len = part_buffer.len();
                    
                    debug!("commit part: {}", part_buffer);
                    debug!("\tlen: {}", part_buffer_len);
                    debug!("\tline_pos: {}", line_pos);


                    let part = PartNumber {
                        value: part_buffer,
                        index: line_pos - part_buffer_len,
                        length: part_buffer_len,
                        is_qualified: false
                    };
                    parts_store.push(part);
                    let part_id = parts_store.len()-1;

                    line_part_radar.push(part_id);
                    part_buffer = String::new();
                }

 
                if c != PADDING_CHAR {
                    // commit symbol to radar
                    let symbol = Symbol {
                        value: c,
                        index: line_pos
                    };

                    line_symbol_radar.push(symbol);

                }
            }

        }
        part_radar.push_back(line_part_radar); 
        symbol_radar.push_back(line_symbol_radar);
    }

    for part in parts_store {
        if part.is_qualified {
            let part_value = match part.value.parse() {
                Ok(v) => v,
                Err(_) => 0
            };
            parts_sum = parts_sum + part_value;
        }
    }

    println!("The answer is: {}", parts_sum);
}
