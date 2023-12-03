use core::panic;
use std::{fs, thread::current, collections::{HashSet, HashMap}};


fn main() {
    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| x.trim()).collect::<Vec<_>>()
        }
        Err(x) => panic!("Error in reading, {}", x),
    };


    let mut gears : HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut sum = 0;

    let mut add_to_gears = |coords : (usize, usize), val : u32| {

        if gears.contains_key(&coords) { gears.get_mut(&coords).unwrap().push(val) }
        else { gears.insert(coords, vec![val]); }
    };

    let mut check_vertical = |inputs : &Vec<&str>, index : usize, line_index: usize, current_num : u32, mut gear_part : &mut Option<(usize, usize)>| {
        let check = _check_vertical(&inputs, index, line_index);
        if let Some(x) = check.1 {
            //add_to_gears(x, current_num )
            *gear_part = check.1
        }
        return check.0;
    };
    //let 
    for (index, input) in inputs.iter().enumerate() {
        let mut current_num: Option<u32> = None;
        let mut is_part_symbol = false;
        let mut is_gear_part: Option<(usize, usize)> = None;

        for (line_index, char) in input.chars().enumerate() {
            if char.is_ascii_digit() {
                match current_num {
                    Some(x) => current_num = Some(x * 10 + (char as u32 - 48)),
                    None => {
                        is_part_symbol = false;
                        is_gear_part = None;
                        current_num = Some(char as u32 - 48);

                        if line_index > 0 && check_vertical(&inputs, index, line_index - 1, current_num.unwrap(), &mut is_gear_part) {
                            is_part_symbol = true;
                        }
                    }
                }

                if check_vertical(&inputs, index, line_index, current_num.unwrap(), &mut is_gear_part) {
                    is_part_symbol = true;
                }
            }

            else {
                if current_num == None { continue; }

                if line_index + 1 < input.len() {
                    let check = check_vertical(&inputs, index, line_index, current_num.unwrap(), &mut is_gear_part);
                    is_part_symbol = true;
                }

                if is_part_symbol {
                    sum += current_num.unwrap();
                }

                if let Some(x) = is_gear_part {
                    add_to_gears(x, current_num.unwrap());
                }
                current_num = None;
            }

            if line_index + 1 >= input.len() && current_num != None{
                if is_part_symbol {
                    sum += current_num.unwrap();
                }
                if let Some(x) = is_gear_part {
                    add_to_gears(x, current_num.unwrap());
                }
                current_num = None;
            }


        }
    }

    let mut gear_sum = 0;

    for x in gears {
        if (x.1).len() == 2 {
            gear_sum += (x.1)[0] * (x.1)[1];
        }
    }

    println!("{} {}", sum, gear_sum);
}

fn _check_vertical(inputs : &Vec<&str>, index : usize, line_index: usize) -> (bool, Option<(usize, usize)>){
    let check_array = if index == 0 { vec![index + 1, index] } 
                                  else if index + 1 >= inputs.len() { vec![index - 1, index] }
                                  else { vec![index - 1, index + 1, index] };
    for check in check_array {
        if let Some(x) = inputs.get(check) {
            if let Some(char_str) = x.get(line_index..=line_index) {
                let char = char_str.chars().next().unwrap();
                if is_symbol(&char){
                    return if char == '*' {(true, Some((check, line_index)))}  else {(true, None)};
                }
            }
        }
    }
    (false, None)
}

fn is_symbol(char : &char) -> bool { !char.is_ascii_digit() && char != &'.' }