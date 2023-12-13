use core::panic;
use std::{fs, collections::HashSet, cmp::Ordering};

use itertools::Itertools;

extern crate rayon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
/* */
#[derive(Debug)]
struct Row<'row> {
    pub row : &'row str,
    pub question_indexes : Vec<usize>,
    //pub blocks: Vec<&'row str>,
    pub splits: Vec<u16>,
}

impl<'row> Row<'row> {
    fn new(row: Vec<&str>) -> Row {
        Row {
            row : row[0],
            question_indexes : row[0].chars().enumerate().filter(|x| x.1 == '?').map(|x| x.0).collect_vec(),
            //blocks: row[0].split('.').filter(|x| !x.is_empty()).collect_vec(),
            splits: row[1]
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect_vec(),
        }
    }
    fn check_with_vec(&self, is_damaged : &Vec<bool>)  -> bool{
        let changed = self.question_indexes.iter().enumerate().filter(|x| is_damaged[x.0]).map(|x| x.1).collect::<HashSet<_>>();
        let mut groups : Vec<u16> = vec![];
        let mut count = 0;

        for (i, x) in self.row.chars().enumerate() {
            if x == '#' || changed.contains(&i) {
                count += 1;
                continue;
            }

            if count > 0 {
                groups.push(count);
            }
            count = 0;
        }

        if count > 0 {
            groups.push(count);
        }

        groups.cmp(&self.splits) == Ordering::Equal
    }
    fn bruteforce(&self, is_damaged: Vec<bool>) -> u16 {
        

        if is_damaged.len() == self.question_indexes.len() {
            return self.check_with_vec(&is_damaged) as u16;
        }

        let mut yes = is_damaged.clone();
        yes.push(true);
        let mut no = is_damaged.clone();
        no.push(false);

        return self.bruteforce(yes) + self.bruteforce(no);


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
                .map(|x| Row::new(x.trim().split_ascii_whitespace().collect_vec()))
                .collect_vec()
        }
        Err(x) => panic!("Error in reading, {}", x),
    };
    
    println!("{}", inputs.par_iter().map(|x| x.bruteforce(vec![])).sum::<u16>());
}
