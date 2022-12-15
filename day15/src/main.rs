use sscanf::sscanf;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;

struct Point{
    pub x: i32,
    pub y: i32,
}

struct SensorBeacon {
    pub sensor: Point,
    pub beacon: Point,
    pub distance: i32,
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p2.y - p1.y).abs() + (p2.x - p1.x).abs()
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Interval {
    pub start: i32,
    pub dist: i32,
}

// fn cmp(in1: &Interval, in2: &Interval) -> Ordering {
//     if in1.start == in2.start {
//         in1.dist.cmp(&in2.dist)
//     }
//     in1.start.cmp(&in2.start)
// }

fn parse_line(line: &str) -> (Point, Point) {
    let parsed = 
        sscanf!(
            line, 
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
             i32, i32, i32, i32
        );
    let (sx, sy, bx, by) = parsed.unwrap();
    let sensor = Point{x: sx, y: sy};
    let beacon = Point{x: bx, y: by};
    (sensor, beacon)
}

fn solve_1(input: &str) -> usize {
    let result: i64 = 0;
    //let interesting_row = 2000000;
    //let interesting_row = 10;
    let mut sb_vector: Vec<SensorBeacon> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let (sensor, beacon) = parse_line(line);
        let dist = distance(&sensor, &beacon);
        sb_vector.push(SensorBeacon { sensor: sensor, beacon: beacon, distance: dist });
    }
    let max_row = 4000000;
    for interesting_row in 3289728..max_row {
        //let interesting_row = 163538;
        if interesting_row % 1000 == 0 {
            println!("interesting_row {interesting_row}");
        }
        let mut intervals: Vec<Interval> = Vec::new();
        for sb in &sb_vector {
            let dist_to_row = (sb.sensor.y - interesting_row).abs();
            if dist_to_row <= sb.distance {
                let start =  sb.sensor.x - (sb.distance - dist_to_row);
                let finish = sb.sensor.x + (sb.distance - dist_to_row);
                //println!("st {start} fn {finish} == sb.sensor.x {}, sb.distance {}, dist_to_row {dist_to_row}", sb.sensor.x, sb.distance);
                intervals.push(Interval { start: max(0, start), dist: min(max_row, finish) });
            }
        }
        intervals.sort();
        let mut found = false;
        let mut min_found = intervals[0].start;
        let mut max_found = intervals[0].dist;
        for id in 1..intervals.len() {
            //println!(" id ({}, {})",intervals[id].start, intervals[id].dist); 
            if intervals[id].start > max_found {
                println!("FOUND interesting_row {interesting_row}, min {min_found} max {max_found} start {} dist {}", intervals[id].start, intervals[id].dist);
                found = true;
                break;
            }
            max_found = max (max_found, intervals[id].dist);
        }
         if found {
             break;
         }
    }
    3403960*4000000+3289729
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
