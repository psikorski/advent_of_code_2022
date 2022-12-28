use std::collections::VecDeque;
use std::cmp::{max, min};

struct Position {
    pub x: i32,
    pub y: i32,
}

fn give_max(ground: &Vec<i32>) -> Position {
    let mut result = Position { x: 0, y: 0 };
    for i in 0..ground.len() {
        if ground[i] > result.y {
            result.y = ground[i];
            result.x = i as i32;
        }
    }
    result
}

trait Blowable {
    fn blow(&mut self, dir: char, ground: &Vec<i32>);
}

trait Fallable {
    fn fall(&mut self);
}

trait GroundCheckable {
    fn check_if_ground(&self, ground: &Vec<i32>) -> bool;
    fn give_highest(&self, ground: &Vec<i32>) -> Vec<i32>;
}

// #### h
// l
struct Dash {
    // 'D'
    pub left: i32,
    pub height: i32,
}

impl Dash{
    pub fn def(highest: i32) -> Self {
        Dash{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Dash {
    fn blow(&mut self, dir: char, ground: &Vec<i32>) {
        print!("Dash blown {}->", self.left);
        if dir == '<' {
            // TODO POROWNAJ Z GROUNE
            self.left = max(0, self.left-1);
        }
        else if dir == '>' {
            self.left = min(6, self.left+3 + 1) - 3;
        }
        print!("{}\n", self.left);
    }
}

impl Fallable for Dash {
    fn fall(&mut self) {
        print!("Dash falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
        print!("{}\n", self.height);
    }
}

impl GroundCheckable for Dash {
    fn check_if_ground(&self, ground: &Vec<i32>) -> bool {
        // TODO ZLE
        let highest = give_max(ground);
        return self.height == highest.y
    }

    fn give_highest(&self, ground: &Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = ground.clone();
        for i in 0..result.len() {
            if i < self.leftu() || i > self.leftu()+3{
                continue; // not needed
            }
            else {
                result[i] = self.height+1;
            }
        }
        result
    }
}

// .#.
// ### h
// .#.
// l
struct Plus {
// 'P'
    pub left: i32, // middle
    pub height: i32, // middle
}

impl Plus{
    pub fn def(highest: i32) -> Self {
        Plus{left: 2, height: highest + 4}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Plus {
    fn blow(&mut self, dir: char) {
        print!("Plus blown {}->", self.left);
        if dir == '<' {
            self.left = max(0, self.left-1);
        }
        else if dir == '>' {
            self.left = min(6, self.left+2 + 1) - 2;
        }
        print!("{}\n", self.left);
    }
}

impl Fallable for Plus {
    fn fall(&mut self) {
        print!("Plus falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
        print!("{}\n", self.height);
    }
}

impl GroundCheckable for Plus {
    fn check_if_ground(&self, ground: &Vec<i32>) -> bool {
        for i in 0..ground.len() {
            if self.leftu() > i {
                continue; // NOT NEEDED case
            }
            else if self.leftu() == i {
                if ground[i] == self.height {
                    return true;
                }
            }
            else if self.leftu() + 1 == i {
                if ground[i] == self.height-1 {
                    return true;
                }
            }
            else if self.leftu() + 2 == i {
                if ground[i] == self.height {
                    return true;
                }
            }
        }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = ground.clone();
        for i in 0..result.len() {
            if i < self.leftu() || i > self.leftu()+2{
                continue; // not needed
            }
            else if i == self.leftu() || i == self.leftu()+2 {
                result[i] += self.height;
            }
            else {
                result[i] += self.height+1;
            }
        }
        result
    }
}

// #
// #
// #
// # b
// l
struct Stick {
// 'T'
    pub left: i32,
    pub bottom: i32,
}

impl Stick{
    pub fn def(highest: i32) -> Self {
        Stick{left: 2, bottom: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Stick {
    fn blow(&mut self, dir: char) {
        print!("Stick blown {dir} {}->", self.left);
        if dir == '<' {
            self.left = max(0, self.left-1);
        }
        else if dir == '>' {
            self.left = min(6, self.left + 1);
        }
        print!("{}\n", self.left);
    }
}

impl Fallable for Stick {
    fn fall(&mut self) {
        print!("Stick falls {}->", self.bottom);
        self.bottom -= 1; // no need to check if >0 as outer algo
        print!("{}\n", self.bottom);
    }
}

impl GroundCheckable for Stick {
    fn check_if_ground(&self, ground: &Vec<i32>) -> bool {
        for i in 0..ground.len() {
            if self.leftu() == i {
                if ground[i] >= self.bottom {
                    return true;
                }
            }
        }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = ground.clone();
        for i in 0..result.len() {
            if i == self.leftu() {
                result[i] += 4;
            }
        }
        result
    }
}

// ..#
// ..#
// ### h
// l
struct Angle {
// 'A'
    pub left: i32,
    pub height: i32,
}

impl Angle{
    pub fn def(highest: i32) -> Self {
        Angle{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Angle {
    fn blow(&mut self, dir: char) {
        print!("Angle blown {dir} {}->", self.left);
        if dir == '<' {
            self.left = max(0, self.left-1);
        }
        else if dir == '>' {
            self.left = min(6, self.left+2 +1) - 2;
        }
        print!("{}\n", self.left);
    }
}

impl Fallable for Angle {
    fn fall(&mut self) {
        print!("Angle falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
        print!("{}\n", self.height);
    }
}

impl GroundCheckable for Angle {
    fn check_if_ground(&self, ground: &Vec<i32>) -> bool {
        for i in 0..ground.len() {
            if self.leftu() <= i && self.leftu() + 2 >= i {
                if ground[i] == self.height {
                    return true;
                }
            }
        }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = ground.clone();
        for i in 0..result.len() {
            if i == self.leftu() || i == self.leftu() + 1 {
                result[i] = self.height+1;
            }
            else if i == self.leftu() + 2 {
                result[i] = self.height+3;
            }
        }
        result
    }
}

// ##
// ## h
// l
struct Square {
// 'S'
    pub left: i32,
    pub height: i32,
}

impl Square{
    pub fn def(highest: i32) -> Self {
        Square{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Square {
    fn blow(&mut self, dir: char) {
        print!("Square blown {}->", self.left);
        if dir == '<' {
            self.left = max(0, self.left-1);
        }
        else if dir == '>' {
            self.left = min(6, self.left +1 + 1) - 1;
        }
        print!("{}\n", self.left);
    }
}

impl Fallable for Square {
    fn fall(&mut self) {
        print!("Square falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
        print!("{}\n", self.height);
    }
}

impl GroundCheckable for Square {
    fn check_if_ground(&self, ground: &Vec<i32>) -> bool {
        for i in 0..ground.len() {
            if self.leftu() <= i && self.leftu() + 1 >= i {
                if ground[i] == self.height {
                    return true;
                }
            }
        }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = ground.clone();
        for i in 0..result.len() {
            if i == self.leftu() || i == self.leftu() + 1 {
                result[i] = self.height+2;
            }
        }
        result
    }
}

trait Tetris: Blowable + Fallable + GroundCheckable {}
impl<T> Tetris for T where T: Blowable + Fallable + GroundCheckable {}


fn parse_line(_: &str) -> VecDeque<char> {
    let jets: VecDeque<char> = VecDeque::from(['>','>','>','<','<','>','<','>','>','<','<','<','>','>','<','>','>','>','<','<','<','>','>','>','<','<','<','>','<','<','<','>','>','<','>','>','<','<','>','>']);

    jets
}

fn solve_1(input: &str) -> usize {
    let mut jets = parse_line(input);

    let mut rocks_counter = 0;

    //let mut order: VecDeque<char> = VecDeque::from(['D','P','A','T','S']);
    let mut order: VecDeque<char> = VecDeque::from(['D','P','A','T','S']);

    let mut ground: Vec<i32> = vec![0;7];
    loop {
        println!("Ground: {:?}", ground);
        let highest_ground = give_max(&ground);
        if rocks_counter == 4 {
            break;
        }
        let ord_c = order.pop_front().unwrap();
        order.push_back(ord_c);
     
        let mut item: Box<dyn Tetris>;
        if ord_c == 'D' {
            item = Box::new(Dash::def(highest_ground.y));
            println!("Dash created");
        }
        else if ord_c == 'P' {
            item = Box::new(Plus::def(highest_ground.y));
            println!("Plus created");
        }
        else if ord_c == 'T' {
            item = Box::new(Stick::def(highest_ground.y));
            println!("Stick created");
        }
        else if ord_c == 'A' {
            item = Box::new(Angle::def(highest_ground.y));
            println!("Angle created");
        }
        else if ord_c == 'S' {
            item = Box::new(Square::def(highest_ground.y));
            println!("Square created");
        }
        else {
            item = Box::new(Dash::def(highest_ground.y));
            println!("Other created");
        }
        loop {
            let dir = jets.pop_front().unwrap();
            jets.push_back(dir);
            item.blow(dir);
            let res = item.check_if_ground(&ground);
            if res {
                ground = item.give_highest(&ground);
                // go to the next rock
                rocks_counter += 1;
                break;
            }
            item.fall();
        }
    }
    rocks_counter
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
