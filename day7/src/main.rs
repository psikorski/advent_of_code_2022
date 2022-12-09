/*pub struct File<'a> {
    pub name: String,
    pub size: usize,
    pub parent: Option<&'a Dir<'a>>, 
}
pub struct Dir<'a> {
    pub name: String,
    pub files: Vec<File<'a>>,
    pub dirs: Vec<Dir<'a>>,
    pub parent: Option<&'a Dir<'a>>, 
}
*/
use nom::{IResult, character::complete::digit1, combinator::map_res};
use std::collections::HashMap;

fn parser(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
  }

fn solve(input: &str) -> u64 {
    let lines = input.lines();
    let mut dirs: HashMap<String, u64> = HashMap::new();
    let mut current_dir = String::new();
    let mut process_tree: Vec<String>= Vec::new();
    for line in lines {
        if line.starts_with("$ cd ..") {
            //unsafe {
                let current_dir_copy = current_dir.clone();
                let current_dir_size = *dirs.get(&current_dir_copy).unwrap();
                let parent_dir = process_tree.pop().unwrap();
                let parent_dir_size = *dirs.get(&parent_dir).unwrap();
                dirs.insert(parent_dir.clone(), parent_dir_size + current_dir_size);
                println!("cd .. ({current_dir}/{current_dir_size} -> {parent_dir}/{parent_dir_size})");
                current_dir = parent_dir;
            //}
        }
        else if line.starts_with("$ cd ") {
            println!("'{line}'");
            if line == "$ cd /" {
                current_dir = line.to_string();
            }
            else {
                process_tree.push(current_dir.clone());
                current_dir = current_dir +"/"+ line;
            }
            dirs.entry(current_dir.clone()).or_insert(0);
        }
        else if line == "$ ls" {
           // println!("skipping the command '{line}'");
        }
        else if line.starts_with("dir ") {
           // println!("skipipping the command '{line}'");
        }
        else {
            let (_, size) = parser(line).unwrap();
            //dirs.entry(current_dir).or_insert(0);
            let val = dirs.get_mut(&current_dir).unwrap();
            *val += size;
            println!("adding the {size} size (total: {} of file '{line}' in dir {current_dir}", val);
        }
    }
    loop {
        let parent_dir = process_tree.pop();
        if parent_dir.is_none() {
            break;
        }

        let current_dir_size = dirs[&current_dir];
        let key = parent_dir.unwrap();        
        let parent_dir_size = dirs[&key];
        dirs.insert(key.clone(), parent_dir_size + current_dir_size);

        //println!("cd .. ({current_dir}/{current_dir_size} -> {}/{parent_dir_size})", parent_dir.unwrap().as_ref());
        current_dir = key.clone();

    }
    let total_size = dirs["$ cd /"];
    let to_free = 30000000 - (70000000 - total_size);
    // println!("dirs: {}", dirs.len());
    // for d in dirs {
    //     println!("{}: {}", d.0, d.1);
    //     total_size += d.1;
    // }
    *dirs.values().filter(|value| **value >= to_free).min().unwrap()
}


fn main() {
    let input = include_str!("../input.txt");
    let result = solve(input);
    println!("result = {result}");
}


