use std::collections::{HashMap, VecDeque};

use sscanf::sscanf;

//#[derive(PartialEq)] - can be used instead of matches!
#[derive(Debug, Clone, Copy)]
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

// could be also impl SignType {}
fn to_string(sign: &SignType) -> &str {
    match sign {
        SignType::Plus => " + ",
        SignType::Minus => " - ",
        SignType::Div => " / ",
        SignType::Mult => " * ",
    }
}

// could be also impl SignType {}
fn do_operation(first: i64, second: i64, sign: &SignType) -> i64 {
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
            monkeys.insert(monkey.name.clone(), monkey);
        }
    }
    monkeys
}

// choose_proper(value: i64, dep &str, monkeys: &MonkeysMap) {
//     if dep == "humn" {
//         println!("FOUND");
//     }
//     let monkey = &monkeys[dep];
// }

#[allow(dead_code)]
fn get_full_operation(dep: &str, monkeys: &MonkeysMap) -> String {
    let monkey = &monkeys[dep];

    if monkey.number.is_some() {
        return monkey.number.as_ref().unwrap().to_string();
    }
    let left_op = get_full_operation(monkey.op_first(), monkeys);
    let right_op = get_full_operation(monkey.op_second(), monkeys);
    let mut val = "(".to_owned();
    val.push_str(&left_op);
    val.push_str(to_string(monkey.op_sign())); 
    val.push_str(&right_op);
    val.push_str(")");
    val
}

fn get_dependent(dep: &str, monkeys: &MonkeysMap) -> (i64, bool) {
    let monkey = &monkeys[dep];
    let mut found = false;
    if monkey.name == "humn" {
        println!("Found");
        found = true;
    }
    if monkey.number.is_some() {
        return (monkey.number.as_ref().unwrap().clone(), found)
    }
    let left_op = get_dependent(monkey.op_first(), monkeys);
    let right_op = get_dependent(monkey.op_second(), monkeys);
    let val = do_operation(left_op.0, right_op.0, monkey.op_sign());
    if left_op.1 || right_op.1 {
        found = true;
    }
    (val, found)
}

struct QueuedOperation {
    pub sign: SignType,
    pub other: i64,
    pub left: bool,
}

fn opposite_sign(sign: &SignType) -> SignType {
    match sign {
        SignType::Plus => SignType::Minus,
        SignType::Mult => SignType::Div,
        SignType::Div => SignType::Mult,
        SignType::Minus => SignType::Plus,
    }
}

impl QueuedOperation {
    fn execute(&self, val: i64) -> i64 {
        if matches!(self.sign, SignType::Plus) || matches!(self.sign, SignType::Mult)  || self.left {
            return do_operation(val, self.other, &opposite_sign(&self.sign))
        }
        else if matches!(self.sign, SignType::Div) {
            return do_operation(self.other, val, &SignType::Div)
        }
        else if matches!(self.sign, SignType::Minus) {
            return do_operation(self.other, val, &SignType::Minus)
        }
        else {
            println!("Panic :) TODO input");
            return 0
        }
    }
}

fn choose_proper(dep: &str, monkeys: &MonkeysMap, queue: &mut VecDeque<QueuedOperation>) -> (i64, bool) {
    let monkey = &monkeys[dep];

    if monkey.name == "humn" {
        println!("Found");
        return (monkey.number.as_ref().unwrap().clone(), true)
    }

    if monkey.number.is_some() {
        return (monkey.number.as_ref().unwrap().clone(), false)
    }
    
    let left_op = choose_proper(monkey.op_first(), monkeys, queue);
    let right_op = choose_proper(monkey.op_second(), monkeys, queue);

    let mut res = left_op.0;
    if left_op.1 {
        res = right_op.0; // humn odnosi sie do lewej, a wiec biore prawy wynik
    }
    queue.push_front(QueuedOperation{sign: *monkey.op_sign(), other: res, left: left_op.1});
    
    if left_op.1 || right_op.1 {
        return (0, true) // obojetne jaka wartosc, bo nie bierzemy pod uwage
    }
    let val = do_operation(left_op.0, right_op.0, monkey.op_sign());
    return (val, false)
}

#[allow(dead_code)]
fn solve_1(input: &str) -> i64 {
    let mut monkeys = read_lines(input);

    let result = get_dependent("root", &mut monkeys);
    println!("Result: {}", result.0);
    0
}

fn solve_2(input: &str) -> i64 {
    let monkeys = read_lines(input);
    let root = &monkeys["root"];
    let left = get_dependent(root.op_first(), &monkeys);
    let right = get_dependent(root.op_second(), &monkeys);

    println!("Left {}: {}, Right {}: {}", left.0, left.1, right.0, right.1);

    let mut val = left.0;
    let mut child = root.op_second();
    if left.1 {
        val = right.0;
        child = root.op_first();
    }

    let mut queue: VecDeque<QueuedOperation> = VecDeque::new();
    let (a,b) = choose_proper(child, &monkeys, &mut queue);
    println!("A {a}, B {b}");

    while ! queue.is_empty() {
        let current = queue.pop_front().unwrap(); //or back?
        val = current.execute(val.clone());
    }


    val
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_2(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
