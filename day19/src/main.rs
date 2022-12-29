use sscanf::sscanf;
use pathfinding::prelude::dfs;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::time::Instant;

struct Blueprint {
    pub id: i32,
    pub ore_robot_ore_cost: i32,
    pub clay_robot_ore_cost: i32,
    pub obsidian_robot_ore_cost: i32,
    pub obsidian_robot_clay_cost: i32,
    pub geode_robot_ore_cost: i32,
    pub geode_robot_obsidian_cost: i32,
    pub spent_ore_rate: i32,
}

pub fn max_spent_ore_rate(
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    geode_robot_ore_cost: i32
) -> i32 {
    max(
        max(ore_robot_ore_cost, clay_robot_ore_cost), 
        max(obsidian_robot_ore_cost, geode_robot_ore_cost)
    )
}


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pub elapsed: i32,
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
    pub geode: i32,

    pub robot_ore: i32,
    pub robot_clay: i32,
    pub robot_obsidian: i32,
    pub robot_geode: i32,
}

impl Node {
    pub fn collect_everything_node(&self, blueprint: &Blueprint, max_elapsed: i32) -> Node {
        let remaining = max_elapsed - self.elapsed; // ? 
        Node{
            elapsed: self.elapsed+1,
            ore: min(self.ore + self.robot_ore,blueprint.spent_ore_rate*remaining),
            clay: min(self.clay + self.robot_clay, blueprint.obsidian_robot_clay_cost*remaining),
            obsidian: min(self.obsidian + self.robot_obsidian, blueprint.geode_robot_obsidian_cost*remaining),
            geode: self.geode + self.robot_geode,
            robot_ore: self.robot_ore,
            robot_clay: self.robot_clay,
            robot_obsidian: self.robot_obsidian,
            robot_geode: self.robot_geode
        }
    }
    pub fn create_ore_robot_node(&self, blueprint: &Blueprint, cache: &mut HashSet<Node>, max_elapsed: i32) -> Option<Node> {
        if self.ore < blueprint.ore_robot_ore_cost {
            return Option::None
        }
        if self.robot_ore >= blueprint.spent_ore_rate {
            return Option::None
        }
        let mut node = self.collect_everything_node(blueprint, max_elapsed);
        node.ore -= blueprint.ore_robot_ore_cost;
        node.robot_ore += 1;
        if ! cache.insert(node) {
            return Option::None
        }
        Some(node)
    }
    pub fn create_clay_robot_node(&self, blueprint: &Blueprint, cache: &mut HashSet<Node>, max_elapsed: i32) -> Option<Node> {
        if self.ore < blueprint.clay_robot_ore_cost {
            return Option::None
        }
        if self.robot_clay >= blueprint.obsidian_robot_clay_cost {
            return Option::None
        }

        let mut node = self.collect_everything_node(blueprint, max_elapsed);
        node.ore -= blueprint.clay_robot_ore_cost;
        node.robot_clay += 1;
        if ! cache.insert(node) {
            return Option::None
        }
        Some(node)
    }
    pub fn create_obsidian_robot_node(&self, blueprint: &Blueprint, cache: &mut HashSet<Node>, max_elapsed: i32) -> Option<Node> {
        if  self.ore < blueprint.obsidian_robot_ore_cost || 
            self.clay < blueprint.obsidian_robot_clay_cost {
            return Option::None
        }
        if self.robot_obsidian >= blueprint.geode_robot_obsidian_cost {
            return Option::None
        }
        let mut node = self.collect_everything_node(blueprint, max_elapsed);
        node.ore -= blueprint.obsidian_robot_ore_cost;
        node.clay -= blueprint.obsidian_robot_clay_cost;
        node.robot_obsidian += 1;
        if ! cache.insert(node) {
            return Option::None
        }
        Some(node)
    }
    pub fn create_geode_robot_node(&self, blueprint: &Blueprint, cache: &mut HashSet<Node>, max_elapsed: i32) -> Option<Node> {
        if  self.ore < blueprint.geode_robot_ore_cost || 
            self.obsidian < blueprint.geode_robot_obsidian_cost {
            return Option::None
        }
        let mut node = self.collect_everything_node(blueprint, max_elapsed);
        node.ore -= blueprint.geode_robot_ore_cost;
        node.obsidian -= blueprint.geode_robot_obsidian_cost;
        node.robot_geode += 1;
        if ! cache.insert(node) {
            return Option::None
        }
        Some(node)
    }

    pub fn neighbors(&self, blueprint: &Blueprint, max_elapsed: i32, cache: &mut HashSet<Node>) -> Vec<Node> {
        let mut result = Vec::new();
        let next_elapsed = self.elapsed + 1;
        if next_elapsed > max_elapsed {
            // end node
            return result
        }
        let collector =self.collect_everything_node(blueprint, max_elapsed);
        if cache.insert(collector) {
            result.push(collector);
        }
        let ore_robot = self.create_ore_robot_node(blueprint, cache, max_elapsed);
        if ore_robot.is_some() {
            result.push(ore_robot.unwrap());
        }
        let clay_robot = self.create_clay_robot_node(blueprint, cache, max_elapsed);
        if clay_robot.is_some() {
            result.push(clay_robot.unwrap());
        }
        let obsidian_robot = self.create_obsidian_robot_node(blueprint, cache, max_elapsed);
        if obsidian_robot.is_some() {
            result.push(obsidian_robot.unwrap());
        }
        let geode_robot = self.create_geode_robot_node(blueprint, cache, max_elapsed);
        if geode_robot.is_some() {
            result.push(geode_robot.unwrap());
        }
        //let remaining = max_elapsed - self.elapsed; // ? 
        // for n in &mut result {
        //     //n.elapsed = next_elapsed;
        //     n.ore = min(n.ore, blueprint.spent_ore_rate*remaining);
        //     n.clay = min(n.clay, blueprint.obsidian_robot_clay_cost*remaining);
        //     n.obsidian = min(n.obsidian, blueprint.geode_robot_obsidian_cost*remaining);
        // }

        result
    }

    pub fn set_max_geode(&self, geode_level: &mut i32, start_timer: &Instant) -> bool {
        if self.geode > *geode_level {
            let duration = start_timer.elapsed();
            println!("Timer = {}, elapsed = {}, ore={}, clay={}, obsidian={}, geode={}", 
                duration.as_secs(),
                self.elapsed,
                self.ore,
                self.clay,
                self.obsidian,
                self.geode
            );
            *geode_level = self.geode;
        }
        false // always
    }
}

fn read_lines(input: &str) -> Vec<Blueprint> {
    let mut result: Vec<Blueprint> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let parsed = sscanf!(
            line, 
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            i32, i32, i32,i32, i32, i32, i32
        );
        let (id, 
            ore_ore_cost, 
            clay_ore_cost, 
            obsidian_ore_cost, 
            obsidian_clay_cost, 
            geode_ore_cost, 
            geode_obsidian_cost
        ) = parsed.unwrap();
        result.push(Blueprint{
            id: id,
            ore_robot_ore_cost: ore_ore_cost,
            clay_robot_ore_cost: clay_ore_cost,
            obsidian_robot_ore_cost: obsidian_ore_cost,
            obsidian_robot_clay_cost: obsidian_clay_cost,
            geode_robot_ore_cost: geode_ore_cost,
            geode_robot_obsidian_cost: geode_obsidian_cost,
            spent_ore_rate: max_spent_ore_rate(
                ore_ore_cost, 
                clay_ore_cost, 
                obsidian_ore_cost, 
                geode_ore_cost
            )
        });
    }
    result

}

fn solve_1(input: &str) -> i32 {
    let blueprints = read_lines(input);

    let max_elapsed = 32; //24 - part1
    let to_id = 3; //blueprints.len();

    let whole_start_timer = Instant::now();
    let mut overall = 1;
    for i in 0..to_id {
        let blueprint = &blueprints[i];
        let start = Node{
            elapsed: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            robot_ore: 1,
            robot_clay: 0,
            robot_obsidian: 0,
            robot_geode: 0
        };
        let start_timer = Instant::now();
        let mut geode_level = 0;
        let mut cache: HashSet<Node> = HashSet::new();

        let result = dfs(
            start,
            |n| n.neighbors(blueprint, max_elapsed, &mut cache),
            |n| -> bool {
                n.set_max_geode(&mut geode_level, &start_timer)
            }
        );

        let duration = start_timer.elapsed();
        if result.is_some() {
            println!("Elapsed: {}, result {}, geode_level= {geode_level}", duration.as_secs(), result.unwrap().len());
        }
        else {
            println!("Elapsed: {}, NOT FOUND, geode_level= {geode_level}", duration.as_secs());
        }
        //overall *= blueprint.id * geode_level;
        overall *=  geode_level;
    }
    let duration = whole_start_timer.elapsed();
    println!("WHOLE Elapsed: {}", duration.as_secs());
    overall

}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
}
