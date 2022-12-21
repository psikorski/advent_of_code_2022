fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut lines_copy = input.lines();
    lines_copy.next();
    let mut total_points: u32 = 0;
    let mut id = 0;
    for line in lines {
        if id % 3 != 0 {
            println!("id {id}");
            id+=1;
            continue;
        }
        id+=1;

        let line1 = lines_copy.next().unwrap();
        let line2 = lines_copy.next().unwrap();
        lines_copy.next();
        println!("line: {line}, line1: {line1}, line2 {line2}");
        
        for c in line.chars() {
            if ! line1.find(c).is_none() {
                if ! line2.find(c).is_none() {
                    let number = (c as u8) as u32;
                    if c.is_ascii_lowercase() {
                        total_points += number - 96;
                    }
                    else {
                        total_points += number - 65 + 27;
                    }
                    break;
                }
            }
        }
       println!("total_points = {total_points}");
    }
    total_points
}

fn main() {
    let input = include_str!("../input_sample.txt");

    let result = solve(input);

    println!("result = {result}");
}
