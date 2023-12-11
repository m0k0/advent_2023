use std::env;
use crate::input::iterate_input;

mod input;

fn main() {
    
    env::set_var("PRINT_DEBUG", "true"); 

    for line in iterate_input() {
        debug!(line);
    }

}
