use std::{cmp::max, collections::VecDeque};

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
    let mut lines_vec: Vec<&str> = input.lines().collect();
    let _row_length = find_length(&lines_vec);
    let _row_count = lines_vec.len();
    //println!("row_length: {row_length}, row_count {row_count}");
    
    let top_row = lines_vec[0];
    let start = top_row.find('.').unwrap();

    let moves = lines_vec[lines_vec.len() - 1];
    let moves_queue = read_moves(moves);
    //for m in &moves_queue {
        //println!("Move: {m}");
    //}

    lines_vec.remove(lines_vec.len()-1);
    lines_vec.remove(lines_vec.len()-1);

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

fn new_position((x, y): (i32, i32), direction: &char) -> (i32, i32) {
    match direction {
        '<'=> (x - 1, y),
        '^'=> (x    , y - 1),
        '>'=> (x + 1, y),
        'v'=> (x    , y + 1),
        _  => (x    , y),
    }
}

fn get(lines_vec: &Vec<&str>, (x,y): (i32, i32)) -> char {
    lines_vec[y as usize].as_bytes()[x as usize] as char
}

// let cached_min_max_h: Vec<(usize, usize)>
// let cached_min_max_w: Vec<(usize, usize)>

fn get_min_height(lines_vec: &Vec<&str>, x: usize) -> i32 {
    for id in 0..lines_vec.len() {
        if lines_vec[id].as_bytes()[x] as char != ' ' {
            return id as i32
        }
    }
    0
}

fn get_max_height(lines_vec: &Vec<&str>, x: usize) -> i32 {
    let mut id = lines_vec.len() as i32 -1;
    while id >= 0 {
        if lines_vec[id as usize].len() <= x {
            id -= 1;
            continue
        }
        if lines_vec[id as usize].as_bytes()[x] as char != ' ' {
            return id
        }
        id -= 1;
    }
    lines_vec.len() as i32 -1
}

fn get_min_width(lines_vec: &Vec<&str>, y: usize) -> i32 {
    for id in 0..lines_vec[y].len() {
        if lines_vec[y].as_bytes()[id] as char != ' ' {
            return id as i32
        }
    }
    0
}

fn get_max_width(lines_vec: &Vec<&str>, y: usize) -> i32 {
    let mut id = lines_vec[y].len() as i32 - 1;
    while id >= 0 {
        if lines_vec[y].as_bytes()[id as usize] as char != ' ' {
            return id
        }
        id -= 1;
    }
    lines_vec[y].len() as i32 -1
}

//    A B
//  N|    |C
//  M| |ED
//K|L  |F  
//J| |HG
//  I
fn new_position_on_map_part2(
    (x, y): (i32, i32), 
    direction: &char, 
    lines_vec: &Vec<&str>
) -> ((i32, i32), char) { // TODO and sign
    
    let mut possible_pos: (i32, i32);
    TODO below there is sth wrong :/
    let mut possible_dir = *direction;
    if x >= 50 && x < 100 && y == 0 && *direction == '^' {
        // A -> J
        possible_pos = (0,x+100);
        possible_dir = '>';
    }
    else if x >= 100 && x < 150 && y == 0 && *direction == '^' {
        // B -> I
        possible_pos = (x-100,199);
        possible_dir = '^';
    }
    else if x == 149 && y >= 0 && y < 50 && *direction == '>'{
        // C -> F
        possible_pos = (99,149-y); // TO
        possible_dir = '<';
    }
    else if x >= 100 && x < 150 && y == 49 && *direction == 'v'{
        // D -> E
        possible_pos = (99, x - 100);
        possible_dir = '<';
    }
    else if x == 99 && y >= 50 && y < 100 && *direction == '>'{
        // E -> D
        possible_pos = (x+50,49);
        possible_dir = '^';
    }
    else if x == 99 && y >= 100 && y < 150 && *direction == '>'{
        // F -> C
        possible_pos = (149, 149 - y); //
        possible_dir = '<';
    }
    else if x >= 50 && x < 100 && y == 149 && *direction == 'v'{
        // G -> H
        possible_pos = (49, x+100);
        possible_dir = '<';
    }
    else if x == 49 && y >= 150 && y < 200 && *direction == '>'{
        // H -> G
        possible_pos = (y-100,149);
        possible_dir = '^';
    }
    else if x >= 0 && x < 50 && y == 199 && *direction == 'v'{
        // I -> B
        possible_pos = (x+100,0);
        possible_dir = 'v';
    }
    else if x == 0 && y >= 150 && y < 200 && *direction == '<'{
        // J -> A
        possible_pos = (y-100,0);
        possible_dir = 'v';
    }
    else if x == 0 && y >= 100 && y < 150 && *direction == '<'{
        // K -> N
        possible_pos = (50,149-y);
        possible_dir = '>';
    }
    else if x >= 0 && x < 50 && y == 100 && *direction == '^'{
        // L -> M
        possible_pos = (50,x+50);
        possible_dir = '>';
    }
    else if x == 50 && y >= 50 && y < 100 && *direction == '<'{
        // M -> L
        possible_pos = (y-50,100);
        possible_dir = 'v';
    }
    else if x == 50 && y >= 0 && y < 50 && *direction == '<' {
        // N -> K
        possible_pos = (0,149-y);
        possible_dir = '>';
    }
    else {
        // inside
        possible_pos = new_position((x as i32, y as i32), &direction);
    }
    let new_char = get(&lines_vec, possible_pos);
    if new_char != '.' {
        // position not changed
        ((x,y), *direction)
    }
    else {
        return (possible_pos, possible_dir)
    }
    //let mut result = (x, y);
    //let possible_pos = new_position((x as i32, y as i32), &direction);
    // A
}

fn new_position_on_map(
    (x, y): (i32, i32), 
    direction: &char, 
    lines_vec: &Vec<&str>
) -> (i32, i32) {
    let mut result = (x, y);
    let possible_pos = new_position((x as i32, y as i32), &direction);
    let min_height = get_min_height(lines_vec, x as usize);
    let max_height = get_max_height(lines_vec, x as usize);
    let min_width = get_min_width(lines_vec, y as usize);
    let max_width = get_max_width(lines_vec, y as usize);

    // TODO "-1" a co z \r\n ?????????
    
    if possible_pos.0 < min_width {
        // check if # at the end, if not then move
        let new_char = get(&lines_vec, (max_width, y));
        if new_char != '.' {
            // position not changed
        }
        else {
            result.0 = max_width;
        }
    }
    else if possible_pos.0 > max_width {
        // check if # at the start, if not then move
        let new_char = get(&lines_vec, (min_width, y));
        if new_char != '.' {
            // position not changed
        }
        else {
            result.0 = min_width;
        }
    }
    else if possible_pos.1 < min_height {
        // check if # at the end, if not then move
        let new_char = get(&lines_vec, (x, max_height));
        if new_char != '.' {
            // position not changed
        }
        else {
            result.1 = max_height;
        }
    }
    else if possible_pos.1 > max_height {
        // check if # at the start, if not then move
        let new_char = get(&lines_vec, (x, min_height));
        if new_char != '.' {
            // position not changed
        }
        else {
            result.1 = min_height;
        }
    }
    else {
        let new_char = get(&lines_vec, possible_pos);
        if new_char != '.' {
            // position not changed
        }
        else {
            result = possible_pos;
        }
    }
    result
}

fn get_num_from_direction(direction: &char) -> i32 {
    match direction {
        '<'=> 2,
        '^'=> 3,
        '>'=> 0,
        'v'=> 1,
        _  => 10,
    }
}

fn solve_1(input: &str) -> i32 {
    let (lines_vec, start, mut moves_queue) = read_map_and_moves(input);

    //< ^ > v
    let mut current_direction = '>';
    let mut position = (start as i32, 0);
    while ! moves_queue.is_empty() {
        let current = moves_queue.pop_front().unwrap();
        let steps = current.parse::<i32>();
        if steps.is_err() {
            let new_direction = change_direction(&current_direction, &(current.as_bytes()[0] as char));
            //println!("Change: {current} Old Dir {current_direction}, new Dir {new_direction}");
            current_direction = new_direction;
        }
        else {
            for _ in 0..steps.unwrap() {
                //let possible_pos = new_position_on_map(position, &current_direction, &lines_vec);
                (position, current_direction)= new_position_on_map_part2(position, &current_direction, &lines_vec);
                //println!("OLD P {}x{}; NEW P {}x{}", position.0, position.1, possible_pos.0, possible_pos.1);
                //position = possible_pos;
            }
        }
    }


    1000 * (position.1 + 1) + 4 * (position.0 + 1) + get_num_from_direction(&current_direction)
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
