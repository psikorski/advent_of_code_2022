// fn unique(s: &str) -> Option<(usize, usize, char)> {
//     s.chars().enumerate().find_map(|(i, c)| {
//         s.chars()
//             .enumerate()
//             .skip(i + 1)
//             .find(|(_, other)| c == *other)
//             .map(|(j, _)| (i, j, c))
//     })
// }
// match unique(chunk) {
//     None => println!("{} is unique", chunk),
//     Some((i, j, c)) => println!(
//         "{} is not unique\n\tfirst duplicate: \"{}\" (U+{:0>4X}) at indices {} and {}",
//         chunk, c, c as usize, i, j
//     ),
// }
use std::collections::HashSet;
const BLOCK_SIZE: usize = 14;
fn uniqueness(arg: &str) -> bool {
    let mut set = HashSet::with_capacity(BLOCK_SIZE);
    for s in arg.as_bytes() {
        if set.insert(s) == false {
            return false
        }
    }
    return true
}

fn solve(input: &str) -> u32 {
    let lines = input.lines();
    for line in lines {
        for i in 0..line.len()-BLOCK_SIZE+1 {
            let chunk = &line[i..i+BLOCK_SIZE];
            if uniqueness(chunk) {
                println!("chunk = {}, i+BLOCK_SIZE = {}", chunk, i+BLOCK_SIZE);
                break;
            }

        }
    }  
    6
}

fn main() {
    let input = include_str!("../input.txt");

    let result = solve(input);

    println!("result = {result}");
}
