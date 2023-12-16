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

    let mut solution = 0;

    let mut directions = Vec::new();
    let mut nodes = HashMap::new();
    let mut starting_nodes = Vec::new();

    for (ix, line) in iterate_input().enumerate() {

        if ix == 0 {
            directions = parse_directions(line);
            continue;
        }

        if let Some(node) = parse_node(line)  {

            let node_label = node.label.clone();

            nodes.insert(node.label.clone(), node);

            if node_label.ends_with('A') {
                starting_nodes.push(node_label);
            }
        }
    }
    
    print_parsed_data(&directions, &nodes);


    //solution = walk_map_part_one(&directions, &nodes);

    //print!("Solution part 1: {}", solution);

    solution = walk_map_part_two(&starting_nodes, &directions, &nodes);

    print!("Solution part 2: {}", solution);


}

fn walk_map_part_two(
    starting_nodes: &Vec<String>, 
    directions: &Vec<bool>, 
    nodes: &HashMap<String, MapNode>) -> usize {
    
    let mut steps = 0;
    let mut current_nodes = Vec::new();
    let node_count = starting_nodes.len();
    for n in starting_nodes {
        current_nodes.push(n.to_string());
    }

    loop {
        
        let mut is_at_destination = true;

        for ix in 0..node_count {

            let current_node = match current_nodes.get(ix) {
                Some(v) => v,
                None => { panic!("Invalid node index '{}'", ix) }
            };

            let node_entry = match nodes.get(current_node) {
                Some(v) => v,
                None => { panic!("Node '{}' isn't on the map", current_node)}
            };

            let direction_ix = steps % directions.len();
            let direction = directions[direction_ix];

            if direction == NODE_DIRECTION_LEFT {
                current_nodes[ix] = node_entry.left.clone();
            } else if direction == NODE_DIRECTION_RIGHT {
                current_nodes[ix] = node_entry.right.clone();
            } else {
                panic!("Invalid direction at step {}", steps);
            }

            is_at_destination &= current_nodes[ix].ends_with('Z');
            
        }
        
        steps += 1;

        // check if at destination
        if is_at_destination {
            break;
        }

    }
    return steps;
}

fn walk_map_part_one(directions: &Vec<bool>, nodes: &HashMap<String, MapNode>) -> usize {
    
    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        
        let node_entry = match nodes.get(current_node) {
            Some(v) => v,
            None => { panic!("Node '{}' isn't on the map", current_node)}
        };

        let direction_ix = steps % directions.len();
        let direction = directions[direction_ix];

        if direction == NODE_DIRECTION_LEFT {
            current_node = node_entry.left.as_str();
        } else if direction == NODE_DIRECTION_RIGHT {
            current_node = node_entry.right.as_str();
        } else {
            panic!("Invalid direction at step {}", steps);
        }
        
        steps += 1;
    }
    return steps;
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

