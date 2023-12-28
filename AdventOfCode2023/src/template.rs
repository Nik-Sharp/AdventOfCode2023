use core::panic;
use std::fs;

use itertools::Itertools;

fn main() {
    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| x.trim()).collect_vec()
        }
        Err(x) => panic!("Error in reading, {}", x),
    };

    let rocks = inputs.iter().enumerate().map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, char)| if char == 'O' { Some((x, y)) } else { None })
            .flatten()
    }).group_by(|(x, y)| y);

    for (_index, input) in inputs.iter().enumerate() {
        println!("{}", input);
    }
}
