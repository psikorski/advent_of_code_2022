use sscanf::sscanf;

struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Point {
    fn from_tuple((a,b,c): (i32,i32,i32)) -> Self {
        Point{x: a, y: b, z: c}
    }
}

fn read_lines(input: &str) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let parsed = sscanf!(line, "{},{},{}", i32, i32, i32);
        result.push(Point::from_tuple(parsed.unwrap()));
    }
    result
}

fn solve_1(input: &str) -> usize {
    let points = read_lines(input);
    println!("Points {}", points.len()) ;
    // TODO solve
0
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
