use std::{fs, collections::HashSet};

struct Card<'card> {
    pub winning_numbers : HashSet<&'card str>,
    pub card_numbers : Vec<&'card str>,
}

impl<'card> Card<'card> {
    pub fn new(string : &'card str) -> Card {
        let mut split_input = string.split(':');
        
        split_input.next();

        let mut input_split = split_input.next().unwrap().trim().split('|');

        let winning_numbers = input_split.next().unwrap().trim().split(' ').filter(|x| !x.trim().is_empty()).collect::<HashSet<&str>>();

        let card_numbers = input_split.next().unwrap().trim().split(' ').collect();

        return Card { winning_numbers, card_numbers : card_numbers }
    }
}

fn main() {

    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| Card::new(x.trim()))
        },
        Err(x) => panic!("Error in reading, {}", x)
    };
    
    let mut sum = 0;

    for (_index, card) in inputs.enumerate() {
        
        let count = card.card_numbers.iter().filter(|x| card.winning_numbers.contains(x.trim())).count();

        if count > 0 { sum += 2_i32.pow(count as u32 - 1) } 

    }

    println!("{}", sum)
    
}