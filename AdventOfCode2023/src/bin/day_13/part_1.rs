use core::panic;
use std::fs;

use itertools::Itertools;
enum LineType {
    Horizontal,
    Vertical,
}
fn find_line(vec: &Vec<&str>) -> Option<usize> {
    for i in 1..(vec[0].len()) {
        let distance = i.min(vec[0].len()-i);

        if vec.iter().all(|line| {
            let left = line.get((i-distance)..i).unwrap().chars().rev().collect_vec();
            let right = line.get(i..(i+distance)).unwrap().chars().collect_vec();

            left == right
        }) {

            return Some(i);
        }
    }

    None
}
fn main() {
    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input
                .split("\n\n")
                .map(|x| x.trim().split("\n").collect_vec())
                .collect_vec()
        }
        Err(x) => panic!("Error in reading, {}", x),
    };

    let mut sum = 0;

    for note in inputs {
        if let Some(line) = find_line(&note) {
            sum += line;
        }
        
        //let rotated_note = (0..(note[0].len())).map(|x| (0..note.len()).map(|y| note[y].chars().nth(x).unwrap()));
        let mut rotated_note = (0..(note[0].len())).map(|_| String::new()).collect_vec();

        for x in 0..(note[0].len()) {
            for y in 0..note.len() {
                rotated_note[x].push(note[y].chars().nth(x).unwrap());
            }
        }

        if let Some(line) = find_line(&rotated_note.iter().map(|x| x.as_str()).collect_vec()) {
            sum += line * 100;
        }
    }
    println!("{}", sum)
}
