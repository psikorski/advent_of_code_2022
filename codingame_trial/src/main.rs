//use std::io;


use std::fs::File;
use std::io::prelude::*;

fn read_from_file(name: String) -> String {
 let mut file = File::open(name)
  .expect("File not found");
  let mut data = String::new();
 file.read_to_string(&mut data)
  .expect("Error while reading file");
// println!("{}", data);
 data
}



macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn get_card_value(letter: char) ->i32 {
    match letter {
        'K' | 'Q' | 'J' | 'T' => 10,
        'A' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0
    }
}

fn get_card_id(letter: char) ->i32 {
    match letter {
        'K' | 'Q' | 'J' | 'T' => 0,
        'A' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0 // TODO can't happen? maybe -1 ?
    }
}

fn filter_input() {
    //TODO
}

fn solve(stream_of_consciousness: String, bust_threshold: i32) ->i32 {
    let result = 67;
   
 
    let next_possible_cards: [i32; 10] = [16, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    
    // for
    let index_of_first_dot = stream_of_consciousness.iter().position(|&&x| x == '.');
    //if index_of_first_dot == None // end
    

    for card in stream_of_consciousness.chars() {

    }

    result
}



/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let input_line = read_from_file("input_1.txt".to_owned());
    //let mut input_line = String::new();
    //io::stdin().read_line(&mut input_line).unwrap();
    let stream_of_consciousness = input_line.trim_matches('\n').to_string();
    let input_line = read_from_file("input_2.txt".to_owned());
    //let mut input_line = String::new();
    //io::stdin().read_line(&mut input_line).unwrap();
    let bust_threshold = parse_input!(input_line, i32);

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    eprintln!("stream_of_consciousness {}", stream_of_consciousness);
    eprintln!("percentageChance {}", bust_threshold);

    let result = solve();
    println!("{}%", result);
}
