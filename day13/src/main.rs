use std::cmp::min;
use std::fmt;
enum ElementType {
    Int,
    Vector,
}
#[derive(Debug)]
struct Element{
    pub elements: Vec<Box<Element>>,
    pub value: Option<i32>,
}
impl Element {
    fn simple_val(val: i32) -> Self {
        Element{elements: Vec::new(), value: Some(val)}
    }
}

fn read_el(line: &str, id: &mut usize) -> Element {
    let mut result = Element{elements: Vec::new(), value: Option::None};
    while *id < line.len() {
        let current_char = line.as_bytes()[*id] as char;
        //println!("id: {id}");
        if current_char == '[' {
            *id+=1;
            result.elements.push(Box::new(read_el(&line, id)));
            *id+=1;
        }
        else if current_char == ']' {
            return result
        }
        else if current_char == ',' {
            *id+=1;
            continue;
        }
        else {
            let comma = line[*id..].find(',');
            let closing_brackets = line[*id..].find(']');
            let mut end = 0;
            if comma.is_none() && closing_brackets.is_some() {
                end = closing_brackets.unwrap();
            }
            else if comma.is_some() && closing_brackets.is_none() {
                end = comma.unwrap();
            }
            else {
                end = min(comma.unwrap(), closing_brackets.unwrap());
            }
            end += *id;
            let val = line[*id..end].parse::<i32>().unwrap();
            result.elements.push(Box::new(Element::simple_val(val)));
            *id = end;
        }
    }
    result
}

fn print_el(elem: &Element) {
    if elem.value.is_some() {
        println!("READ Value: {}", elem.value.unwrap());
    }
    else {
        println!("READ: {:?}", elem.elements);
    }
}

fn compare(el1: &Element, el2: &Element) -> bool{
    let val1_b = el1.value.is_some();
    let val2_b = el1.value.is_some();
    
    if el1.value.is_some() && el2.value
}

fn solve_1(input: &str) -> usize {
    let lines_vec: Vec<&str> = input.lines().collect();
    let lines_len = (lines_vec.len()+1)/3;
    let mut pair_id = 0;
    
    //0*3, 1*3,  2*3  3*3
    //0,1, 3,4, ,6,7, 9,10

    // read and check pairs
    while pair_id < lines_len {
        let mut id = 1;
        let el1 = read_el(&lines_vec[3*pair_id], &mut id);
        id = 1;
        //print_el(&el1);
        let el2 = read_el(&lines_vec[3*pair_id+1], &mut id);
        //print_el(&el2);
        pair_id+=1; // TODO consider replacing with for loop
    }
0
}

fn main() {
    let input = include_str!("../input_sample.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
