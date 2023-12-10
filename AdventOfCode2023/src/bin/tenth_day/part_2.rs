use core::panic;
use std::{fs, collections::HashSet};

use itertools::Itertools;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn get_group_from_pos(mut map : &mut Vec<Vec<char>>, (x, y) : (usize, usize)) -> Vec<(usize, usize)> {

    map[y][x] = ' ';
    // Gets surrounding chars
    let binding = [-1, 0, 1].map(|y_offset| [-1, 0, 1].map(|x_offset| {
        if x_offset == y_offset {
            return None;
        }

        if (x_offset < 0 && x == 0) || (y_offset < 0 && y == 0) {
            return None;
        }

        let surround_pos = (x as isize + x_offset, y as isize + y_offset);

        let empty = Vec::<char>::new();
        let char = map.get(surround_pos.1 as usize).unwrap_or(&empty).get(surround_pos.0 as usize).unwrap_or(&' ');
        if *char != '.' {
            return None;
        }
        Some(surround_pos)
        
    }));
    let surrounding = binding.iter().map(|x| x.iter().flatten().map(|x| (x.0 as usize, x.1 as usize))).flatten().collect_vec();

    let mut positions = vec![(x, y)];

    for char in surrounding {
        positions.append(&mut get_group_from_pos(&mut map, char))
    }

    return positions.iter().unique().map(|x| x.clone()).collect_vec();
}

fn get_groups(map : &mut Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {


    let mut groups = Vec::<Vec<(usize, usize)>>::new();

    let vertical_length = map.len();
    let horizontal_length = map.first().unwrap().len();

    for y in 0..vertical_length {
        for x in 0..horizontal_length {
            if *map.get(y).unwrap().get(x).unwrap() == '.' {
                groups.push(get_group_from_pos(map, (x, y)));
            }
            
        }
    }

    groups
}

fn get_next_from_starting(map : &Vec<Vec<char>>, (x,y) : (usize, usize)) -> (usize, usize, Direction) {
        let mut surrounding = Vec::<(char, (isize, isize))>::new();
        
        surrounding.push((*map.get(y + 1).unwrap_or(&Vec::<char>::new()).get(x).unwrap_or(&' '), (0, 1)));

        if y != 0 {
            surrounding.push((*map.get(y - 1).unwrap_or(&Vec::<char>::new()).get(x).unwrap_or(&' '), (0, -1)));
        }

        if x != 0 {
            surrounding.push((*map.get(y).unwrap_or(&Vec::<char>::new()).get(x-1).unwrap_or(&' '), (-1, 0)))
        }

        surrounding.push((*map.get(y).unwrap_or(&Vec::<char>::new()).get(x+1).unwrap_or(&' '), (1, 0)));

        for val in surrounding {
            if val.1.0 != 0 {
                let direction = match val.0 {
                    'L' => (Direction::Up, -1),
                    'J' => (Direction::Up, 1),
                    '7' => (Direction::Down, 1),
                    'F' => (Direction::Down, -1),
                    '-' => if val.1.0 == 1 {(Direction::Right, 1)} else {(Direction::Left, -1)}
                    _ => continue,
                };

                if val.1.0 == direction.1 {
                    return ((x as isize + direction.1) as usize, y, direction.0)
                }
            }
            else {
                let direction = match val.0 {
                    'L' =>  (Direction::Right, 1),
                    'J' => (Direction::Left, 1),
                    '7' => (Direction::Left, -1),
                    'F' => (Direction::Right, -1),
                    '|' => if val.1.1 == 1 {(Direction::Down, 1)} else {(Direction::Up, -1)}
                    _ => continue,
                };

                if val.1.1 == direction.1 {         
                    return (x, (y as isize + direction.1) as usize, direction.0)
                }
            }
        }
        panic!("Invalid input");
}

fn get_path(map : &Vec<Vec<char>>, (x, y, d) : (usize, usize, Direction)) -> HashSet<(usize, usize)> {

    let mut pos = (x, y);
    let mut direction = d;

    let mut positions = HashSet::<(usize, usize)>::new();

    while map[pos.1][pos.0]!= 'S' {

        positions.insert(pos);

        let current_char = map[pos.1][pos.0];

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
    return positions;
}

fn in_polygon(map : &Vec<Vec<char>>, path : &HashSet<(usize, usize)>, group : &Vec<(usize, usize)>) -> bool {
    let mut coord = group[0];

    let mut intersections = 0;

    let horizontal_len = map[0].len();

    let mut previous_letter : Option<char> = None;

    while coord.0 < horizontal_len {
        let current_char = map[coord.1][coord.0];

        if current_char == 'S' && ['7', 'J'].iter().any(|x| x == map[coord.1].get(coord.0 + 1).unwrap_or(&'-')){
            intersections -= 1;
        }

        if path.contains(&coord) && ['|', 'L', 'F', '7', 'J'].iter().any(|x| *x == current_char) {
            if ['|', '7', 'J'].iter().any(|x| *x == current_char) {
                intersections += 1;
            }
            if let Some(x) = previous_letter {
                if x == 'L' && current_char == 'J' {
                    intersections -= 1;
                }
                else if x == 'F' && current_char == '7' {
                    intersections -= 1;
                }
            }

            if current_char == '|' {
                previous_letter = None;
            }
            else {
                previous_letter = Some(current_char);
            }

        }

        coord.0 += 1
    }
    (intersections / 1) % 2 == 1
}
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

    let starting_y: usize = inputs.iter().enumerate().find(|x| x.1.contains(&'S')).unwrap().0;
    let starting_x: usize = inputs[starting_y].iter().enumerate().find(|x| *x.1 == 'S').unwrap().0;

    let next_from_starting =  get_next_from_starting(&inputs, (starting_x, starting_y));

    let path = get_path(&inputs, next_from_starting);

    for (y, line) in inputs.iter_mut().enumerate() {
        for (x, char) in line.iter_mut().enumerate() {
            if !path.contains(&(x, y)) && *char != 'S'{
                *char = '.';
            }

        }
    }
    let groups = get_groups(&mut inputs);
    
    let mut count = 0;


    for group in groups {
        if in_polygon(&inputs ,&path, &group) {
            count += group.len();
        }
    }

    println!("{}", count)
}
