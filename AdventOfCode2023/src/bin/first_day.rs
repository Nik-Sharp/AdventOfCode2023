use core::panic;
use std::{io, collections::HashSet};

fn main() {
    let mut inputs = vec![];

    loop {
        let mut str = String::new();
        let std_in = io::stdin().read_line(&mut str);

        match std_in {
            Ok(_) => {
                if str.trim_end().is_empty() {
                    break;
                }
                else {
                    inputs.push(str);
                }
            }
            Err(error) => {panic!("Error in reading, {}", error)},
        }
    }

    let mut count : u32 = 0;

    for x in inputs {
        let mut first_number_char : Option<u8> = None;
        let mut last_number_char : Option<u8> = None;
        
        let chars = x.chars();

        for (i, _y) in chars.enumerate() {

            let num = check_for_num(&x, i);

            if let Some(x) = num {
                if first_number_char == None {
                    first_number_char = Some(x);
                }

                last_number_char = Some(x);

            }
        }

        if let Some(_x) = first_number_char {

            let first_number = first_number_char.unwrap() as u32;
            let last_number = last_number_char.unwrap() as u32;
            
            count += first_number * 10 + last_number;
        }
    }

    println!("{}", count);
    
}

const CONVERSIONS : [&str; 9]= ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn check_for_num(string : &str, index : usize) -> Option<u8> {

    if let Some(str_char) = string.get(index..=index) {

        let char = str_char.chars().nth(0)?;

        if char.is_ascii_digit() {
            return Some(char as u8 - 48);
        }
        
        for (i, str) in CONVERSIONS.into_iter().enumerate(){
            let length = str.len();
            if string.get(index..(index + length)) == Some(str) { return Some((i + 1) as u8); }
        }

    }

    return None;


}