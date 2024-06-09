use core::panic;
use std::collections::HashMap;
use std::{cmp::Ordering, collections::HashSet, fs};
use std::fmt::Debug;
use itertools::Itertools;

extern crate rayon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Splits {
    changed_since_last_split: bool,
    splits: Vec<u16>,
}

impl Debug for Splits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?})", self.changed_since_last_split, self.splits)
    }
}

impl Splits {
    // returns valid if the split is still valid (e.g hasn't gone over)
    fn add_damaged(&mut self) -> bool {
        if self.splits.is_empty() || self.splits[0] == 0 {
            return false;
        }
        self.changed_since_last_split = true;

        self.splits[0] -= 1;

        true
    }

    fn split(&mut self) -> bool {
        if self.splits.is_empty() || !self.changed_since_last_split {
            return true;
        }

        if self.splits[0] != 0 {
            return false;
        }

        self.changed_since_last_split = false;
        self.splits.remove(0);
        true
    }

    fn is_complete(&self) -> bool {
        (self.splits.len() == 1 && self.splits[0] == 0) || self.splits.is_empty()
    }
}
#[derive(Debug, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}
impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Spring::Unknown,
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => panic!(),
        }
    }
}
#[derive(Debug, Hash)]
struct Row {
    pub row: Vec<Spring>,
    //pub blocks: Vec<&'row str>,
    pub splits: Splits,
}

impl Row {
    fn new(mut row: Vec<String>) -> Row {
        row[0] = (0..5).map(|_| row[0].clone()).join("?");
        row[1] = (0..5).map(|_| row[1].clone()).join(",");


        Row {
            row: row[0].chars().map(|x| Spring::from(x)).collect_vec(),
            //blocks: row[0].split('.').filter(|x| !x.is_empty()).collect_vec(),
            splits: Splits {
                changed_since_last_split: false,
                splits: row[1]
                    .split(',')
                    .map(|x| x.parse::<u16>().unwrap())
                    .collect_vec(),
            },
        }
    }

    fn process(&self) -> usize {
        let mut vec: Vec<(Splits, usize, bool)> = vec![(self.splits.clone(), 1, false)];
        
        for spring in &self.row {
            for x in vec.iter_mut() {
                x.2 = false;
            }
            match spring {
                Spring::Operational => for x in (0..vec.len()).rev() {
                    if !vec[x].0.split() {
                        vec.remove(x);
                    }
                },
                Spring::Damaged => {
                    for x in (0..vec.len()).rev() {
                        if !vec[x].0.add_damaged() {
                            vec.remove(x);
                        }
                    }
                }
                Spring::Unknown => {
                    for x in (0..vec.len()).rev() {
                        vec[x].2 = true;

                        let mut new_splits = vec[x].clone();

                        if new_splits.0.add_damaged() {
                            if let Some(split) = vec.iter_mut().find(|x| x.0 == new_splits.0 && x.2 == true) {
                                split.1 += new_splits.1;
                            }
                            else {
                                vec.push(new_splits);
                            }
                        }
                        if !vec[x].0.split() {
                            vec.remove(x);
                        }
                    }
                }
            }
            //println!("\n\n{:?} {:?}", vec, spring);
        }
        
        //println!("Final: \n\n{:?}", vec);
        //println!("Finished!");
        return vec.iter().filter(|x| x.0.is_complete()).map(|x| x.1).sum();
    }
}

fn main() {
    // Gets input from file
    let full_input;

    let inputs = match fs::read_to_string("input.txt") {
        Ok(x) => {
            full_input = x;
            full_input
                .split("\n")
                .map(|x| Row::new(x.trim().split_ascii_whitespace().map(|x| x.to_owned()).collect_vec()))
                .collect_vec()
        }
        Err(x) => panic!("Error in reading, {}", x),
    };
    println!("{}", inputs.par_iter().map(|x| x.process()).sum::<usize>());
}
