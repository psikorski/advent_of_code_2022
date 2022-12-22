use std::{cmp::max, collections::VecDeque};
use sscanf::sscanf;
use dynamic_matrix::{DynamicMatrix};

fn find_length(lines_vec: &Vec<&str>) -> usize {
    let mut result = 0;
    for id in 0..lines_vec.len()-1 {
        result = max(lines_vec[id].len(), result);
    }

    result
}

// fn find_rows_count(lines_vec: &Vec<&str>) -> usize {
//     let mut max
//     lines_vec.iter().map(f)
// }

fn read_moves(input: &str) -> VecDeque<String> {
    let mut moves_ch = input.chars();
    let mut c = moves_ch.next();
    let mut moves_queue = VecDeque::new();
    let mut current: String = String::new();
    while c.is_some() {
        let c_un = c.unwrap();
        if c_un.is_digit(10) {
            current = current + &c_un.to_string(); 
        }
        else {
            // c_un is alphabet
            if ! current.is_empty() {
                moves_queue.push_back(current.clone()); // TODO push struct
                current.clear();
            }
            moves_queue.push_back(c_un.to_string()); // TODO push struct
        }
        c = moves_ch.next();
    }
    if ! current.is_empty() {
        moves_queue.push_back(current.clone()); // TODO push struct
    }

    moves_queue
}

fn read_map_and_moves(input: &str) -> (Vec<&str>, usize, VecDeque<String>) {
    let lines_vec: Vec<&str> = input.lines().collect();
    let row_length = find_length(&lines_vec);
    let row_count = lines_vec.len();
    println!("row_length: {row_length}, row_count {row_count}");
    
    let top_row = lines_vec[0];
    let start = top_row.find('.').unwrap();

    let moves = lines_vec[lines_vec.len() - 1];
    let moves_queue = read_moves(moves);
    for m in &moves_queue {
        println!("Move: {m}");
    }

    (lines_vec, start, moves_queue)
}

fn clockwise(direction: &char) -> char {
    match direction {
        '<'=> '^',
        '^'=> '>',
        '>'=> 'v',
        'v'=> '<',
        _  => *direction,
    }
}

fn anti_clockwise(direction: &char) -> char {
    match direction {
        '<'=> 'v',
        '^'=> '<',
        '>'=> '^',
        'v'=> '>',
        _  => *direction,
    }
}

fn change_direction(current_direction: &char, turn: &char) -> char {
    //< ^ > v
    if *turn == 'R' {
        clockwise(current_direction)
    }
    else {
        anti_clockwise(current_direction)
    }
}

fn new_position((x, y): (usize, usize), direction: &char) -> (i32, i32) {
    match direction {
        '<'=> ((x as i32) - 1, y as i32),
        '^'=> (x as i32, (y as i32) - 1),
        '>'=> ((x as i32) + 1, y as i32),
        'v'=> (x as i32, (y as i32) + 1),
        _  => (x as i32,y as i32),
    }
}

fn get(lines_vec: &Vec<&str>, (x,y): (usize, usize)) -> char {
    lines_vec[y].as_bytes()[x] as char
}

fn solve_1(input: &str) -> i64 {
    let (lines_vec, start, mut moves_queue) = read_map_and_moves(input);

    //< ^ > v
    let mut current_direction = '>';
    let mut position = (start, 0 as usize);
    while ! moves_queue.is_empty() {
        let current = moves_queue.pop_front().unwrap();
        let steps = current.parse::<i32>();
        if steps.is_err() {
            let new_direction = change_direction(&current_direction, &(current.as_bytes()[0] as char));
            println!("Change: {current} Old Dir {current_direction}, new Dir {new_direction}");
            current_direction = new_direction;
        }
        else {
            for _ in 0..steps.unwrap() {
                let new_position = new_position(position, &current_direction);
                if new_position.0 < 0 {
                    // check if # at the end, if not then move
                    let new_char = get(&lines_vec, (lines_vec[position.1].len()-1, position.1));
                    if new_char == '#' {
                        // position not changed
                    }
                    else {
                        position.0 = lines_vec[position.1].len()-1;
                    }
                }
                else if new_position.0 > lines_vec[new_position.1].len()-1 {
                    // check if # at the start, if not then move
                }
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
