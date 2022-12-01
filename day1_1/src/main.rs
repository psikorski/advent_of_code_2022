//use std::io;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn solve(name: String) -> i32 {
    let mut biggest_callories = 0;
    let mut current_callories = 0;
    if let Ok(lines) = read_lines(name) {
        
        for line in lines {
            if let Ok(line_str) = line {
                if line_str.is_empty() { // TODO it is not empty line
                    println!("current_callories = {}, biggest_callories = {}", current_callories, biggest_callories);
                    if current_callories > biggest_callories {
                        biggest_callories = current_callories;
                    }
                    current_callories = 0;
                }
                else {
                    // if number is int, sum it to current_callories
                    let callorie = parse_input!(line_str, i32);
                    current_callories += callorie;
                    println!("new = {}, current_callories = {}, biggest_callories = {}", callorie, current_callories, biggest_callories);
                }
            }
        }
    }
    if current_callories > biggest_callories {
        biggest_callories = current_callories;
    }
    biggest_callories
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let result = solve("input_1.txt".to_owned());
    println!("{}", result);
}
