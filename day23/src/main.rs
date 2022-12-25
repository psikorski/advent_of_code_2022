use std::collections::{VecDeque, HashMap};
use dynamic_matrix::DynamicMatrix;
use std::i32::MAX;
use std::cmp::min;
use std::cmp::max;

type Ground = DynamicMatrix<char>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum WorldDirections {
    North,
    South,
    West,
    East,
}

// move to another file TODO
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

fn position_from_dir(direction: WorldDirections, position: &Position) -> Position {
    match direction {
        WorldDirections::North => Position::new(position.x, position.y-1),
        WorldDirections::South => Position::new(position.x, position.y+1),
        WorldDirections::West => Position::new(position.x-1, position.y),
        WorldDirections::East => Position::new(position.x+1, position.y),
    }
}

fn dir_pos_mapping(direction: WorldDirections, pos: &Position) -> VecDeque<Position> {
    let mut result: VecDeque<Position> = VecDeque::new();
    if direction == WorldDirections::North {
        result.push_back(Position::new(pos.x, pos.y-1));
        result.push_back(Position::new(pos.x+1, pos.y-1));
        result.push_back(Position::new(pos.x-1, pos.y-1));
    }
    else if direction == WorldDirections::South {
        result.push_back(Position::new(pos.x, pos.y+1));
        result.push_back(Position::new(pos.x+1, pos.y+1));
        result.push_back(Position::new(pos.x-1, pos.y+1));
    }
    else if direction == WorldDirections::West {
        result.push_back(Position::new(pos.x-1, pos.y));
        result.push_back(Position::new(pos.x-1, pos.y-1));
        result.push_back(Position::new(pos.x-1, pos.y+1));
    }
    else if direction == WorldDirections::East {
        result.push_back(Position::new(pos.x+1, pos.y));
        result.push_back(Position::new(pos.x+1, pos.y-1));
        result.push_back(Position::new(pos.x+1, pos.y+1));
    }
    result
}

struct Elf{
    pub pos: Position,
    pub moves: VecDeque<WorldDirections>,
}

fn free_pos(ground: &Ground, pos: &Position) -> bool {
    let shape = ground.shape();
    if pos.y >= 0 && pos.x >= 0 && pos.y < shape.0 as i32 && pos.x < shape.1 as i32 {
        if *ground.get(pos.pair()).unwrap() == '#' {
            return false
        }
        return true
    }
    false
    //true
}

impl Elf {
    pub fn new(a_pos: Position) -> Self {
        Self { 
            pos: a_pos, 
            moves: VecDeque::from([WorldDirections::North, WorldDirections::South, WorldDirections::West, WorldDirections::East])
        }
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

        let shape = ground.shape();
        while ! to_check.is_empty() {
            let single = to_check.pop_front().unwrap();
            if single.y >= 0 && single.x >= 0 && single.y < shape.0 as i32 && single.x < shape.1 as i32 {
                if ! free_pos(ground, &single) {
                    return true
                }
            }
        }
        false
    }
    // give the first possible move
    pub fn possible_move(&self, ground: &Ground) -> Option<Position> {
        let mut to_check: VecDeque<Position>;

        let mut move_to: Option<Position> = Option::None;
        for m in &self.moves {
            move_to = Some(position_from_dir(*m, &self.pos));
            to_check = dir_pos_mapping(*m, &self.pos);
            
            while ! to_check.is_empty() {
                let single = to_check.pop_front().unwrap();
                if ! free_pos(ground, &single) {
                    move_to = Option::None;
                }
            }
            if move_to.is_some() {
                return move_to
            }
        }
        move_to
    }
    pub fn move_me(&mut self, ground: &mut Ground, position: &Position) {
        //let prev = ground[self.pos.pair()];
        //print!("\tMOVE_ME ({:?}) ->", self.pos);
        ground[self.pos.pair()] = '.';
        self.pos = *position;
        //print!("({:?})\n", self.pos);
        ground[self.pos.pair()] = '#';
    }

    pub fn flip_move(&mut self) {
        let flipped = self.moves.pop_front();
        self.moves.push_back(flipped.unwrap());
    }
}

// move to another file TODO
fn read_matrix_from_input(input: &str, extra: usize) -> Ground {
    // 10 rounds mean max +10 in each direction

    let lines_vec: Vec<&str> = input.lines().collect();
    let height = lines_vec.len();
    let width = lines_vec[0].len();
    // println!("To create Matrix({}x{})", width, height);
    let mut result = DynamicMatrix::with_capacity((height+2*extra, width+2*extra));
    
    for i in 0..extra {
        result.push_row(vec!['.'; width+2*extra]).unwrap();
    }
    for line in lines_vec {
        let mut empty_pre = vec!['.'; extra];
        let mut empty_post = vec!['.'; extra];
        let mut chars: Vec<char> = line.chars().map(|c| c as char).collect();

        empty_pre.append(&mut chars);
        empty_pre.append(&mut empty_post);
        result.push_row(empty_pre).unwrap();
    }

    for i in 0..extra {
        result.push_row(vec!['.'; width+2*extra]).unwrap();
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

fn smallest_rectangle(elves: &Vec<Elf>) -> i32 {
    let mut left = MAX;
    let mut top = MAX;
    let mut right = 0;
    let mut bottom = 0;
    for elf in elves {
        left = min(left, elf.pos.x);
        right = max(right, elf.pos.x);
        top = min(top, elf.pos.y);
        bottom = max(bottom, elf.pos.y);
    }
    println!("L: {left}, R: {right}, T: {top}, B: {bottom}");
    let smallest_rectangle = (right-left+1)*(bottom-top+1);
    println!("smallest_rectangle: {smallest_rectangle}");
    smallest_rectangle
}

// TODO print could be moved to common utils
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

fn solve_1(input: &str) -> i32 {
    let mut ground = read_matrix_from_input(input, 1000);
    let mut elves = read_elves(&ground);
    
    let mut rounds = 1;
    loop {
        //print_matrix(&ground);
        let mut all_moves: HashMap<Position, usize> = HashMap::new();
        // for every elf, get possible move
        for id in 0..elves.len() {
            let elf = &mut elves[id];
            if ! elf.needs_move(&ground) {
                //println!("ELF({:?}, {:#?}) NOT NEED", elf.pos, elf.moves.front().unwrap());
                elf.flip_move();
                continue;
            }
            let poss_move = elf.possible_move(&ground);
            if poss_move.is_some() { // andThen?
                let poss_move_un = poss_move.unwrap();
                //println!("ELF({:?}) HAS POTENTIAL MOVE({:?}) ", elf.pos, poss_move_un);
                let removed = all_moves.remove(&poss_move_un);
                if removed.is_none() {
                    all_moves.insert(poss_move_un, id);
                }
                else {
                    //println!("ELF({:?}) POTENTIAL MOVE REMOVED ({:?}) ", elf.pos, poss_move_un);
                }
            }
            else {
                //println!("ELF ({:?}) CANNOT MOVE ANYWHERE", elf.pos);
            }
            elf.flip_move();
        }
        if all_moves.is_empty() {
            break;
        }
        for one_move in &all_moves {
            //println!("ELF({:?}) MOVED({:?}) ", elves[*one_move.1].pos, one_move.0);
            elves[*one_move.1].move_me(&mut ground, one_move.0);
        }
        if rounds == 10 {
            //print the smallest rectangle... (part 1)
        }
        rounds += 1;
        println!("\nROUND = {rounds}");
    }
    println!("\nROUND = {rounds}");
    //print_matrix(&ground);
    println!("Elves count: {}", elves.len());
    smallest_rectangle(&elves) - elves.len() as i32
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
