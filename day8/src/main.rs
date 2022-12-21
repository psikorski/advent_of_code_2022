fn solve(input: &str) -> usize {
    let width = input.find('\n').unwrap() - 1;
    let height = input.len() / (width + 1);
    println!("input.len {}", input.len());
    println!("height {}", height);
    println!("width {}", width);
    let raw_bytes = input.as_bytes();
    //println!("{:?}", raw_bytes);
    let mut max_points = 0; 
    for j in 1..height-1 {
        for i in 1..width-1 {
            let id = j*(width+2)+i;
            //println!("{i}x{j} = [{}] = {}", id, raw_bytes[id] as char);

            // LEFT
            //let mut seen = true;
            let mut sub_score_left = 0;
            let mut xl = i-1;
            //for x in (-1..i-1).rev() {
            while xl > 0 {
                let idx = j*(width+2)+xl;
                //println!("LEFT j{j}, i{i}, id{id}, x{xl}, idx{idx} [c{}]", raw_bytes[idx]);
                sub_score_left+=1;

                if raw_bytes[idx] >= raw_bytes[id] {
                    //println!("1. NIE WIDAC j{j}, i{i}, id{id}, x{xl}, idx{idx} {} >= {}", raw_bytes[idx] as char, raw_bytes[id] as char);
                    //seen = false;
                    break;
                }
                if xl == 0 {
                    break;
                }
                xl -= 1;
            }
            // if seen {
            //     println!("1. next tree");
            //     total_seen += 1;
            //     continue;
            // }
            // RIGHT
            //seen = true;
            let mut sub_score_right = 0;
            for x in i+1..width {
                let idx = j*(width+2)+x;
                //println!("RIGHT j{j}, i{i}, id{id}, x{x}, idx{idx} {}", raw_bytes[idx] as char);
                sub_score_right+=1;

                if raw_bytes[idx] >= raw_bytes[id] {
                    //println!("2. NIE WIDAC j{j}, i{i}, id{id}, x{x}, idx{idx} {} >= {}", raw_bytes[idx] as char, raw_bytes[id] as char);
                    //seen = false;
                    break;
                }
            }
            // if seen {
            //     println!("2. next tree");
            //     total_seen += 1;
            //     continue;
            // }
            // TOP
            //seen = true;
            let mut sub_score_top = 0;
            let mut yt = j-1;
            //for y in (0..j-1).rev() {
            while yt >0 {
                let idy = yt*(width+2)+i;
                //println!("TOP j{j}, i{i}, id{id}, x{yt}, idx{idy} {}", raw_bytes[idy] as char);
                sub_score_top +=1;

                if raw_bytes[idy] >= raw_bytes[id] {
                    //println!("3. NIE WIDAC j{j}, i{i}, id{id}, y{yt}, idy{idy} {} >= {}", raw_bytes[idy] as char, raw_bytes[id] as char);
                    //seen = false;
                    break;
                }
                if yt == 0 {
                    break;
                }
                yt -= 1;              
            }
            // if seen {
            //     println!("3. next tree");
            //     total_seen += 1;
            //     continue;
            // }
            //BOTTOM
            //seen = true;
            let mut sub_score_bottom = 0;
            for y in j+1..height {
                let idy = y*(width+2)+i;
                //println!("BOTTOM j{j}, i{i}, id{id}, x{y}, idx{idy} {}", raw_bytes[idy] as char);
                sub_score_bottom+=1;
                if raw_bytes[idy] >= raw_bytes[id] {
                    //println!("4. NIE WIDAC j{j}, i{i}, id{id}, y{y}, idy{idy} {} >= {}", raw_bytes[idy] as char, raw_bytes[id] as char);
                    //seen = false;
                    break;
                }
            }

            let total = sub_score_bottom*sub_score_left*sub_score_right*sub_score_top;
            //println!("score left {sub_score_left}, score right {sub_score_right}, score top {sub_score_top}, score bottom {sub_score_bottom} = total {total}");
            // if seen {
            //     println!("4. next tree");
            //     total_seen += 1;
            //     continue;
            // }
            if max_points < total {
                max_points = total;
                println!("{i}x{j} = [{}] = {}", id, raw_bytes[id] as char);
                println!("score left {sub_score_left}, score right {sub_score_right}, score top {sub_score_top}, score bottom {sub_score_bottom} = total {total}");
            }
        }
    }
    max_points
}

fn main() {
    let input = include_str!("../input.txt");
    let result = solve(input);
    println!("result = {result}");
}
