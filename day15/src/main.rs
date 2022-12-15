use sscanf::sscanf;
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
    for interesting_row in 0..max_row {
        println!("interesting_row {interesting_row}");
        let mut cannot: HashSet<i32> = HashSet::new();
        let mut left_min = -1;
        let mut right_max = -1;
        // mut beacons_in_row: HashSet<i32> = HashSet::new();
        // let lines = input.lines();
        // for line in lines {
        //     let (sensor, beacon) = parse_line(line);
        //     let dist = distance(&sensor, &beacon);
            //println!("sensor: {}x{}, beacon {}x{} = distance={dist}", sensor.x, sensor.y, beacon.x, beacon.y);
            //if beacon.y == interesting_row {
              //  beacons_in_row.insert (beacon.x);
            //}
        for sb in &sb_vector {
            let cannot_len_inner_1 = cannot.len();
            if cannot_len_inner_1 as i32 >= max_row - 1 {
                println!("cannot_len_inner_1");
                break;
            }
            let dist_to_row = (sb.sensor.y - interesting_row).abs();
            if dist_to_row <= sb.distance {
                //println!("CLOSE {dist_to_row} <= {dist}");
                cannot.insert(sb.sensor.x);
                if dist_to_row == sb.distance {
                    println!("cannot_len_inner");
                    continue;
                }
                // if sb.sensor.x - sb.distance + dist_to_row < 0 {
                //     println!("dist_to_row - sb.sensor.x");
                //     break;
                // }
                // if sb.sensor.x + sb.distance - dist_to_row >= max_row {
                //     println!("dist_to_row + sb.sensor.x");
                //     break;
                // }
                left_min = min(sb.sensor.x - (dist_to_row - sb.distance), left_min);
                right_max= 
                // for i in 1..(dist_to_row - sb.distance).abs()+1 {
                //     let cannot_len_inner = cannot.len();
                //     if cannot_len_inner as i32 >= max_row - 1 {
                //         break;
                //     }
                //     let temp1 = sb.sensor.x - i;
                //     if temp1 > 0 && temp1 < max_row {
                //         cannot.insert(temp1);
                //     }
                //     let temp2 = sb.sensor.x + i;
                //     if temp2 > 0 && temp2 < max_row {
                //         cannot.insert(temp2);
                //     }
                // }
            }
            else {
                //println!("FAR {dist_to_row} > {dist}");
            }
        }
        
        //println!("beacons_in_row {}", beacons_in_row.len());
        //for b in beacons_in_row {
        //    cannot.remove(&b);
        //}
        let mut found = false;
        let cannot_len = cannot.len();
        println!("cannot len {cannot_len}");
        if cannot_len as i32 >= max_row - 1 {
            continue;
        }
        for c in 1..max_row {
            if ! cannot.contains(&c) {
                println!("FOUND c {c} int_row {interesting_row} sum {}", c*max_row+interesting_row);
                found = true;
            }
        }
        if found {
            break;
        }
    }
    0
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
