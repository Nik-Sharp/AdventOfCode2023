use core::panic;
use std::io;

// Note this specifically solves part two, I refactored my code after I solved part 1 to solve part 2
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
                    // inputs.push(str.parse::<i8>()); for ints
                    inputs.push(str);
                }
            }
            Err(error) => {panic!("Error in reading, {}", error)},
        }
    }
}
