use core::panic;
use std::fs;

use itertools::Itertools;

fn main() {
    // Gets input from file
    let full_input;

    let mut inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input
                .split("\n")
                .map(|x| x.trim().chars().collect_vec())
                .map(|x| {
                    if x.iter().all(|y| *y == '.') {
                        vec![x.clone(), x]
                    } else {
                        vec![x]
                    }
                })
                .flatten()
                .collect_vec()
        }
        Err(x) => panic!("Error in reading, {}",  x),
    };
    
    for column_num in (0..inputs[0].len()).rev() {
        let mut column = inputs.iter().map(|x| x[column_num]);
        if column.all(|x| x == '.') {
            inputs.iter_mut().for_each(|x| x.insert(column_num, '.'))
        }
    }
    let mut galaxies = inputs
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, galaxy)| **galaxy == '#')
                .map(move |(x, _galaxy)| (x as isize, y as isize))
        })
        .flatten();

    println!("{:?}", galaxies.clone().collect_vec());

    let mut sum = 0;

    while let Some(galaxy_1) = galaxies.next() {
        for galaxy_2 in galaxies.clone() {
            sum += (galaxy_1.0 - galaxy_2.0).abs();
            sum += (galaxy_1.1 - galaxy_2.1).abs();
        }
    }

    println!("{}", sum);
}
