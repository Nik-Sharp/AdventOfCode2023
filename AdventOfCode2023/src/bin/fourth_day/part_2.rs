use std::{fs, collections::HashSet};

struct Card<'card> {
    pub winning_numbers : HashSet<&'card str>,
    pub card_numbers : Vec<&'card str>,
    pub amount : usize
}

impl<'card> Card<'card> {
    pub fn new(string : &'card str) -> Card {
        let mut split_input = string.split(':');
        
        split_input.next();

        let mut input_split = split_input.next().unwrap().trim().split('|');

        let winning_numbers = input_split.next().unwrap().trim().split(' ').filter(|x| !x.trim().is_empty()).collect::<HashSet<&str>>();

        let card_numbers = input_split.next().unwrap().trim().split(' ').collect();

        return Card { winning_numbers, card_numbers : card_numbers, amount: 1 }
    }
}

fn main() {

    // Gets input from file
    let full_input;

    let mut inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| Card::new(x.trim())).collect::<Vec<_>>()
        },
        Err(x) => panic!("Error in reading, {}", x)
    };
    
    let inputs_len = inputs.len();
    let mut card_count = 0 as usize;
    let mut index = 0;

    while index < inputs.len() {
        
        let mut next_cards = inputs.get_mut(index..).unwrap().iter_mut().enumerate();
        println!("{}", next_cards.len());
        let card = next_cards.next().unwrap().1;
        let count = card.card_numbers.iter().filter(|x| card.winning_numbers.contains(x.trim())).count() as usize;

        let amount_left = if inputs_len - 1 - index  > 0 { inputs_len - 1 - index } else {0};

        for (i, peek_card) in next_cards {
            let new_count = ( count as i64 - (i as i64 - 1)) as i64;
            if new_count <= 0 || amount_left <= 0 { break; }
            //println!("{}", amount_left);
            peek_card.amount += ( new_count as usize ).div_ceil(amount_left) * card.amount as usize;

            println!("Card {}: {} {} {}",  index + i, new_count, amount_left, peek_card.amount);
        }

        card_count += card.amount;
        println!("Card count {}", card_count);
        index += 1;

    }

    println!("{}", card_count)
    
}