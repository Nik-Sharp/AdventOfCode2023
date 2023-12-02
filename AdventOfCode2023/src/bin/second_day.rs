use core::panic;
use std::{io, slice::Split};

// Note this specifically solves part two, I refactored my code after I solved part 1 to solve part 2
fn main() {
    let max = (12, 13, 14);
    let mut game_num = 0;
    let mut first_sum = 0;
    let mut second_sum = 0;
    'line_loop : loop {
        game_num+=1;

        let mut str = String::new();
        let std_in = io::stdin().read_line(&mut str);

        let mut remove_start;
        let split_input;    

        let mut min = (0,0,0);

        match std_in {
            Ok(_) => {
                if str.trim_end().is_empty() {
                    break;
                }
                else {
                    remove_start = str.split(": ");
                    remove_start.next();
                    //println!("{:?}", remove_start);
                    split_input = remove_start.next().unwrap().split(";");
                }
            }
            Err(error) => {panic!("Error in reading, {}", error)},
        }

        let mut valid_game = true;
        for input in split_input {
            let amount_tuple = parse_sample(input);

            min.0 = min.0.max(amount_tuple.0);
            min.1 = min.1.max(amount_tuple.1);
            min.2 = min.2.max(amount_tuple.2);

            if amount_tuple.0 > max.0 || amount_tuple.1 > max.1 || amount_tuple.2 > max.2 {
                valid_game = false;
            }


        }

        //println!("{:?}", min);
        let power = min.0 * min.1 * min.2;
        if valid_game { first_sum += game_num; }
        second_sum += power;
    }


    println!("{} {}", first_sum, second_sum);
    

}
fn parse_sample(string : &str) -> (i32, i32, i32) {
    let trimmed = string.trim();
    let split = trimmed.split(",");

    let mut item_tuple = (0, 0, 0);
    for item in split {
        let split_item = item.trim().split_ascii_whitespace().collect::<Vec<_>>();
        
        let number = match split_item[0].parse::<i32>() {
            Ok(x) => x,
            Err(_) => panic!("{}", split_item[0])
        };

        let color = split_item[1];

        match color {
            "red" => item_tuple.0 += number,
            "green" => item_tuple.1 += number,
            "blue" => item_tuple.2 += number,
            _ => panic!("Invalid color")
        }
    }   

    item_tuple
    
}
