use std::{fs, collections::HashSet};

fn main() {

    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| x.trim())
        },
        Err(x) => panic!("Error in reading, {}", x)
    };
    
    let mut sum = 0;

    for (_index, input) in inputs.enumerate() {
        let mut split_input = input.split(':');
        
        split_input.next();

        let mut input_split = split_input.next().unwrap().trim().split('|');

        let winning_numbers = input_split.next().unwrap().trim().split(' ').filter(|x| !x.trim().is_empty()).collect::<HashSet<&str>>();

        let card_numbers = input_split.next().unwrap().trim().split(' ');

        let count = card_numbers.filter(|x| winning_numbers.contains(x.trim())).count();

        if count > 0 { sum += 2_i32.pow(count as u32 - 1) } 

        
    }

    println!("{}", sum)
    
}