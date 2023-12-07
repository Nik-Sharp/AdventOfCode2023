use core::panic;
use itertools::{Groups, Itertools};
use std::{
    collections::{BTreeMap, HashMap},
    fs, cmp::Ordering,
};

use fancy_regex;

#[derive(Eq, PartialOrd, PartialEq, Ord, Copy, Clone, Debug)]
enum TypesOfPairs {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Card<'card> {
    pub card: &'card str,
    pub bid: u32,
}

impl<'card> From<&'card str> for Card<'card> {
    fn from(string: &'card str) -> Card {
        let mut split: std::str::SplitAsciiWhitespace<'_> = string.split_ascii_whitespace();
        return Card {
            card: split.next().unwrap(),
            bid: split.next().unwrap().parse().unwrap(),
        };
    }
}

impl<'card> Card<'card> {
    fn get_rank(&self) -> TypesOfPairs{
        let cards = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
        let max = *cards.map(|x| Self::_get_rank(&self.card.replace("J", x))).iter().max().unwrap();
        max
        
    }
    fn _get_rank(card : &str) -> TypesOfPairs {
        let groupings = card
            .chars()
            .fold(HashMap::<char, u8>::new(), |mut init, x| {
                if init.contains_key(&x) {
                    *init.get_mut(&x).unwrap() += 1;
                } else {
                    init.insert(x, 1);
                }
                init
            }).into_iter().map(|x| x.1).collect::<Vec<_>>();
            
            let length = groupings.len();
        
        
        if length == 1 { return TypesOfPairs::FiveOfAKind; }
        if length == 2 && (groupings[0] == 1 || groupings[0] == 4) { return TypesOfPairs::FourOfAKind; }
        if length == 2 && (groupings[0] == 2 || groupings[0] == 3) { return TypesOfPairs::FullHouse; }
        if length == 3 && (groupings.iter().any(|x| *x == 3)) { return TypesOfPairs::ThreeOfAKind; }
        if length == 3 && (groupings.iter().filter(|x| **x == 2).count() == 2) { return TypesOfPairs::TwoPair; }
        if length == 4 { return TypesOfPairs::OnePair; }
        
        TypesOfPairs::HighCard
    }

    fn compare(&self, card : &Card) -> Ordering {
        let comparisons : HashMap<char, u8> = HashMap::from([
            ('J', 1), ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6),
            ('7', 7), ('8', 8), ('9', 9), ('T', 10), ('Q', 11), ('K', 12), ('A', 13)
        ]);

        let self_rank = self.get_rank() as u8;
        let card_rank = card.get_rank() as u8;

        return match card_rank.cmp(&self_rank) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let zipped = self.card.chars().zip(card.card.chars());

                for (self_char, card_char) in zipped {
                    match comparisons.get(&card_char).unwrap().cmp(comparisons.get(&self_char).unwrap()) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue
                    }
                }

                panic!("Same card")
            }
        }

}
}
fn main() {
    // Gets input from file
    let full_input;

    let cards = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| Card::from(x.trim()))
        }
        Err(x) => panic!("Error in reading, {}", x),
    };
    println!("{:?}", cards.sorted_by(|i, x| i.compare(&x)).rev().enumerate().fold(0, |sum, (index, card)| sum + card.bid * (index as u32+ 1)));

    
}
