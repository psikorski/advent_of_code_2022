use sscanf::sscanf;
use std::{collections::{HashMap, VecDeque, HashSet}, i32::MAX, arch::x86_64::has_cpuid};
use pathfinding::prelude::bfs;

#[derive(Clone)]
struct Valve {
    pub name: String,
    pub rate: i32,
    pub valves: Vec<String>,
}

impl Valve {
    fn simple(a_name: &str, a_rate: i32) -> Self {
        Valve { name: a_name.to_string(), rate: a_rate, valves: Vec::new() }
    }
    
    fn successors(&self, valves: &HashMap<String, Valve>) -> Vec<Valve> {
        let mut result: Vec<Valve> = Vec::new();
        for v in &self.valves {
            let child = valves.get(v);
            if child.is_none() {
                println!("EEROOR: {}", v);
                continue;
            }
            result.push(child.unwrap().clone());
        }
        result
    }
    
}

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


fn solve_1(input: &str) -> usize {
    let lines = input.lines();
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in lines {
        let valve = parse_line(line);
        valves.insert(valve.name.clone(), valve);
    }
    let start = valves.get("AA").unwrap();

    let mut dists: HashMap<String, i32> = HashMap::new();
    let mut nonempty = Vec::new();


    for (v_name, valve) in &valves {
        if v_name != "AA" && valve.rate == 0 {
            continue;
        }
        //dists.insert(v_name, 0);
        //dists.insert("AA", 0);
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(*v_name);
        let mut queue: VecDeque<(i32, &str)> = VecDeque::new();
        queue.push_back((0, &v_name));
        while queue.len() > 0 {
            let (dist, position) = queue.pop_front().unwrap();
            let neighbours = valves[position].valves;
            for nb in neighbours {
                if visited.contains(&nb) {
                    continue;
                }
                visited.insert(nb);
                if valves[&nb].rate != 0 {
                    dists
                }
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
