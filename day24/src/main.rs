use dynamic_matrix::DynamicMatrix;
use pathfinding::prelude::astar;

type Ground = DynamicMatrix<char>;

// TODO print could be moved to common utils
#[allow(dead_code)]
fn print_matrix(ground: &Ground) {
    let shape = ground.shape();
    for y in 0..shape.0 {
        print!("{y} ");
        for x in 0..shape.1 {
            print!("{}", ground[(y, x)]);
        }
        print!("\n");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub round: i32,
}

impl Position {
    pub fn new(v_x: i32, v_y: i32, v_r: i32) -> Self {
        Self { x: v_x, y: v_y, round: v_r }
    }
    pub fn def() -> Self {
        Position::new(0,0, 0)
    }
    pub fn from_pair(pair: (usize, usize)) -> Self {
        Position::new(pair.1 as i32, pair.0 as i32, 0) // exchanged .0 with .1
    }
    pub fn pair(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
    pub fn ux(&self) -> usize {
        self.x as usize
    }
    pub fn uy(&self) -> usize {
        self.y as usize
    }
    pub fn distance(&self, other: &Position) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}

pub fn is_dir_move_hor(dir: char) -> bool {
    match dir {
        '<' => true,
        '>' => true,
        _ => false,
    }
}

pub fn is_dir_move_ver(dir: char) -> bool {
    match dir {
        '^' => true,
        'v' => true,
        _ => false,
    }
}

pub fn dir_to_move(dir: char) -> i32 {
    match dir {
        '<' => -1,
        '>' => 1,
        '^' => -1,
        'v' => 1,
        _ => 0,
    }
}

pub fn possible_next(position: &Position, ground: Ground) -> Vec<(Position, i32)> {
    //println!("possible_next for ({}x{}) round {}", position.ux(), position.uy(), position.round);
    let mut result: Vec<(Position, i32)> = Vec::new();
    // same place
    if ground[position.pair()] == '.' {
        result.push((Position::new(position.x, position.y, position.round+1), 1));
    }
    // top
    if position.uy() > 0 && ground[(position.uy()-1, position.ux())] == '.' {
        result.push((Position::new(position.x, position.y-1, position.round+1), 1));
    }
    // right
    if ground[(position.uy(), position.ux()+1)] == '.' {
        result.push((Position::new(position.x+1, position.y, position.round+1), 1));
    }
    // bottom
    if position.uy() < ground.shape().0 - 1 && ground[(position.uy()+1, position.ux())] == '.' {
        result.push((Position::new(position.x, position.y+1, position.round+1), 1));
    }
    // left
    if ground[(position.uy(), position.ux()-1)] == '.' {
        result.push((Position::new(position.x-1, position.y, position.round+1), 1));
    }
    result
}

pub fn is_success(position: &Position, end: &Position) -> bool {
    //println!("(is_success), round", )
    if position.x == end.x && position.y == end.y {
        return true
    }
    return false
}

pub fn count_new_pos(curr: i32, to_move: i32, len: i32) -> usize {
    let mod_move = to_move % len;
    let mut new_curr = curr + mod_move;
    if new_curr < 1 {
        new_curr = len + curr + mod_move;
    }
    else if new_curr > len {
        new_curr = curr + mod_move - len;
    }
    new_curr as usize
}

pub fn generate_next(ground: &Ground, rounds: i32) -> Ground { //(Ground, Vec<Position>) {
    //println!("(generate_next) round {rounds}");
    let shape = ground.shape();
    let x_in = shape.1 as i32 - 2;
    let y_in = shape.0 as i32 - 2;
    let mut next_valley = ground.clone();
    for y in 1..shape.0-1 {
        for x in 1..shape.1-1 {
            next_valley[(y,x)] = '.';
        }
    }

    for y in 1..shape.0-1 {
        for x in 1..shape.1-1 {
            let current = ground[(y,x)];
            if current == '.' {
                continue;
            }
            else if is_dir_move_hor(current) {
                // x_in -2 czy -1 ?
                let new_id = count_new_pos(x as i32, rounds * dir_to_move(current), x_in);
                //println!("(generate_next) hor({x}x{y}), new_id{new_id}, rounds {rounds}, char {current}");
                next_valley[(y,new_id)] = current;

            }
            else if is_dir_move_ver(current) {
                // x_in -2 czy -1 ?
                let new_id = count_new_pos(y as i32, rounds * dir_to_move(current), y_in);
                //println!("(generate_next) ver({x}x{y}), new_id{new_id}, rounds {rounds}, char {current}");
                next_valley[(new_id, x)] = current;
            }
        }
    }
    next_valley
}

// move to another file TODO
fn read_matrix_from_input(input: &str) -> Ground {
    let lines_vec: Vec<&str> = input.lines().collect();
    let height = lines_vec.len();
    let width = lines_vec[0].len();
    // println!("To create Matrix({}x{})", width, height);
    let mut result = DynamicMatrix::with_capacity((height, width));
    
    for line in lines_vec {
        let chars: Vec<char> = line.chars().map(|c| c as char).collect();
        result.push_row(chars).unwrap();
    }

    println!("Matrix({}x{})", result.cols(), result.rows());
    result
}

fn find_start(_ground: &Ground) -> Position {
    let result = Position::new(1,0, 0);
    println!("START ({}x{})", result.x, result.y);
    result
}

fn find_end(ground: &Ground) -> Position {
    let (height, width) = ground.shape();
    let result = Position::new(width as i32 - 2,height as i32 -1, 0);
    println!("FINISH ({}x{})", result.x, result.y);
    result
}

fn solve_1(input: &str) -> i32 {
    let ground = read_matrix_from_input(input);
    let mut start = find_start(&ground);
    let mut finish = find_end(&ground);
    print_matrix(&ground);
    let result = astar(
        &start,
        |p| possible_next(p, generate_next(&ground, p.round + 1)),
        |p| p.distance(&finish),
                   |p| is_success(p, &finish)
    );

    let minutes_finish = result.unwrap().1;
    // println!("MINUTES TO FINISH: {minutes_finish}");
    //let ground_finish = generate_next(&ground, minutes_finish);
    //print_matrix(&ground_finish);
    finish.round = minutes_finish;
    start.round = 0;
    let result_to_start = astar(
        &finish,
        |p| possible_next(p, generate_next(&ground, p.round + 1)),
        |p| p.distance(&start),
                   |p| is_success(p, &start)
    );
    let minutes_to_start = result_to_start.unwrap().1;
    println!("MINUTES TO START: {minutes_to_start}");
    //let ground_finish_start = generate_next(&ground, minutes_finish + minutes_to_start);

    finish.round = 0;
    start.round = minutes_finish + minutes_to_start;    

    let result_to_finish_start_finish = astar(
        &start,
        |p| possible_next(p, generate_next(&ground, p.round + 1)),
        |p| p.distance(&finish),
                   |p| is_success(p, &finish)
    );
    let minutes_to_finish_start_finish = result_to_finish_start_finish.unwrap().1;
    println!("MINUTES TO FINISH-START-FINISH: {minutes_to_finish_start_finish}");



    minutes_to_finish_start_finish + minutes_to_start + minutes_finish
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
