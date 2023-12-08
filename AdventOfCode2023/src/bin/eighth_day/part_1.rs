use core::panic;
use std::{fs, collections::HashMap, thread::current};

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
struct GameInfo {
    directions : Vec<Direction>,
    map : HashMap<String, Split>
}
fn parse(lines : Vec<&str>) -> GameInfo{
    let mut map = HashMap::<String, Split>::new();

    let directions = lines[0].chars().map(|x| Direction::from(x.clone()));
    for line in lines.get(2..).unwrap() {
        let mut split = line.split_ascii_whitespace();

        let key = split.next().unwrap().to_owned();
        split.next();
        let left = split.next().unwrap().replace('(',"").replace(',',"");
        let right = split.next().unwrap().replace(')',"");

        map.insert(key, Split {left, right});
    }


    GameInfo { directions: directions.collect_vec(), map: map}
}    
fn main() {

    // Gets input from file
    let full_input;

    let game_info = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            parse(full_input.split("\n").map(|x| x.trim()).collect_vec())
        },
        Err(x) => panic!("Error in reading, {}", x)
    };


    let mut current_node = "AAA";
    let mut count : usize= 0;

    while current_node != "ZZZ" {
        current_node = match game_info.directions[count % game_info.directions.len()] {
            Direction::Left => &game_info.map.get(current_node).unwrap().left,
            Direction::Right => &game_info.map.get(current_node).unwrap().right,
        };

        count += 1;
    }

    println!("{}", count);
     
}
