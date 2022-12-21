use dynamic_matrix::{DynamicMatrix};
use sscanf::sscanf;
use std::cmp;

#[derive(Eq, PartialEq, Clone, Copy)]
struct Point{
    pub x: i32,
    pub y: i32,
}
#[allow(non_snake_case)]
struct Line{
    pub A: Point,
    pub B: Point,
}

impl Point {
    pub fn new(v_x: i32, v_y: i32) -> Self {
        Self { x: v_x, y: v_y }
    }
    // pub fn from_pair(p: (i32, i32)) -> Self {
    //     Self { x: p.0, y: p.1 }
    // }
    pub fn xu(&self) -> usize {
        self.x as usize
    }
    pub fn yu(&self) -> usize {
        self.y as usize
    }
    pub fn pair(&self) -> (usize, usize) {
        (self.xu(), self.yu())
    }
    pub fn increase(&self, x: i32, y: i32) -> Point {
        Point::new(self.x+x, self.y+y)
    }
}

// impl Line {
//     pub fn new(a: Point, b: Point) -> Self {
//         Self { A: a, B: b }
//     }
// }

struct RockLines{
    pub lines: Vec<Line>,
}

//TODO real parsing
fn parse_input(input: &str) -> RockLines {
    let mut result: RockLines = RockLines{lines: Vec::new()};
    let lines = input.lines();
    for line in lines {
        println!("LINE: {line}");

        let pairs = line.split(" -> ");
        let mut previous: Option<Point> = Option::None;
        for p in pairs {
            let parsed = sscanf!(p, "{},{}",i32, i32);
            let (x, y) = parsed.unwrap();
            let current_pt = Point::new(x, y);
            if previous.is_some() {
                let l = Line{A: previous.unwrap(), B: current_pt};
                previous = Some(current_pt);
                println!("A:{}x{} B: {}x{} ", l.A.x, l.A.y, l.B.x, l.B.y);
                result.lines.push(l);
            }
            else {
                previous = Some(current_pt);
            }
        }
    }
    result
}

// left, top, right, bottom
fn find_boundaries(rock_lines: &RockLines) -> (i32, i32, i32, i32) {
    // TODO real find
   // (494, 0, 503, 9);
    let mut left_max = i32::MAX;
    let mut right_max = i32::MIN;
    let mut bottom_max = 0;
    for rl in &rock_lines.lines {
        left_max = cmp::min(rl.A.x, left_max);
        left_max = cmp::min(rl.B.x, left_max);
        right_max = cmp::max(right_max, rl.A.x);
        right_max = cmp::max(right_max, rl.B.x);
        bottom_max = cmp::max(rl.A.y, bottom_max);
        bottom_max = cmp::max(rl.B.y, bottom_max);
    }
    (left_max, 0, right_max, bottom_max)
}

fn draw_rocks(mat: &mut DynamicMatrix<char>, rock_lines: &RockLines, shift: i32) {
    for line in &rock_lines.lines {
        let mut current = line.A;
        loop {
            // draw '#'
            mat[((current.x - shift) as usize, current.y as usize)] = '#';
            if current == line.B {
                break;
            }
            if current.x == line.B.x {
                if current.y < line.B.y {
                    current.y += 1;
                }
                else {
                    current.y -= 1;
                }
            }
            else {
                if current.x < line.B.x {
                    current.x += 1;
                }
                else {
                    current.x -= 1;
                }
            }
        }
    }
    let (width, height) = mat.shape();
    for x in 0..width {
        mat[(x, height-1)] = '#';
    }
}

fn is_in(mat: &mut DynamicMatrix<char>, pt: &Point) -> bool {
    let (width, height) = mat.shape();
    if pt.x < 0 || pt.xu() >= width {
        return false
    }
    if pt.yu() >= height {
        return false
    }
    true
}

// fn will_fall(mat: &mut DynamicMatrix<char>, pt: &Point) {
//     let (width, height) = mat.shape();
//     if ()
// }

fn is_empty(mat: &mut DynamicMatrix<char>, pt: &Point) -> bool {
    let (width, height) = mat.shape();
    if pt.x < 0 || pt.xu() > width || pt.yu() > height {
        return true
    }
    mat[pt.pair()] == '.'
}

fn mark_sand(mat: &mut DynamicMatrix<char>, start: Point, falls: &mut bool) -> bool {
    // if *falls {
    //     println!("fall detected {} {}?", start.x, start.y);
    //     *falls = true;
    //     return false
    // }
    //let (_, _) = mat.shape();
    //println!("mark_sand {:#?}", start.pair());
    let down = start.increase(0, 1);
    let left = start.increase(-1, 1);
    let right = start.increase(1, 1);
    // if start.yu() >= height-1 && is_empty(mat, &start) {
    //     println!("START will fall {} {}?", start.x, start.y);
    //     *falls = true;
    //     return false
    // }
    // if down.yu() >= height-1 && is_empty(mat, &down){
    //     println!("DOWN will fall {} {}?", down.x, down.y);
    //     *falls = true;
    //     return false
    // }
    // if left.yu() >= height-1 && is_empty(mat, &left){
    //     println!("LEFT will fall {} {}?", left.x, left.y);
    //     *falls = true;
    //     return false
    // }
    // if right.yu() >= height-1 && is_empty(mat, &right){
    //     println!("RIGT will fall {} {}?", right.x, right.y);
    //     *falls = true;
    //     return false
    // }

    if is_in(mat, &down) && is_empty(mat, &down) && mark_sand(mat, down, falls) {
        return true
    }
    else if is_in(mat, &left) && is_empty(mat, &left) && mark_sand(mat, left, falls) {
        return true
    }
    else if is_in(mat, &right) && is_empty(mat, &right) && mark_sand(mat, right, falls) {
        return true
    }

    // if *falls {
    //     return false
    // }

    if is_in(mat, &start) && is_empty(mat, &start) {
        mat[start.pair()] = 'o';
        return true
    }
    return false
}

fn solve_1(input: &str) -> usize {
    let rock_lines = parse_input(input);
    
    let (left, top, right, bottom) = find_boundaries(&rock_lines);
    println!("L {left} T {top} R{right} B {bottom}");
    let width = (right - left+500).abs() as usize;
    let height = (bottom - top+2).abs() as usize;
    println!("W {width} H {height}");
    //top-left is 0,0
    // right - left = width
    // bottom - top = hight
    let mut mat: DynamicMatrix<char> = DynamicMatrix::with_capacity((width+1, height+1));
    for _ in 0..width+1 {
         let err = mat.push_row(vec!['.'; height+1]);
         if err.is_err() {
            println!("error: {:?}", err.unwrap());
            break;
         }
    }
    let (x,y) = mat.shape();
    println!("width {width}, height {height} mat size {x} {y}");
    // mat fields set with 0

    // przesuniecie o left
    draw_rocks(&mut mat, &rock_lines, left-250);

    // draw start
    let start = Point{x: 750 - left, y: 0};
    mat[start.pair()] = 'S';

    let mut count_sand = 0;
    loop {
        let mut falls = false;
        if mark_sand(&mut mat, start, &mut falls) {
            //if falls {
            //    break;
            //}
            count_sand += 1;
        }
        else {
            break;
        }
    //     for col in 0..mat.cols() {
    //         for row in 0..mat.rows() {
    //             print!("{}", mat.get((row, col)).unwrap());
    //         }
    //         println!("");
    //    }
    }
    for col in 0..mat.cols() {
        for row in 0..mat.rows() {
            print!("{}", mat.get((row, col)).unwrap());
        }
        println!("");
   }
    count_sand
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
