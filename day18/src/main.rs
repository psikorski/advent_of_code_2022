use sscanf::sscanf;
use std::collections::HashSet;
use std::hash::{Hash};

#[derive(Eq, Hash, PartialEq)]
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

#[derive(Eq, PartialEq)]
struct Side {
    pub pts: HashSet<Point>,
}

fn generate_sides(point: &Point) -> Vec<Side> {
    // cube out of point
    // 6 sides
    let mut result: Vec<Side> = Vec::new();
    {
        // FRONT
        let mut s: Side = Side{pts: HashSet::new()};
        s.pts.insert(Point{x: point.x, y: point.y, z: point.z});
        s.pts.insert(Point{x: point.x, y: point.y+1, z: point.z});
        s.pts.insert(Point{x: point.x+1, y: point.y, z: point.z});
        s.pts.insert(Point{x: point.x+1, y: point.y+1, z: point.z});
        result.push(s);
    }
    {
        // BACK
        let mut s: Side = Side{pts: HashSet::new()};
        s.pts.insert(Point{x: point.x, y: point.y, z: point.z+1});
        s.pts.insert(Point{x: point.x, y: point.y+1, z: point.z+1});
        s.pts.insert(Point{x: point.x+1, y: point.y, z: point.z+1});
        s.pts.insert(Point{x: point.x+1, y: point.y+1, z: point.z+1});
        result.push(s);
    }
    {
        // LEFT
        let mut s: Side = Side{pts: HashSet::new()};
        s.pts.insert(Point{x: point.x, y: point.y, z: point.z});
        s.pts.insert(Point{x: point.x, y: point.y+1, z: point.z});
        s.pts.insert(Point{x: point.x, y: point.y, z: point.z+1});
        s.pts.insert(Point{x: point.x, y: point.y+1, z: point.z+1});
        result.push(s);
    }
    {
        // RIGHT
        let mut s: Side = Side{pts: HashSet::new()};
        s.pts.insert(Point{x: point.x+1, y: point.y, z: point.z});
        s.pts.insert(Point{x: point.x+1, y: point.y+1, z: point.z});
        s.pts.insert(Point{x: point.x+1, y: point.y, z: point.z+1});
        s.pts.insert(Point{x: point.x+1, y: point.y+1, z: point.z+1});
        result.push(s);
    }
    {
        // UP
        let mut s: Side = Side{pts: HashSet::new()};
        s.pts.insert(Point{x: point.x, y: point.y+1, z: point.z});
        s.pts.insert(Point{x: point.x+1, y: point.y+1, z: point.z});
        s.pts.insert(Point{x: point.x, y: point.y+1, z: point.z+1});
        s.pts.insert(Point{x: point.x+1, y: point.y+1, z: point.z+1});
        result.push(s);
    }
    {
        // DOWN
        let mut s: Side = Side{pts: HashSet::new()};
        s.pts.insert(Point{x: point.x, y: point.y, z: point.z});
        s.pts.insert(Point{x: point.x+1, y: point.y, z: point.z});
        s.pts.insert(Point{x: point.x, y: point.y, z: point.z+1});
        s.pts.insert(Point{x: point.x+1, y: point.y, z: point.z+1});
        result.push(s);
    }
    result
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
    println!("Points (volume) {}", points.len());
    let mut all_sides: Vec<Side> = Vec::new();
    let mut removed = 0;
    let mut counter = 0;
    for point in &points {
        if counter %100 == 0 {
            println!("counter: {counter}");
        }
        counter += 1;
        let sides = generate_sides(point);
        for s in sides {
            let found = all_sides.iter().position(|a_s| a_s==&s);
            if found.is_some() {
                all_sides.remove(found.unwrap());
                removed += 2;
                continue;
            }
            all_sides.push(s);
        }
    }
    let surface = all_sides.len();
    println!("Volume = {}, Surface = {} Removed {}", points.len(), surface, removed);
    0

}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
