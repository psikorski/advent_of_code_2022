//use pathfinding::dijkstra;
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node {
    pub x: i32,
    pub y: i32,
}

impl Node {

    // fn neighbours(&self) -> Vec<(Node, usize)> {
    //   let &Node(x, y) = self;
    //   vec![Node(x+1,y+2), Node(x+1,y-2), Node(x-1,y+2), Node(x-1,y-2),
    //     Node(x+2,y+1), Node(x+2,y-1), Node(x-2,y+1), Node(x-2,y-1)]
    //        .into_iter().map(|p| (p, 1)).collect()
    // }
    fn successors(&self, raw_bytes: &[u8], length: i32, height: i32, end: &Node) -> Vec<Node> {
        let &Node{x, y} = self;
        let mut result: Vec<Node> = Vec::new();
        let mut curr = raw_bytes[(x+y * length) as usize];
        if curr == 'S' as u8 {
            curr = 'a' as u8
        }
        println!("iter: ({}, {}) -> ({}, {})", x, y, end.x, end.y);  
        if x == end.x && y == end.y {
            println!("FOUND: {}, {}", end.x, end.y);  
        }
        //println!("SUCCESSORS: curr {}x{} = {curr} | length {length}, height {height}", x, y);
        let mut right = 0;
        if x+1 < length {
            right = raw_bytes[(y*length + x+1) as usize];
            if right == 'E' as u8 {
                right = 'z' as u8;
            }
        }
        let mut left = 0;
        if x-1 >= 0 {
           left = raw_bytes[(y*length + x-1) as usize];
            if left == 'E' as u8 {
                left = 'z' as u8;
            }
        }
        let mut top = 0;
        if y-1>=0 {
            top = raw_bytes[(x+(y-1)*length) as usize];
            if top == 'E' as u8 {
                top = 'z' as u8;
            }
        }
        let mut bottom = 0;
        if y+1 < height {
            bottom = raw_bytes[(x+(y+1)*length) as usize];
            if bottom == 'E' as u8 {
                bottom = 'z' as u8;
            }
        }

        //println!("current {curr}, > {right}, < {left}, ^ {top}, v {bottom}");
        if x+1 < length && right as i32 - curr  as i32 <= 1 { // >
            result.push(Node{x: x+1, y: y});
            if x+1 == end.x && y == end.y {
                println!("FOUND: > {}, {}", end.x, end.y);  
            }
            //println!("SUCCESSORS: > {}, {}", x+1, y);
        }
        //println!("right");
        if x-1>=0 && left as i32 - curr as i32 <= 1 { // <
            result.push(Node{x: x-1, y});
            if x-1 == end.x && y == end.y {
                println!("FOUND: < {}, {}", end.x, end.y);  
            }
            //println!("SUCCESSORS: < {}, {}", x-1, y);
        }
        //println!("left");
        if y-1>=0 && top as i32 - curr as i32 <= 1 { // ^
            result.push(Node{x: x, y: y-1});
            //println!("SUCCESSORS: ^ {}, {}", x, y-1);
            if x == end.x && y-1 == end.y {
                println!("FOUND: ^ {}, {}", end.x, end.y);  
            }
        }
        //println!("top");
        if y+1 < height && bottom as i32 - curr as i32 <= 1 { // V
            result.push(Node{x: x, y: y+1});
            //println!("SUCCESSORS: ^ {}, {}", x, y+1);
            if x == end.x && y+1 == end.y {
                println!("FOUND: v {}, {}", end.x, end.y);  
            }
        }
        println!("SUCCESSORS of ({},{}): all {}: {:?}", x, y, result.len(), result);
        result
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn solve_1(input: &str) -> usize {
    let lines = input.lines();
    let height = lines.count() as i32;
    let length = (input.find('\n').unwrap() - 1) as i32;
    //let length_with_white = length + 2;
    println!("W{length}xH{height}");
    
    let removed_whitespaces = remove_whitespace(input);
    println!("size orig {} noWh {}", input.len(), removed_whitespaces.len());

    let raw_bytes = removed_whitespaces.as_bytes();
    let s_id = raw_bytes.iter().position(|&i| i == 'S' as u8).unwrap() as i32;
    let e_id = raw_bytes.iter().position(|&i| i == 'E' as u8).unwrap() as i32;
    //std::mem::replace(&mut raw_bytes[s_id as usize], 'a' as u8);
    let start = Node{x: s_id%length, y: s_id / length};
    let end = Node{x: e_id%length, y: e_id / length};
    println!("start s_id{s_id} ({}x{}), end e_id{e_id} ({}x{})", start.x, start.y, end.x, end.y);

    let result = bfs(&start, |p| p.successors(raw_bytes, length, height, &end), |p| *p == end);
    let mut ret = 0;
    if result.is_none() {
        println!("NONE");
    }
    else {
        let res_unwrapped = result.unwrap();
        println!("SOME: size {}", res_unwrapped.len());
        ret = res_unwrapped.len() - 1;
        for r in res_unwrapped {
            println!("N {}x{}", r.x, r.y);
        }
    }
    ret
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
