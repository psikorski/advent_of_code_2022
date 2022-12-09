use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::{IResult, character::complete::digit1, combinator::map_res, character::complete::alpha1};
use nom::sequence::separated_pair;

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_line(line: &str) -> (&str, u32) {
    let mut parser = separated_pair(alpha1, tag(" "), parse_int);
    let (_, (letter, number)) = parser(line).unwrap();
    (letter, number)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(v_x: i32, v_y: i32) -> Self {
        Self { x: v_x, y: v_y }
    }
    pub fn def() -> Self {
        Position::new(0,0)
    }
}

fn dist(a: i32, b: i32) -> i32 {
    a - b
}

fn move_pos(p: &mut Position, d: &str) {
    match d{
        "R"=>p.x += 1,
        "L"=>p.x -= 1,
        "U"=>p.y += 1,
        "D"=>p.y -= 1,
        _=>println!("unknown direction '{d}'"),
    };
}

fn move_pos_p(p: &mut i32, dist: i32) {
    if dist > 0 {
        *p += 1;
    }
    else {
        *p -= 1;
    }
}

fn solve(input: &str) -> usize {
    let mut h = Position::def();
    let mut t = Position::def();

    let mut visitted_by_t: HashSet<Position> = HashSet::new();
    //visitted_by_t.push(Position::new(0,0));
    visitted_by_t.insert(t);
    let lines = input.lines();
    for line in lines {
        let (letter, number) = parse_line(line);
        println!("Letter: {letter}, number {number}");
        for _i in 0..number {
            move_pos(&mut h, letter);
            let dist_x = dist(h.x, t.x);
            let dist_y = dist(h.y, t.y);
            
            if dist_x != 0 && dist_y != 0 && dist_x.abs()+dist_y.abs() > 2 {
                move_pos_p(&mut t.x, dist_x);
                move_pos_p(&mut t.y, dist_y);
            }
            else {
                if dist_x.abs() > 1 {
                    move_pos_p(&mut t.x, dist_x);
                }
                if dist_y.abs() > 1 {
                    move_pos_p(&mut t.y, dist_y);
                }
            }
            println!("H ({},{}; T ({},{})", h.x, h.y, t.x, t.y);
            visitted_by_t.insert(t);
        }
    }
    visitted_by_t.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let result = solve(input);
    println!("result = {result}");
}
