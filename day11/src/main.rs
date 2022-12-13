use std::collections::VecDeque;

use sscanf::sscanf;

type Operation = fn(i32, i32) -> i32;

struct Monkey{
    pub id: i32,
    pub items: VecDeque<i32>,
    pub operation: Option<Operation>,
    pub operation_number: i32,
    pub divisible_by: i32,
    pub divisible_true: i32,
    pub divisible_false: i32,
    pub items_visitted: i32,
}

impl Monkey {
    pub fn def() -> Self {
        Self {
            id : 0,
            items : VecDeque::new(),
            operation : Option::None,
            operation_number: 0,
            divisible_by : 0,
            divisible_true : 0,
            divisible_false : 0,
            items_visitted : 0,
        }
    }
}

/*
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
*/
fn read_monkey(lines: &Vec<&str>, id: usize) -> Monkey {
    let mut monkey = Monkey::def();
    let parsed = sscanf!(lines[id], "Monkey {}:", i32);
    monkey.id = parsed.unwrap();
    println!("Monkey number {}", monkey.id);
    let starting_items = lines[id+1]; //   Starting items: 79, 98
    monkey.items = starting_items[18..].split(", ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    //monkey.items = VecDeque::from(temp_items);
    println!("Monkey items {:?}", monkey.items);
    let operations14 = &lines[id+2][13..]; //  Operation: new = old * 19
    let parsed_operations = sscanf!(operations14, "new = {} {} {}", str, str, str);
    let (first, operator, second) = parsed_operations.unwrap();
    println!("first {first}, operator {operator} second {second}");

    if second == "old" {
        if operator == "+" {
            monkey.operation = Some(|x, _| x + x);
        }
        else if operator == "-" {
            monkey.operation = Some(|x, _| x - x);
            // it can be just 0
        }
        else if operator == "*" {
            monkey.operation = Some(|x, _| x * x);
        }
        else if operator == "/" {
            monkey.operation = Some(|x, _| x / x);
            // it can be just 1
        }
        else {
            println!("ERROR");
        }
    }
    else {
        monkey.operation_number = second.parse().expect("should be convertable");
        if operator == "+" {
            monkey.operation = Some(|x, y| x + y);
        }
        else if operator == "-" {
            monkey.operation = Some(|x, y| x - y);
        }
        else if operator == "*" {
            monkey.operation = Some(|x, y| x * y);
        }
        else if operator == "/" {
            monkey.operation = Some(|x, y| x / y);
        }
        else {
            println!("ERROR");
        }
    }    
    monkey.divisible_by = sscanf!(lines[id+3], "  Test: divisible by {}", i32).unwrap();
    monkey.divisible_true = sscanf!(lines[id+4], "    If true: throw to monkey {}", i32).unwrap();
    monkey.divisible_false = sscanf!(lines[id+5], "    If false: throw to monkey {}", i32).unwrap();
    println!("divisible_by {}, divisible_true {}, divisible_false {}", monkey.divisible_by, monkey.divisible_true, monkey.divisible_false);
    monkey
}


fn solve_1(input: &str) -> i32 {
    let lines_vec: Vec<&str> = input.lines().collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut id = 0;
    while id <= lines_vec.len() - 7 {
        monkeys.push(read_monkey(&lines_vec, id));
        id += 7;
    }
    let monkeys_count = monkeys.len();
    for _i in 1..21 {
        // round
        //let mut mid = monkeys.len();
        for mid in 0..monkeys_count {           
            loop {
                let passed_value: i32;
                let to_whom: usize;
                {
                    let curr_monkey = &mut monkeys[mid];
                    let item = curr_monkey.items.pop_front();
                    if item.is_none() {
                        break;
                    }
                    curr_monkey.items_visitted+=1;
                    let item_num = item.unwrap();
                    let item_num_after = curr_monkey.operation.unwrap()(item_num, curr_monkey.operation_number);
                    //println!("Monkey {mid}; item before {item_num}, item_after {item_num_after}");
                    passed_value = item_num_after / 3;
                    if passed_value % curr_monkey.divisible_by == 0 {
                        to_whom = curr_monkey.divisible_true as usize; 
                    }
                    else {
                        to_whom = curr_monkey.divisible_false as usize;
                    }
                }
                //println!("from {mid} to {to_whom} value {passed_value}");
                monkeys[to_whom].items.push_back(passed_value);
            }
        } // monkeys
    } //round
    let mut max1 = 0;
    let mut max2 = 0;
    for inner in 0..monkeys_count {
        let visitted = monkeys[inner].items_visitted;
        if visitted >= max1 {
            max2 = max1;
            max1 = visitted;
        }
        else if visitted > max2 {
            max2 = visitted;
        }
        println!("ROUND 20, monkey {}, visitted {}, items {:?}", monkeys[inner].id, monkeys[inner].items_visitted, monkeys[inner].items);
     }
     max1 * max2
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
