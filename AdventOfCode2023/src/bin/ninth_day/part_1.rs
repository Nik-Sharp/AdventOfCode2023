use core::panic;
use std::fs;

use itertools::Itertools;

struct Oasis {
    pub nums: Vec<i64>,
}

impl Oasis {
    fn get_next(&self) -> i64 {
        let mut sets: Vec<Vec<i64>> = vec![self.nums.clone()];

        while !sets.last().unwrap().iter().all(|x| *x == 0) {
            let mut last = sets.last().unwrap().iter().enumerate().collect::<Vec<(usize, &i64)>>();
            let mut set = Vec::<i64>::new();


            last.iter().for_each(|(i, _x)|  if *i > 0 {set.push(last[*i].1 - last[*i - 1].1)});

            sets.push(set);
            
        }
        
        sets.reverse();

        sets[0].push(0);

        let mut previous_difference = 0;

        for (index, set) in sets.iter_mut().enumerate() {
            if index == 0 {
                continue;
            }

            let difference = set.last().unwrap() + previous_difference;

            set.push(difference);

            previous_difference = difference;
        }

        *sets.last().unwrap().last().unwrap()
    }
}
fn main() {
    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| Oasis {
                nums: x
                    .trim()
                    .split_ascii_whitespace()
                    .filter(|x| !x.replace(" ", "").is_empty())
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            })
        }
        Err(x) => panic!("Error in reading, {}", x),
    };
    let mut sum = 0;
    for (_index, input) in inputs.enumerate() {
        sum += input.get_next();
    }

    println!("{}", sum);
}
