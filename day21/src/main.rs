use std::collections::HashMap;

use sscanf::sscanf;

#[derive(Debug)]
enum SignType{
    Plus,
    Minus,
    Mult,
    Div,
}

fn to_sign_type(sign: &str) -> SignType {
    match sign {
        " + " => SignType::Plus,
        " - " => SignType::Minus,
        " * " => SignType::Mult,
        " / " => SignType::Div,
        _ => SignType::Div // not neeeded
    }
}

fn do_operation(first: i64, second: i64, sign: &SignType) -> i64 {
    println!("{} {:#?} {}",first, sign, second);
    match sign {
        SignType::Plus => first + second,
        SignType::Minus => first - second,
        SignType::Div => first / second,
        SignType::Mult => first * second,
    }
}

struct Operation{
    pub sign: SignType,
    pub first: String, // TODO maybe reference?
    pub second: String,
}

struct Monkey {
    pub name: String,
    pub number: Option<i64>,
    pub operation: Option<Operation>,
}

type MonkeysMap = HashMap<String, Monkey>;

impl Monkey {
    fn as_number(aname:&str, anumber: i64) -> Self {
        Monkey{name: aname.to_string(), number: Some(anumber), operation: Option::None}
    }

    fn as_operation(aname: &str, asign: &str, afirst: &str, asecond: &str) -> Self {
        Monkey { 
            name: aname.to_string(), 
            number: Option::None, 
            operation: Some(
                Operation{
                    sign: to_sign_type(asign), 
                    first: afirst.to_string(),
                    second: asecond.to_string()
                }
            )
        }
    }
    fn op_sign(&self) -> &SignType {
        &self.operation.as_ref().unwrap().sign
    }
    fn op_first(&self) -> &str {
        &self.operation.as_ref().unwrap().first
    }
    fn op_second(&self) -> &str {
        &self.operation.as_ref().unwrap().second
    }
}

fn read_lines(input: &str) -> MonkeysMap {
    let mut monkeys: MonkeysMap = HashMap::new();
    let lines = input.lines();
    for line in lines {

        let mut name_sth = line.split(": ");
        let name = name_sth.next();
        let sth_wrapped = name_sth.next();
        let sth = sth_wrapped.unwrap();
        let possible_number = sscanf!(sth, "{}",i64);
        if possible_number.is_ok() {
            let monkey = 
                Monkey::as_number(name.unwrap(), possible_number.unwrap());
            //println!("name: {}, simple val {:?}", monkey.name, monkey.number);
            monkeys.insert(monkey.name.clone(), monkey);
        }
        else {
            let ops = vec![" + ", " - ", " * ", " / "];
            let mut first = Option::None;
            let mut second = Option::None;
            let mut operation= " ? ";
            for op in ops {
                if sth.find(op).is_none() {
                    continue;
                }
                let mut operands = sth.split(op);
                first = operands.next();
                second = operands.next();
                operation = op;
                break;
            }
            let monkey = 
                Monkey::as_operation(
                    name.unwrap(), 
                    operation, 
                    first.unwrap(), 
                    second.unwrap()
                );
            // println!("name: {}, simple val {:?}, operation {:#?}, f {}, s {}",
            //     monkey.name,
            //     monkey.number,
            //     monkey.op_sign(),
            //     monkey.op_first(),
            //     monkey.op_second()
            // );
            monkeys.insert(monkey.name.clone(), monkey);
        }
    }
    monkeys
}

fn get_dependent(dep: &str, monkeys: &MonkeysMap) -> i64 {
    let monkey = &monkeys[dep];

    if monkey.number.is_some() {
        return monkey.number.as_ref().unwrap().clone()
    }
    let left_op = get_dependent(monkey.op_first(), monkeys);
    let right_op = get_dependent(monkey.op_second(), monkeys);
    let val = do_operation(left_op, right_op, monkey.op_sign());
    //monkey.number = Some(val);
    val
}

fn solve_1(input: &str) -> i64 {
    let mut monkeys = read_lines(input);

    let result = get_dependent("root", &mut monkeys);

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
