use core::panic;
use std::fs;

use itertools::Itertools;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}
fn get_next_from_starting(map : &Vec<&str>, (x,y) : (usize, usize)) -> (usize, usize, Direction) {
        if y != 0 && map.get(y - 1).unwrap_or(&"").chars().nth(x).unwrap_or(' ') == '|' {
            return (x, y-1, Direction::Up);
        }
        if map.get(y + 1).unwrap_or(&"").chars().nth(x).unwrap_or(' ') == '|' {
            return (x, y+1, Direction::Down);
        }

        if x != 0 && map.get(y).unwrap_or(&"").chars().nth(x - 1).unwrap_or(' ') == '-' {
            return (x-1, y, Direction::Left);
        }
        if map.get(y ).unwrap_or(&"").chars().nth(x + 1).unwrap_or(' ') == '-' {
            return (x+1, y, Direction::Right);
        }

        panic!("Invalid input");
}
fn main() {

    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| x.trim()).collect_vec()
        },
        Err(x) => panic!("Error in reading, {}", x)
    };

    let starting_y: usize = inputs.iter().enumerate().find(|x| x.1.contains('S')).unwrap().0;
    let starting_x: usize = inputs[starting_y].chars().enumerate().find(|x| x.1 == 'S').unwrap().0;

    let next_from_starting =  get_next_from_starting(&inputs, (starting_x, starting_y));

    let mut pos = (next_from_starting.0, next_from_starting.1);
    let mut direction = next_from_starting.2;

    let mut count = 0;

    while inputs[pos.1].chars().nth(pos.0).unwrap()!= 'S' {

        count += 1;

        let current_char = inputs[pos.1].chars().nth(pos.0).unwrap();

        if !(current_char == '|' || current_char == '-') {
            direction = match current_char {
                'L' => if direction == Direction::Left { Direction::Up }  else {Direction::Right},
                'J' => if direction == Direction::Right { Direction::Up }  else {Direction::Left},
                '7' => if direction == Direction::Right { Direction::Down }  else {Direction::Left},
                'F' => if direction == Direction::Left { Direction::Down }  else {Direction::Right},

                _ => panic!(),
            };
        }

        pos = match direction {
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
        };


    }
    println!("{}", (count + 1) / 2);

    
    
}
