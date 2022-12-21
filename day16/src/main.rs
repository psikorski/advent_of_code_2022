use sscanf::sscanf;
use std::{collections::{HashMap, VecDeque}, i32::MAX, arch::x86_64::has_cpuid};
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

/*

 1  procedure BFS(G, root) is
 2      let Q be a queue
 3      label root as explored
 4      Q.enqueue(root)
 5      while Q is not empty do
 6          v := Q.dequeue()
 7          if v is the goal then
 8              return v
 9          for all edges from v to w in G.adjacentEdges(v) do
10              if w is not labeled as explored then
11                  label w as explored
12                  w.parent := v
13                  Q.enqueue(w)


1  procedure DFS(G, root) is
let S be a stack
    S.push(v)
    while S is not empty do
        v = S.pop()
        if v is not labeled as discovered then
            label v as discovered
            for all edges from v to w in G.adjacentEdges(v) do 
                S.push(w)


 1  function Dijkstra(Graph, source):
 2      
 3      for each vertex v in Graph.Vertices:
 4          dist[v] ← INFINITY
 5          prev[v] ← UNDEFINED
 6          add v to Q
 7      dist[source] ← 0
 8      
 9      while Q is not empty:
10          u ← vertex in Q with min dist[u]
11          remove u from Q
12          
13          for each neighbor v of u still in Q:
14              alt ← dist[u] + Graph.Edges(u, v)
15              if alt < dist[v]:
16                  dist[v] ← alt
17                  prev[v] ← u
18
19      return dist[], prev[]

*/

fn Dijkstra(start: &Valve) {
    let mut dist: HashMap<String, i32> = HashMap::new();
    let mut prev: HashMap<String, (String, i32)> = HashMap::new();

    let mut queue: VecDeque<Valve> = Vec::new();
    queue.push_back(start.clone());

    dist.insert(start.name, 0);  
  
    let mut minutes = 30;
    while queue.len() > 0 {
        // TODO get current rate (for opened_valves...)
        let current_opt = queue.pop_front();
        let current = current_opt.unwrap();


        let children = current.valves();
        let mut c_dist = dist.get_mut(current.name);
        for c in children {
            dist.get_mut(c.)
        }
        //      if 
        //      if new_dist > dist[current.name]
        //         dist[c.name] = new_dist
        //         prev[c.name] = child.name


    }
}

fn solve_1(input: &str) -> usize {
    let lines = input.lines();
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in lines {
        let valve = parse_line(line);
        valves.insert(valve.name.clone(), valve);
    }
    let start = valves.get("AA").unwrap();

    let 
    let result = 
        bfs(
            &start, 
            |p| p.successors(), 
            |p| 
        );


    valves.len()
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
