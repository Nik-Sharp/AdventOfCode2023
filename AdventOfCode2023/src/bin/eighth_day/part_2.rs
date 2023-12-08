use core::panic;
use std::{fs, collections::HashMap, thread::current, cmp::Ordering};

use itertools::{KMerge, Itertools};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(char : char) -> Direction {
        if char == 'L' { Direction::Left } else { Direction::Right }
    }
}
#[derive(Debug)]
struct Split {
    left : String,
    right : String
}
struct GameInfo<'game> {
    directions : Vec<Direction>,
    map : HashMap<&'game str, Split>,
    starting_nodes : Vec<&'game str>
}

fn gcf(mut a : usize, mut b : usize) -> usize {
    while a > 0 && b > 0 {
        match a.cmp(&b) {
            Ordering::Greater => a = a % b,
            Ordering::Less => b = b % a,
            Ordering::Equal => return a
        }
    }

    return a + b;
}

fn lcm(mut a : usize, mut b : usize) -> usize {
    (a*b)/gcf(a, b)
}

fn run_path(starting_pos : &str, game_info : &mut GameInfo) -> usize {
    let mut current_node = starting_pos;
    let mut count : usize= 0;

    while current_node.chars().nth_back(0).unwrap() != 'Z' {
        current_node = match game_info.directions[count % game_info.directions.len()] {
            Direction::Left => &game_info.map.get(current_node).unwrap().left,
            Direction::Right => &game_info.map.get(current_node).unwrap().right,
        };

        count += 1;
    }

    count
}
fn parse(lines : Vec<&str>) -> GameInfo {
    let mut map = HashMap::<&str, Split>::new();
    let mut starting_nodes = Vec::<&str>::new();
    let directions = lines[0].chars().map(|x| Direction::from(x.clone()));

    for line in lines.get(2..).unwrap() {
        let mut split = line.split_ascii_whitespace();

        let key = split.next().unwrap();

        if key.chars().nth_back(0).unwrap() == 'A' {
            starting_nodes.push(key)
        }

        split.next();

        let left = split.next().unwrap().replace('(',"").replace(',',"");
        let right = split.next().unwrap().replace(')',"");

        map.insert(key, Split {left, right});
    }


    GameInfo { directions: directions.collect_vec(), map: map, starting_nodes : starting_nodes}
}    
fn main() {

    // Gets input from file
    let full_input;

    let mut game_info = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            parse(full_input.split("\n").map(|x| x.trim()).collect_vec())
        },
        Err(x) => panic!("Error in reading, {}", x)
    };

    let mut result : Option<usize> = None;
    for path in game_info.starting_nodes.clone() {
        let path_result = run_path(path, &mut game_info);

        result = match result {
            Some(x) => Some(lcm(path_result, x)),
            None => Some(path_result)
        }
    }

    println!("{}", result.unwrap());

     
}
