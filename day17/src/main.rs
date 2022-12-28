use std::collections::{VecDeque, HashSet};
use std::cmp::{max, min};
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position {
    pub x: i32,
    pub y: i64,
}

impl Position {
    pub fn new(v_x: i32, v_y: i64) -> Self {
        Self { x: v_x, y: v_y }
    }

}

// fn give_max(ground: &Vec<i64>) -> Position {
//     let mut result = Position { x: 0, y: 0 };
//     for i in 0..ground.len() {
//         if ground[i] > result.y {
//             result.y = ground[i];
//             result.x = i as i32;
//         }
//     }
//     result
// }

trait Blowable {
    fn blow(&mut self, dir: char, occupied: &HashSet<Position>);
}

trait Fallable {
    fn fall(&mut self, occupied: &HashSet<Position>) -> bool;
}

trait GroundCheckable {
    fn check_if_ground(&self, ground: &Vec<i64>) -> bool;
    fn give_highest(&self, ground: &Vec<i64>) -> Vec<i64>;
    fn occuppies(&self) -> Vec<Position>;
}

// #### h
// l
struct Dash {
    // 'D'
    pub left: i32,
    pub height: i64,
}

impl Dash{
    pub fn def(highest: i64) -> Self {
        Dash{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Dash {
    fn blow(&mut self, dir: char, occupied: &HashSet<Position>) {
       // print!("Dash blown {}->", self.left);
        if dir == '<' {
            let maxed = max(0, self.left-1);
            if ! occupied.contains(&Position::new(maxed, self.height)) {
                self.left = maxed;
            }
            else {
         //       print!("!");
            }
        }
        else if dir == '>' {
            let minned = min(3, self.left+1);
            if ! occupied.contains(&Position::new(minned, self.height)) {
                self.left = minned;
            }
            else {
           //     print!("!");
            }
        }
  //      print!("{}\n", self.left);
    }
}

impl Fallable for Dash {
    fn fall(&mut self, occupied: &HashSet<Position>) -> bool {
        if  self.height == 0 ||
            occupied.contains(&Position::new(self.left, self.height-1)) || 
            occupied.contains(&Position::new(self.left+1, self.height-1)) || 
            occupied.contains(&Position::new(self.left+2, self.height-1)) || 
            occupied.contains(&Position::new(self.left+3, self.height-1)) {
    //        println!("Dash NOT falls {}", self.height);
            return false
        }
      //  print!("Dash falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
    //    print!("{}\n", self.height);
        return true
    }
}

impl GroundCheckable for Dash {
    fn check_if_ground(&self, ground: &Vec<i64>) -> bool {
        if 
            ground[self.leftu()] == self.height || 
            ground[self.leftu()+1] == self.height ||
            ground[self.leftu()+2] == self.height ||
            ground[self.leftu()+3] == self.height {
            return true
        }
        // for i in 0..ground.len() {
        //     if i>= self.leftu() && i<= self.leftu()+3 {
        //         if self.height == ground[i] {
        //             return true
        //         }
        //     }
        // }
        false
    }

    fn give_highest(&self, ground: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = ground.clone();
        result[self.leftu()] = self.height;
        result[self.leftu()+1] = self.height;
        result[self.leftu()+2] = self.height;
        result[self.leftu()+3] = self.height;
        // for i in 0..result.len() {
        //     if i < self.leftu() || i > self.leftu()+3{
        //         continue; // not needed
        //     }
        //     else {
        //         result[i] = self.height;
        //     }
        // }
        result
    }

    fn occuppies(&self) -> Vec<Position> {
        let mut result = Vec::new();
        result.push(Position::new(self.left, self.height));
        result.push(Position::new(self.left+1, self.height));
        result.push(Position::new(self.left+2, self.height));
        result.push(Position::new(self.left+3, self.height));
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
    pub height: i64, // middle
}

impl Plus{
    pub fn def(highest: i64) -> Self {
        Plus{left: 2, height: highest + 4}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Plus {
    fn blow(&mut self, dir: char, occupied: &HashSet<Position>) {
       // print!("Plus blown {}->", self.left);
        if dir == '<' {
            let maxed = max(0, self.left-1);
            if  ! occupied.contains(&Position::new(maxed, self.height)) && 
                ! occupied.contains(&Position::new(maxed+1, self.height-1)) &&
                ! occupied.contains(&Position::new(maxed+1, self.height+1)) {
                self.left = maxed;
            }
            else {
         //       print!("!");
            }
        }
        else if dir == '>' {
            let minned = min(4, self.left+1) ;
            if  ! occupied.contains(&Position::new(minned+2, self.height)) && 
                ! occupied.contains(&Position::new(minned+1, self.height-1)) &&
                ! occupied.contains(&Position::new(minned+1, self.height+1)) {
                self.left = minned;
            }
            else {
         //       print!("!");
            }
        }
        //print!("{}\n", self.left);
    }
}

impl Fallable for Plus {
    fn fall(&mut self, occupied: &HashSet<Position>) -> bool {
        if  self.height == 1 ||
            occupied.contains(&Position::new(self.left, self.height-1)) || 
            occupied.contains(&Position::new(self.left+1, self.height-2)) || 
            occupied.contains(&Position::new(self.left+2, self.height-1))  {
       //     println!("Plus NOT falls {}", self.height);
            return false
        }
     //   print!("Plus falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
      //  print!("{}\n", self.height);
        return true
    }
}

impl GroundCheckable for Plus {
    fn check_if_ground(&self, ground: &Vec<i64>) -> bool {
        if 
            ground[self.leftu()] == self.height || 
            ground[self.leftu()+1] == self.height-1 ||
            ground[self.leftu()+2] == self.height {
            return true
        }
        // for i in 0..ground.len() {
        //     if self.leftu() > i {
        //         continue; // NOT NEEDED case
        //     }
        //     else if self.leftu() == i {
        //         if ground[i] == self.height {
        //             return true;
        //         }
        //     }
        //     else if self.leftu() + 1 == i {
        //         if ground[i] == self.height-1 {
        //             return true;
        //         }
        //     }
        //     else if self.leftu() + 2 == i {
        //         if ground[i] == self.height {
        //             return true;
        //         }
        //     }
        // }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = ground.clone();
        result[self.leftu()] = self.height;
        result[self.leftu()+1] = self.height+1;
        result[self.leftu()+2] = self.height;
        // for i in 0..result.len() {
        //     if i < self.leftu() || i > self.leftu()+2{
        //         continue; // not needed
        //     }
        //     else if i == self.leftu() || i == self.leftu()+2 {
        //         result[i] = self.height;
        //     }
        //     else {
        //         result[i] = self.height+1;
        //     }
        // }
        result
    }

    fn occuppies(&self) -> Vec<Position> {
        let mut result = Vec::new();
        result.push(Position::new(self.left, self.height));
        result.push(Position::new(self.left+1, self.height));
        result.push(Position::new(self.left+2, self.height));
        result.push(Position::new(self.left+1, self.height+1));
        result.push(Position::new(self.left+1, self.height-1));
        result
    }
}

// #
// #
// #
// # h
// l
struct Stick {
// 'T'
    pub left: i32,
    pub height: i64,
}

impl Stick{
    pub fn def(highest: i64) -> Self {
        Stick{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Stick {
    fn blow(&mut self, dir: char, occupied: &HashSet<Position>) {
       // print!("Stick blown {dir} {}->", self.left);
        if dir == '<' {
            let maxed = max(0, self.left-1);
            if  ! occupied.contains(&Position::new(maxed, self.height)) && 
                ! occupied.contains(&Position::new(maxed, self.height+1)) &&
                ! occupied.contains(&Position::new(maxed, self.height+2)) &&
                ! occupied.contains(&Position::new(maxed, self.height+3)) {
                self.left = maxed;
            }
            else {
         //       print!("!");
            }
        }
        else if dir == '>' {
            let minned = min(6, self.left + 1);
            if  ! occupied.contains(&Position::new(minned, self.height)) && 
                ! occupied.contains(&Position::new(minned, self.height+1)) &&
                ! occupied.contains(&Position::new(minned, self.height+2)) &&
                ! occupied.contains(&Position::new(minned, self.height+3)) {
            self.left = minned;
          }
            else {
      //          print!("!");
            }
        }
      //  print!("{}\n", self.left);
    }
}

impl Fallable for Stick {
    fn fall(&mut self, occupied: &HashSet<Position>) -> bool {
        if  self.height == 0 ||
            occupied.contains(&Position::new(self.left, self.height-1)) {
            //println!("Stick NOT falls {}", self.height);
            return false
        }

        //print!("Stick falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
        //print!("{}\n", self.height);
        return true
    }
}

impl GroundCheckable for Stick {
    fn check_if_ground(&self, ground: &Vec<i64>) -> bool {
        if 
            ground[self.leftu()] == self.height {
                return true
        }
        // for i in 0..ground.len() {
        //     if self.leftu() == i {
        //         if ground[i] == self.height {
        //             return true
        //         }
        //     }
        // }
        return false
    }

    fn give_highest(&self, ground: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = ground.clone();
        result[self.leftu()] = self.height+3;
        // for i in 0..result.len() {
        //     if i == self.leftu() {
        //         result[i] = self.height+3;
        //     }
        // }
        result
    }

    fn occuppies(&self) -> Vec<Position> {
        let mut result = Vec::new();
        result.push(Position::new(self.left, self.height));
        result.push(Position::new(self.left, self.height+1));
        result.push(Position::new(self.left, self.height+2));
        result.push(Position::new(self.left, self.height+3));
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
    pub height: i64,
}

impl Angle{
    pub fn def(highest: i64) -> Self {
        Angle{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Angle {
    fn blow(&mut self, dir: char, occupied: &HashSet<Position>) {
       // print!("Angle blown {dir} {}->", self.left);
        if dir == '<' {
            let maxed = max(0, self.left-1);
            if  ! occupied.contains(&Position::new(maxed, self.height)) && 
                ! occupied.contains(&Position::new(maxed+2, self.height+1)) &&
                ! occupied.contains(&Position::new(maxed+2, self.height+2)) {
                self.left = maxed;
            }
            else {
         //       print!("!");
            }
        }
        else if dir == '>' {
            let minned = min(4, self.left +1) ;
            if  ! occupied.contains(&Position::new(minned+2, self.height)) && 
                ! occupied.contains(&Position::new(minned+2, self.height+1)) &&
                ! occupied.contains(&Position::new(minned+2, self.height+2)) {
                self.left = minned;
            }
            else {
       //         print!("!");
            }
        }
     //   print!("{}\n", self.left);
    }
}

impl Fallable for Angle {
    fn fall(&mut self, occupied: &HashSet<Position>) -> bool {
        if  self.height == 0 ||
            occupied.contains(&Position::new(self.left, self.height-1)) ||
            occupied.contains(&Position::new(self.left+1, self.height-1)) ||
            occupied.contains(&Position::new(self.left+2, self.height-1)) {
          //  println!("Angle NOT falls {}", self.height);
            return false
        }
     //   print!("Angle falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
     //   print!("{}\n", self.height);
        return true
    }
}

impl GroundCheckable for Angle {
    fn check_if_ground(&self, ground: &Vec<i64>) -> bool {
        if 
            ground[self.leftu()] == self.height || 
            ground[self.leftu()+1] == self.height ||
            ground[self.leftu()+2] == self.height {
            return true
        }
        // for i in 0..ground.len() {
        //     if self.leftu() <= i && self.leftu() + 2 >= i {
        //         if ground[i] == self.height {
        //             return true;
        //         }
        //     }
        // }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = ground.clone();
        result[self.leftu()] = self.height;
        result[self.leftu()+1] = self.height;
        result[self.leftu()+2] = self.height+2;
        // for i in 0..result.len() {
        //     if i == self.leftu() || i == self.leftu() + 1 {
        //         result[i] = self.height;
        //     }
        //     else if i == self.leftu() + 2 {
        //         result[i] = self.height+2;
        //     }
        // }
        result
    }

    fn occuppies(&self) -> Vec<Position> {
        let mut result = Vec::new();
        result.push(Position::new(self.left, self.height));
        result.push(Position::new(self.left+1, self.height));
        result.push(Position::new(self.left+2, self.height));
        result.push(Position::new(self.left+2, self.height+1));
        result.push(Position::new(self.left+2, self.height+2));
        result
    }
}

// ##
// ## h
// l
struct Square {
// 'S'
    pub left: i32,
    pub height: i64,
}

impl Square{
    pub fn def(highest: i64) -> Self {
        Square{left: 2, height: highest + 3}
    }
    pub fn leftu(&self) -> usize {
        self.left as usize
    }
}

impl Blowable for Square {
    fn blow(&mut self, dir: char, occupied: &HashSet<Position>) {
        //print!("Square blown {}->", self.left);
        if dir == '<' {
            let maxed = max(0, self.left-1);
            if  ! occupied.contains(&Position::new(maxed, self.height)) && 
                ! occupied.contains(&Position::new(maxed, self.height+1)) {
            self.left = maxed;
            }
            else {
                //print!("!");
            }
        }
        else if dir == '>' {
            let minned = min(5, self.left + 1);
            if  ! occupied.contains(&Position::new(minned+1, self.height)) && 
                ! occupied.contains(&Position::new(minned+1, self.height+1)) {
            self.left = minned;
            }
            else {
                //print!("!");
            }
        }
        //print!("{}\n", self.left);
    }
}

impl Fallable for Square {
    fn fall(&mut self, occupied: &HashSet<Position>) -> bool{
        if  self.height == 0 ||
            occupied.contains(&Position::new(self.left, self.height-1)) ||
            occupied.contains(&Position::new(self.left+1, self.height-1)) {
            //println!("Square NOT falls {}", self.height);
            return false
        }
        //print!("Square falls {}->", self.height);
        self.height -= 1; // no need to check if >0 as outer algo
        //print!("{}\n", self.height);
        return true
    }
}

impl GroundCheckable for Square {
    fn check_if_ground(&self, ground: &Vec<i64>) -> bool {
        if 
            ground[self.leftu()] == self.height || 
            ground[self.leftu()+1] == self.height {
            return true
        }
        // for i in 0..ground.len() {
        //     if self.leftu() <= i && self.leftu() + 1 >= i {
        //         if ground[i] == self.height {
        //             return true;
        //         }
        //     }
        // }
        return false;
    }

    fn give_highest(&self, ground: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = ground.clone();
        result[self.leftu()] = self.height+1;
        result[self.leftu()+1] = self.height+1;
        // for i in 0..result.len() {
        //     if i == self.leftu() || i == self.leftu() + 1 {
        //         result[i] = self.height+1;
        //     }
        // }
        result
    }

    fn occuppies(&self) -> Vec<Position> {
        let mut result = Vec::new();
        result.push(Position::new(self.left, self.height));
        result.push(Position::new(self.left+1, self.height));
        result.push(Position::new(self.left, self.height+1));
        result.push(Position::new(self.left+1, self.height+1));
        result
    }
}

trait Tetris: Blowable + Fallable + GroundCheckable {}
impl<T> Tetris for T where T: Blowable + Fallable + GroundCheckable {}


fn parse_line(line: &str) -> VecDeque<char> {
    //let jets: VecDeque<char> = VecDeque::from(['>','>','>','<','<','>','<','>','>','<','<','<','>','>','<','>','>','>','<','<','<','>','>','>','<','<','<','>','<','<','<','>','>','<','>','>','<','<','>','>']);
    let jets = VecDeque::from(line.chars().collect::<Vec<char>>());
    jets
}

fn solve_1(input: &str) -> usize {
    let mut jets = parse_line(input);

    let mut rocks_counter: i64 = 0;

    //let mut order: VecDeque<char> = VecDeque::from(['D','P','A','T','S']);
    let mut order: VecDeque<char> = VecDeque::from(['D','P','A','T','S']);

    let mut ground: Vec<i64> = vec![-1;7];
    let mut occupied: HashSet<Position> = HashSet::with_capacity(600);
    let start = Instant::now();
    
    
    loop {
        let mut highest_ground = -1;
        let mut lowest_ground = i64::MAX;
        //let highest_ground = give_max(&ground);
        

        for g in &ground {
            lowest_ground = min(*g, lowest_ground);
           highest_ground = max(*g, highest_ground);
        }
        //if rocks_counter == 0 {
           // occupied.retain(|f| -> bool {highest_ground = max(f.y, highest_ground); f.y >= lowest_ground-1 });
            for oc in &occupied {
                highest_ground = max(oc.y, highest_ground);
            }
        //}
        
        highest_ground += 1;

        if occupied.len() > 300 {
            //print!("\tocc_before={}", occupied.len());
            occupied.retain(|f| f.y >= lowest_ground-1 );
            //print!(" occ_after={}\n", occupied.len());
        }
        if rocks_counter % 100000 == 0 {
            let duration = start.elapsed();

            print!("time={:?}, occ={} rocks={rocks_counter} Ground({lowest_ground}/{}): {:?}", duration, occupied.len(), highest_ground, ground);
            //occupied.retain(|f| f.y >= lowest_ground-1 );
            print!(" ---> occ_after={}\n\n", occupied.len());
            // if duration.as_secs() > 20 {
            //     break;
            // }
        }
        //if rocks_counter == 2022 {
        if rocks_counter == 1000000000000 {
            break;
        }
        let ord_c = order.pop_front().unwrap();
        order.push_back(ord_c);
     
        let mut item: Box<dyn Tetris>;
        if ord_c == 'D' {
            item = Box::new(Dash::def(highest_ground));
           // println!("Dash created");
        }
        else if ord_c == 'P' {
            item = Box::new(Plus::def(highest_ground));
           // println!("Plus created");
        }
        else if ord_c == 'T' {
            item = Box::new(Stick::def(highest_ground));
           // println!("Stick created");
        }
        else if ord_c == 'A' {
            item = Box::new(Angle::def(highest_ground));
           // println!("Angle created");
        }
        else if ord_c == 'S' {
            item = Box::new(Square::def(highest_ground));
           // println!("Square created");
        }
        else {
            item = Box::new(Dash::def(highest_ground));
            println!("Other created");
        }
        loop {
            let dir = jets.pop_front().unwrap();
            jets.push_back(dir);
            item.blow(dir, &occupied);
            //let res = item.check_if_ground(&ground);
            if  !item.fall(&occupied) {
                ground = item.give_highest(&ground);
                let occs = item.occuppies();
                // for oc in &occs {
                //     highest_ground = max(oc.y, highest_ground);
                // }
                for oc in occs {
                    //highest_ground = max(oc.y, highest_ground);
                    occupied.insert(oc);
                }
                // go to the next rock
                rocks_counter += 1;
                break;
            }
        }
    }
    //println!("{:#?}", occupied);
    let mut max_y = 0;
    for oc in occupied {
        max_y = max(oc.y, max_y);
    }
    max_y as usize
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
