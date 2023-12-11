use core::panic;
use std::fs;

use itertools::Itertools;

fn main() {
    // Gets input from file
    let full_input;

    let mut inputs = match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input
                .split("\n")
                .map(|x| x.trim().chars().collect_vec())
                .collect_vec()
        }
        Err(x) => panic!("Error in reading, {}", x),
    };

    let mut empty_rows = Vec::<usize>::new();
    let mut empty_columns = Vec::<usize>::new();

    for (index, row) in inputs.iter().enumerate() {
        if row.iter().all(|x| *x == '.') {
            empty_rows.push(index);
        }
    }

    for column_num in (0..inputs[0].len()).rev() {
        let mut column = inputs.iter().map(|x| x[column_num]);
        if column.all(|x| x == '.') {
            empty_columns.push(column_num);
        }
    }
    let mut galaxies = inputs
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, galaxy)| **galaxy == '#')
                .map(move |(x, _galaxy)| (x, y))
        })
        .flatten();

    let mut sum = 0;

    while let Some(galaxy_1) = galaxies.next() {
        for galaxy_2 in galaxies.clone() {
            let min_x = (galaxy_1.0).min(galaxy_2.0);
            let max_x = (galaxy_1.0).max(galaxy_2.0);

            let million_columns = empty_columns
                .iter()
                .filter(|x| **x >= min_x && **x <= max_x)
                .count();

            sum += million_columns * 1000000 - million_columns;
            sum += max_x - min_x;

            let min_y = (galaxy_1.1).min(galaxy_2.1);
            let max_y = (galaxy_1.1).max(galaxy_2.1);

            let million_rows = empty_rows
                .iter()
                .filter(|y| **y >= min_y && **y <= max_y)
                .count();
            sum += million_rows * 1000000 - million_rows;
            sum += max_y - min_y;
        }
    }
    println!("{}", sum);
}
