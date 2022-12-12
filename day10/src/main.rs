fn parse_int(input: &str) -> i32 {
    let ret: i32 = input.trim().parse().unwrap();
    ret
}

fn parse_line(line: &str) -> i32 {
    // omitting "ADDX "
    let number = parse_int(&line[4..]);
    number
}

pub struct RegAtCycle {
    pub cycle: i32,
    pub register: i32,
}

fn is_wanted_cycle(cycle: i32, values: &Vec<RegAtCycle>) -> i32 {
    for i in 0..values.len() {
        //println!("is wanted {} == {cycle}", values[i].cycle);
        if values[i].cycle == cycle {
            return i as i32
        }
    }
    -1
}

fn solve_1(input: &str) -> i32 {
    let mut cycle = 0;
    let mut register = 1;
    let mut values: Vec<RegAtCycle> = Vec::new();
    values.push(RegAtCycle{cycle: 20, register: 0});
    values.push(RegAtCycle{cycle: 60, register: 0});
    values.push(RegAtCycle{cycle: 100, register: 0});
    values.push(RegAtCycle{cycle: 140, register: 0});
    values.push(RegAtCycle{cycle: 180, register: 0});
    values.push(RegAtCycle{cycle: 220, register: 0});
    let lines = input.lines();
    let mut pixel_buffer = vec!['.' as u8; 40];
    for line in lines {
        cycle += 1;
        let mut pix_number = (cycle - 1) % 40;
        if (register as i32 - pix_number).abs() <= 1 {
           // println!(">> cycle {cycle}, pix_number {pix_number}, register {register}");
            pixel_buffer[pix_number as usize] = '#' as u8;
        }
        else {
           // println!("cycle {cycle}, pix_number {pix_number}, register {register}");
        }
        if cycle % 40 == 0 {
            let s = std::str::from_utf8(&pixel_buffer);
            println!("{:?} , cycle {}", s, cycle);
            pixel_buffer = vec!['.' as u8; 40];
        }
        let i = is_wanted_cycle(cycle, &values);
        if i != -1 {
           // println!("FOUND+2 {cycle} cycle = {register}");
            values[i as usize].register = register;
            //break;
        }
        
        if line == "noop" {
           // println!("cycle{cycle} noop, register {register}, i {i}");
            continue;
        }
        else {
            let a = parse_line(line);
           // println!("cycle{cycle}, register {register} addx {a}, i {i}");
            
            cycle += 1;
            pix_number = (cycle - 1) % 40;
            if (register as i32 - pix_number as i32).abs() <= 1 {
                //println!("> cycle {cycle}, pix_number {pix_number}, register {register}");
                pixel_buffer[pix_number as usize] = '#' as u8;
            }
            else {
                //println!("cycle {cycle}, pix_number {pix_number}, register {register}");
            }
            let j = is_wanted_cycle(cycle, &values);
            if cycle % 40 == 0 {
                let s = std::str::from_utf8(&pixel_buffer);
                println!("{:?} , cycle {}", s, cycle);
                pixel_buffer = vec!['.' as u8; 40];
            }
            if j >= 0 {
               // println!("FOUND+2 {cycle} cycle = {register}");
                values[j as usize].register = register;
                //break;
            }
            register += a;
        }
    }
    let mut s = 0;
    for v in values {
        let temp  = v.register*v.cycle;
        s += temp;
        //println!("SUM {s} {temp} = {}x{}", v.register, v.cycle);
    }
    s
    //values.iter().map(|&i| i.register as i32).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
