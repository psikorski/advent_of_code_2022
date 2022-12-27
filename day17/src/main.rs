use std::collections::VecDeque;

use sscanf::sscanf;



fn parse_line(line: &str) -> Valve {
    let mut left_right = line.split("; ");
    let left = left_right.next();
    let right = left_right.next();
    let parsed_valve = 
         sscanf!(left.unwrap(), "Valve {} has flow rate={}", String, i32);
    let (name, rate) = parsed_valve.unwrap();

    // TODO find valves / valve
    //  changed in input_sample.txt
    //  maybe also can be changed in input.txt :)
    let mut right_right = right.unwrap().split(" valves ");
    let right0 = right_right.next();
    let right1 = right_right.next();
    let mut to_valves = right1.unwrap().split(", ");

    let mut result = Valve::simple(&name, rate);
    
    for v in to_valves {
        let parsed_to_valve = sscanf!(v, "{}", String);
        result.valves.push(parsed_to_valve.unwrap());
    }
    result
}

fn solve_1(_: &str) -> usize {
    let mut jets: VecDeque<char> = VecDeque::from(['>','>','>','<','<','>','<','>','>','<','<','<','>','>','<','>','>','>','<','<','<','>','>','>','<','<','<','>','<','<','<','>','>','<','>','>','<','<','>','>',']);

    
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
