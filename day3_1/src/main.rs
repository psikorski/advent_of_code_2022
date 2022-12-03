fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut total_points: u32 = 0;

    for line in lines {
        let comp_size = line.len() / 2;
        let second = &line[comp_size..line.len()];
        for n in 0..comp_size {
            let c = line.chars().nth(n).unwrap();
            let found = second.find(c);
            if ! found.is_none() {
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
    total_points
}

fn main() {
    let input = include_str!("../input.txt");

    let result = solve(input);

    println!("result = {result}");
}
