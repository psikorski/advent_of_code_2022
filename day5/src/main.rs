/// Parse a `u32` from the start of the input string.
/// (this is copied/pasted from earlier in the blog post!)
//pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
//    map_res(digit1, u32::from_str)(input)
//}

use nom::{
    character::complete::{digit1},
    combinator::map_res,
    IResult,
    sequence::delimited,
    sequence::separated_pair,
    bytes::complete::tag,
};

/// A point in 2D space
//#[derive(Debug, Eq, PartialEq)]


pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

pub struct Tower {
    pub id: usize,
    pub heap: Vec<char>,
}

impl Tower {
    fn print(&mut self) {
        println!("id {}: {:?}", self.id, self.heap);
    }
}
pub struct Towers {
    pub towers: Vec<Tower>,
}
impl Towers {
    fn init_tower(&mut self, count: usize) {
        for i in 1..count+1 {
            self.towers.push(Tower{id: i, heap: Vec::new()});
        }
    }
    fn from_line(&mut self, line: &str) {
        for i in 0..self.towers.len() {
            //println!("i {i}, char {}", i * 4 + 1);
            let el = line.as_bytes()[i * 4 + 1] as char;
            if el.is_whitespace() {
                continue;
            }
            self.towers[i].heap.insert(0, el);
            //println!("tower {i}, el {el}, tower all {}", self.towers[i].heap.len());
        }
    }
    fn print(&mut self) {
        for t in self.towers.iter_mut() {
            t.print();
        }
    }

    #[allow(dead_code)]
    fn move_blocks(&mut self, mv: Move) {
        for _i in 0..mv.count {
            let moved = self.towers[(mv.source - 1) as usize].heap.pop().unwrap();
            self.towers[(mv.dest - 1) as usize].heap.push(moved);
        }
    }
    fn move_blocks_part2(&mut self, mv: Move) {
        let mut count = mv.count as usize;
        while count > 0 {
            let len = self.towers[(mv.source - 1) as usize].heap.len();
            println!("len {len}, count {count}");
            let moved = self.towers[(mv.source - 1) as usize].heap.remove( len - count);
            self.towers[(mv.dest - 1) as usize].heap.push(moved);
            count -=1;
        }

    }
    fn print_last(&mut self) -> String {
        let mut res = String::new();
        for t in self.towers.iter_mut() {
            res.push(t.heap.pop().unwrap());
        }
        res
    }
}


pub struct Move {
    pub count: u32,
    pub source: u32,
    pub dest: u32,
}

impl Move {
    fn from_line(&mut self, line: &str) {
        let mut parser_first = delimited(tag("move "), parse_numbers, tag(" from "));
        let mut parser_second = separated_pair(parse_numbers, tag(" to "), parse_numbers);

        let (next, count) = parser_first(line).unwrap();
        let (_, (src, dest)) = parser_second(next).unwrap();
        self.count = count;
        self.source = src;
        self.dest = dest;
    }
}



fn solve(input: &str) -> String {
    let lines = input.lines();
    let mut templ = input.lines();

    let towers_count = (templ.next().unwrap().len() + 1) / 4;
    let mut towers = Towers{towers: Vec::new()};
    towers.init_tower(towers_count);

    for line in lines {
        if line.starts_with(" 1") {
            println!("tower numbers");
        }
        else if line.starts_with(" ") || line.starts_with("[") {
            println!("towers");
            towers.from_line(line);
        }
        else if line.is_empty() {
            
            println!("EMPTY");
            towers.print();
            println!("EMPTY");

        }
        else {
            println!("{line}");
            let mut mv = Move{count: 0,source: 0, dest:0};
            mv.from_line(line);
            println!("count {}, source {}, dest {}", mv.count, mv.source, mv.dest);
            towers.move_blocks_part2(mv);
            towers.print();
        }
    }
    towers.print_last()
}

fn main() {
    let input = include_str!("../input.txt");

    let result = solve(input);

    println!("result = {result}");
}
