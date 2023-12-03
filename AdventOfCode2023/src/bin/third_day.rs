use core::panic;
use std::fs;

fn main() {

    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| x.trim())
        },
        Err(x) => panic!("Error in reading, {}", x)
    };

    
    for (_index, input) in inputs.enumerate() {
        println!("{}", input);
    }
    
}
