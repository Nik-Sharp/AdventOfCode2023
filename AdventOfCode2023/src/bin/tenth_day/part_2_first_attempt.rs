use core::panic;
use std::{fs, ops::Index};

use itertools::Itertools;


struct Node {
    char : char,
    distance : Option<usize>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}
fn get_next_from_starting(map : &Vec<Vec<char>>, (x,y) : (usize, usize)) -> (usize, usize, Direction) {
        if y != 0 && *map.get(y - 1).unwrap_or(&vec![]).get(x).unwrap_or(&' ') == '|' {
            return (x, y-1, Direction::Up);
        }
        if *map.get(y + 1).unwrap_or(&vec![]).get(x).unwrap_or(&' ') == '|' {
            return (x, y+1, Direction::Down);
        }

        if x != 0 && *map.get(y).unwrap_or(&vec![]).get(x - 1).unwrap_or(&' ') == '-' {
            return (x-1, y, Direction::Left);
        }
        if *map.get(y).unwrap_or(&vec![]).get(x + 1).unwrap_or(&' ') == '-' {
            return (x+1, y, Direction::Right);
        }

        panic!("Invalid input");
}
fn get_group_from_pos(mut map : &mut Vec<Vec<char>>, (x, y) : (isize, isize)) -> Vec<(isize, isize)> {

    map[y as usize][x as usize] = ' ';
    // Gets surrounding chars
    let binding = [-1, 0, 1].map(|y_offset| [-1, 0, 1].map(|x_offset| {
        if x_offset == y_offset {
            return None;
        }

        if (x_offset < 0 && x == 0) || (y_offset < 0 && y == 0) {
            return None;
        }

        let surround_pos = (x + x_offset, y + y_offset);

        let empty = Vec::<char>::new();
        let char = map.get(surround_pos.1 as usize).unwrap_or(&empty).get(surround_pos.0 as usize).unwrap_or(&' ');
        if *char != '.' {
            return None;
        }
        Some(surround_pos)
        
    }));
    let surrounding = binding.iter().map(|x| x.iter().flatten()).flatten().collect_vec();

    let mut positions = vec![(x, y)];

    for char in surrounding {
        positions.append(&mut get_group_from_pos(&mut map, *char))
    }

    return positions
}
fn get_groups(mut map : &mut Vec<Vec<char>>) -> Vec<Vec<(isize, isize)>> {

    let mut groups = Vec::<Vec<(isize, isize)>>::new();

    let vertical_length = map.len();
    let horizontal_length = map.first().unwrap().len();

    for y in (0..vertical_length) {
        for x in (0..horizontal_length) {
            if *map.get(y).unwrap().get(x).unwrap() == '.' {
                groups.push(get_group_from_pos(map, (x as isize, y as isize)));
            }
            
        }
    }

    groups
}   
fn get_coord(mut map : &mut Vec<Vec<char>>, point : (isize, isize)) -> Option<char>{
    Some(*map.get(point.1 as usize)?.get(point.0 as usize)?)
}
/*
fn find_path(mut map : &mut Vec<Vec<char>>, point : (isize, isize), squeezing : bool) -> bool {
    let below_point = get_coord(map, (point.0, point.1 + 1));
    let below = (below_point, Direction::Down, '7', 'F', '|');

    let above_point = get_coord(map, (point.0, point.1 - 1));
    let above = (above_point, Direction::Up, 'J', 'L', '|');

    let forward_point = get_coord(map, (point.0 + 1, point.1));
    let forward = (forward_point, Direction::Right, 'L', 'F', '-');

    let behind_point = get_coord(map, (point.0 - 1, point.1));
    let behind = (behind_point, Direction::Left, 'J', '7', '-');

    let mut paths : Vec<bool> = vec![];

    // Check for squeezing
    for thingy in [below, above, forward, behind].iter() {
        if let Some(x) = thingy.0 {

            let change = match thingy.1 {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
            };

            let one = thingy.2;
            let two = thingy.3;

            if !squeezing {
                let can_squeeze = match x {
                    // why is this unreachable
                    one => get_coord(map, (point.0 + change.0, point.1 + change.1)).unwrap_or('A') == two,
                    two => get_coord(map, (point.0 - change.0, point.1 - change.1)).unwrap_or('A') == one,
                    _ => false
                };

                if can_squeeze {
                    return find_path(map, (point.0 + change.0, point.1 + change.1), true);
                }
            }
            else {
                let still_squeezing = match thingy.1 {
                    Direction::Left | Direction::Right => get_coord(map, point) == Some('-') && (get_coord(map, (point.0, point.1 + 1)) == Some('-')|| get_coord(map, (point.0, point.1 - 1)) == Some('-')),
                    Direction::Up | Direction::Down => get_coord(map, point) == Some('|') && (get_coord(map, (point.0 + 1, point.1)) == Some('|')|| get_coord(map, (point.0 - 1, point.1)) == Some('|')),
                };

                if still_squeezing {
                    return find_path(map, (point.0 + change.0, point.1 + change.1), true);
                }
            }
        }
    }

    //if map.get(point.1).unwrap_or(vec![]).get(point.0).unwrap_or('A') ||


    todo!()
}
fn can_escape(mut map : &mut Vec<Vec<char>>, group : Vec<(isize, isize)>) -> bool{

    let new_map = map.clone();
    let vertical_length = map.len()-1;
    let horizontal_length = map.first().unwrap().len();

    let on_edge = |x : &(isize, isize)|  x.0 == 0 || x.1==0 || x.0 == horizontal_length as isize - 1 || x.1 == vertical_length as isize - 1;

    if group.iter().any(|x| on_edge(x)) {
        return true;
    }

    false
}
 */

 
fn main() {

    // Gets input from file
    let full_input;

    let mut inputs = match fs::read_to_string("../input.txt") {
        Ok(x) => {
            full_input = x;
            full_input.split("\n").map(|x| x.trim().chars().collect_vec()).collect_vec()
        },
        Err(x) => panic!("Error in reading, {}", x)
    };

    let groups = get_groups(&mut inputs);

    



    
    
}
