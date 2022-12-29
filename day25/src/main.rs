fn read_snafu_numbers(input: &str) -> Vec<&str> {
    let lines_vec: Vec<&str> = input.lines().collect();
    lines_vec
}

fn snafu_dig_into_dec(snafu: char) -> i64 {
    match snafu {
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        '0' => 0,
        _   => 0
    }
}
fn dec_into_snafu(dig: i64) -> char {
    match dig {
        1 => '1',
        2 => '2',
        -1 => '-',
        -2 => '=',
        0 => '0',
        _   => '0'
    }
}

fn solve_1(input: &str) -> i64 {
    let snafu_numbers = read_snafu_numbers(input);
    let base:i64 = 5;
    let mut whole_dec:i64 = 0;
    for snafu in &snafu_numbers {
        let mut dec_number:i64 = 0;
        let snafu_b: Vec<char> = snafu.chars().map(|c| c as char).collect();
        let len = snafu_b.len();
        println!("SNAFU({len}) = {:?}", snafu/*_b*/);
        for i in 0..len-1 {
            let pw = len - i -1;
            let dec = snafu_dig_into_dec(snafu_b[i]) * base.pow((pw) as u32);
            //println!("snafu({})*5.pow({}) = {dec}", snafu_dig_into_dec(snafu_b[i]), pw);
            dec_number += dec;
        }
        dec_number+=snafu_dig_into_dec(snafu_b[len-1]);
        whole_dec += dec_number;
        println!("\t {dec_number}");
    }
    let whole_dec_copy = whole_dec;
    let mut result: Vec<i64> = Vec::new();
    let mut add_one = 0;
    while whole_dec != 0 {
        let mut curr = add_one + whole_dec % 5;
        add_one = 0;
        if curr > 2 {
            curr -= 5;
            add_one = 1;
        }
        result.push(curr);
        whole_dec /= 5;
    }
    if add_one == 1 {
        let last = result.len() - 1;
        result[last]+=1;
        if result[last] > 2 {
            result[last] -= 5;
            result.push(1);
        }
    }

    
    let res_snafu: Vec<u8> = result.iter().rev().map(|i| dec_into_snafu(*i) as u8).collect();
    let res_string = std::str::from_utf8(&res_snafu);
    println!("dec = {whole_dec_copy}, RESULT({}): {:#?}", result.len(), res_string);

    whole_dec
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
