use dynamic_matrix::DynamicMatrix;

type Ground = DynamicMatrix<char>;

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

pub fn is_dir_move(dir: char) -> bool {
    match dir {
        '<' => true,
        '>' => true,
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

struct Valley {
    pub valley: Ground,
    pub e: Position,
}

impl Valley {
    pub fn generate_next(&self, rounds: i32) -> Ground { //(Ground, Vec<Position>) {
        let shape = self.valley.shape();
        let mut next_valley = DynamicMatrix::with_capacity(shape);

        for y in 1..shape.0-1 {
            for x in 1..shape.1-1 {
                let current = self.valley[(y,x)];
                if current == '.' {
                    continue;
                }
                else if current == '<' {
                    // use rounds to count how many left
                    let mut new_x = x - rounds;
                    //new_id %= (leni-1);
                    //if new_id < 0 {
                        //new_id = leni - 1 + new_id;
                    //}
                }
                    
                }
            }
        }
        next_valley
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
        let mut chars: Vec<char> = line.chars().map(|c| c as char).collect();
        result.push_row(chars).unwrap();
    }

    println!("Matrix({}x{})", result.cols(), result.rows());
    result
}

fn find_start(ground: &Ground) -> Position {
    let mut result = Position::new(1,0);
    result
}

fn find_end(ground: &Ground) -> Position {
    let (height, width) = ground.shape();
    let mut result = Position::new(width as i32 - 2,height as i32 -1);
    result
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
    let mut ground = read_matrix_from_input(input);
    let mut elves = read_elves(&ground);
    
    0
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
