use std::{env, collections::HashMap};

use input::iterate_input;

mod input;

struct MapNode {
    label: String,
    left: String,
    right: String,
}


const NODE_DIRECTION_LEFT: bool = false;
const NODE_DIRECTION_RIGHT: bool = true;


fn main() {

    env::set_var("PRINT_DEBUG", "true");

    let solution = 0;

    let mut directions = Vec::new();
    let mut nodes = HashMap::new();


    for (ix, line) in iterate_input().enumerate() {

        if ix == 0 {
            directions = parse_directions(line);
            continue;
        }

        if let Some(node) = parse_node(line)  {
            nodes.insert(node.label.to_string(), node);
        }
    }
    
    print_parsed_data(&directions, &nodes);

    print!("Solution: {}", solution);
}

fn print_parsed_data(directions: &Vec<bool>, nodes: &HashMap<String, MapNode>) {
    
    for d in directions {
        if *d == NODE_DIRECTION_LEFT {
            debug!("left");
        } else if *d == NODE_DIRECTION_RIGHT {
            debug!("right");
        }
    }

    for key in nodes.keys() {
        if let Some(n) = nodes.get(key) {
            debug!(format!("node: {}, left: {}, right: {}",
                       n.label,
                       n.left,
                       n.right));
        }
    }

}

fn parse_directions(line: String) -> Vec<bool> {

    let mut directions = Vec::new();

    for c in line.chars() {
        if c == 'L' {
            directions.push(NODE_DIRECTION_LEFT);
        } else if c == 'R' {
            directions.push(NODE_DIRECTION_RIGHT);
        }
    }

    return directions;
}

fn parse_node(line: String) -> Option<MapNode> {
    
    if line.is_empty() {
        return None;
    }


    let node = MapNode { 
        label: line[0..3].to_string(),
        left: line[7..10].to_string() ,
        right: line[12..15].to_string()
    };
    

    return Some(node);

}

