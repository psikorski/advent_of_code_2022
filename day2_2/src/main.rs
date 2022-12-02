fn solve(input: &str) -> usize {
    let lines = input.lines();
    let mut total_points = 0;
    for line in lines {
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        let loose = 0;
        let draw = 3;
        let win = 6;
    
        match line{
            "A X"=>total_points+= scissors + loose,
            "A Y"=>total_points+= rock + draw,
            "A Z"=>total_points+= paper + win,
            "B X"=>total_points+= rock + loose,
            "B Y"=>total_points+= paper + draw,
            "B Z"=>total_points+= scissors + win,
            "C X"=>total_points+= paper + loose,
            "C Y"=>total_points+= scissors + draw,
            "C Z"=>total_points+= rock + win,
            _=>println!("unknown '{line}'"),
        }
    }
    total_points
}

fn main() {
    let input = include_str!("../input.txt");

    let result = solve(input);

    println!("result = {result}");
}
