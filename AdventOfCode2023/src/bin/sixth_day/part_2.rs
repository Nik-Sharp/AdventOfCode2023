use core::panic;
use std::fs;

const EPSILON : f64 = 0.000001;

fn quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sqrt = ((b * b) - (4.0 * a * c)).sqrt();
    ((-b + sqrt) / (2.0 * a), (-b - sqrt) / (2.0 * a))
}
#[derive(Debug)]
struct Race {
    pub length: f64,
    pub record_distance: f64,
}

impl Race {
    pub fn get_amount_possible(&self) -> u32 {
        let x_intercepts = quadratic_equation(-1.0, self.length, -self.record_distance);

        let mut result = x_intercepts.1.ceil() as u32 - x_intercepts.0.ceil() as u32;

        if ((x_intercepts.1  - x_intercepts.0) - (result as f64)).abs() < EPSILON {
            result -= 1;
        }

        result
    }
}
fn main() {
    // Gets input from file
    let full_input;

    let mut inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| {
                x.trim()
                    .split_ascii_whitespace()
                    .filter(|x| !x.is_empty() && x.chars().next().unwrap().is_ascii_digit())
                    .collect::<String>()
                    .parse::<f64>()
                    .unwrap()
            })
        }

        Err(x) => panic!("Error in reading, {}", x),
    };
    let length = inputs.next().unwrap();
    let record = inputs.next().unwrap();

    let race = Race { length: length, record_distance: record };

    println!("{}", race.get_amount_possible());
}
