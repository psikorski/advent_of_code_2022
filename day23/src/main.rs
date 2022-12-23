use std::collections::VecDeque;

use dynamic_matrix::DynamicMatrix;


type Ground = DynamicMatrix<char>;

// move to another file TODO
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// move to another file TODO
impl Position {
    pub fn new(v_x: i32, v_y: i32) -> Self {
        Self { x: v_x, y: v_y }
    }
    pub fn def() -> Self {
        Position::new(0,0)
    }
    pub fn from_pair(pair: (usize, usize)) -> Self {
        Position::new(pair.1 as i32, pair.0 as i32) // exchanged 0 with 1
    }
    pub fn pair(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}

enum WorldDirections {
    North,
    South,
    West,
    East,
}

struct Elf{
    pub pos: Position,
    pub moves: VecDeque<WorldDirections>,
    pub planned_move: Option<WorldDirections>,
}

fn free_pos(ground: &Ground, pos: &Position) -> bool {
    let shape = ground.shape();
    if pos.y >= 0 && pos.x >= 0 && pos.y < shape.0 && pos.x < shape.1 {
        if ground.get(pos.pair()).unwrap() == '#' {
            return false
        }
    }
    true
}

impl Elf {
    pub fn new(a_pos: Position) -> Self {
        Self { pos: a_pos, moves: VecDeque::new(), planned_move: Option::None }
    }

    // check if elf needs to move at all
    pub fn needs_move(&self, ground: &Ground) -> bool {
        let mut to_check: VecDeque<Position> = VecDeque::new();
        // N, NE, NW
        to_check.push_back(Position::new(self.pos.x, self.pos.y-1));
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y-1));
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y-1));
        // S, SE, SW
        to_check.push_back(Position::new(self.pos.x, self.pos.y+1));
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y+1));
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y+1));

        // W, NW, SW
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y));
        //to_check.push_back(Position::new(self.pos.x-1, self.pos.y-1)); // repeated
        //to_check.push_back(Position::new(self.pos.x-1, self.pos.y+1)); // repeated

        // E, NE, SE
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y));
        //to_check.push_back(Position::new(self.pos.x+1, self.pos.y-1)); // repeated
        //to_check.push_back(Position::new(self.pos.x+1, self.pos.y+1)); // repeated

        while ! to_check.is_empty() {
            let single = to_check.pop_front().unwrap();
            if ! free_pos(ground, &single) {
                return true
            }
        }
        false
    }

    // give the first possible move
    pub fn possible_move(&self, ground: &Ground) -> Option<WorldDirections> {
        let mut to_check: VecDeque<Position> = VecDeque::new();
        
        // N, NE, NW
        let mut move_to: Option<WorldDirections> = Some(WorldDirections::North);
        to_check.push_back(Position::new(self.pos.x, self.pos.y-1));
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y-1));
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y-1));
        while ! to_check.is_empty() {
            let single = to_check.pop_front().unwrap();
            if ! free_pos(ground, &single) {
                move_to = Option::None;
            }
        }
        if move_to.is_some() {
            return move_to
        }

        // S, SE, SW
        move_to = Some(WorldDirections::South);
        to_check.push_back(Position::new(self.pos.x, self.pos.y+1));
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y+1));
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y+1));
        while ! to_check.is_empty() {
            let single = to_check.pop_front().unwrap();
            if ! free_pos(ground, &single) {
                move_to = Option::None;
            }
        }
        if move_to.is_some() {
            return move_to
        }

        // W, NW, SW
        move_to = Some(WorldDirections::West);
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y));
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y-1)); // repeated
        to_check.push_back(Position::new(self.pos.x-1, self.pos.y+1)); // repeated
        while ! to_check.is_empty() {
            let single = to_check.pop_front().unwrap();
            if ! free_pos(ground, &single) {
                move_to = Option::None;
            }
        }
        if move_to.is_some() {
            return move_to
        }

        // E, NE, SE
        move_to = Some(WorldDirections::East);
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y));
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y-1)); // repeated
        to_check.push_back(Position::new(self.pos.x+1, self.pos.y+1)); // repeated
        while ! to_check.is_empty() {
            let single = to_check.pop_front().unwrap();
            if ! free_pos(ground, &single) {
                move_to = Option::None;
            }
        }
        move_to
    }
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

fn read_elves(ground: &Ground) -> Vec<Elf> {
    let mut result: Vec<Elf> = Vec::new();
    let shape = Position::from_pair(ground.shape());
    for x in 0..shape.x as usize {
        for y in 0..shape.y as usize {
            if *ground.get((y, x)).unwrap() == '#' {
                result.push(Elf::new(Position::new(x as i32, y as i32)));
            }
        }
    }
    result
}

fn single_round(ground: &mut Ground, elves: &mut Vec<Elf>) -> bool {
    println!()
}

fn solve_1(input: &str) -> i32 {
    let ground = read_matrix_from_input(input);
    let elves = read_elves(&ground);
    println!("Elves: {}", elves.len());
    0
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
