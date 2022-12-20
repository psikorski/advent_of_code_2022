use std::cmp::min;
use std::cmp::Ordering;

#[derive(Debug)]
struct Element{
    pub elements: Vec<Box<Element>>,
    pub value: Option<i32>,
}
impl Element {
    fn simple_val(val: i32) -> Self {
        Element{elements: Vec::new(), value: Some(val)}
    }
    fn copyy(self: &Element) -> Self {
        let mut result = Element{elements: Vec::new(), value: Option::None};
        if self.value.is_some() {
            result.value = Some(self.value.unwrap());
            return result
        }
        for el in &self.elements {
            result.elements.push(Box::new(el.copyy()));
        }
        result
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
            let mut end: usize;
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

#[allow(dead_code)]
fn print_el(elem: &Element) {
    if elem.value.is_some() {
        println!("READ Value: {}", elem.value.unwrap());
    }
    else {
        println!("READ: {:?}", elem.elements);
    }
}


fn compare(el1: &Element, el2: &Element) -> Ordering {
    let mut mel1 = el1.copyy();
    let mut mel2 = el2.copyy();
    //println!("compare: \n\t{:?}\n\t{:?}", mel1, mel2);
    let res = compare_internal(&mut mel1, &mut mel2);
    //println!("{:?}", res);
    res
}

fn compare_internal(el1: &mut Element, el2: &mut Element) -> Ordering {
    let val1_b = el1.value.is_some();
    let val2_b = el2.value.is_some();
    
    if val1_b && val2_b {
        let el1_v = el1.value.unwrap();
        let el2_v = el2.value.unwrap();
        if el1_v < el2_v {
            return Ordering::Less
        }
        else if el1_v > el2_v {
            return Ordering::Greater
        }
        return Ordering::Equal
    }
    else if val1_b && !val2_b {
        el1.elements.push(Box::new(Element::simple_val(el1.value.unwrap())));
        el1.value = Option::None;
        return compare_internal(el1, el2)
    }
    else if !val1_b && val2_b {
        el2.elements.push(Box::new(Element::simple_val(el2.value.unwrap())));
        el2.value = Option::None;
        return compare_internal(el1, el2)
    }
    else {
        let mut id = 0;
        let len1 = el1.elements.len();
        let len2 = el2.elements.len();
        loop {
            if id >= len1 && id >= len2 {
                return Ordering::Equal // FOR PART TWO
            }
            if id >= len1 { //LEFT run out of items (RIGHT does not matter)
                return Ordering::Less
            }
            if id < len1 && id >= len2 { //RIGHT run out of items
                return Ordering::Greater
            }
            let cmp_res = compare_internal(&mut el1.elements[id], &mut el2.elements[id]);
            if cmp_res == Ordering::Equal {
                id += 1;
                continue
            }
            else {
                return cmp_res
            }
            
        }
    }
}

fn solve_1(input: &str) -> usize {
    let lines_vec: Vec<&str> = input.lines().collect();
    let lines_len = (lines_vec.len()+1)/3;
    let mut pair_id = 0;

    let mut result = 0;
    while pair_id < lines_len {
        let mut id = 1;
        let el1 = read_el(&lines_vec[3*pair_id], &mut id);
        id = 1;
        let el2 = read_el(&lines_vec[3*pair_id+1], &mut id);
        pair_id+=1; // TODO consider replacing with for loop
        let cmp_res = compare(&el1, &el2);
        match cmp_res {
            Ordering::Less => result+=pair_id,
            Ordering::Equal => result+=pair_id,
            _=>(),
        }
        println!("{pair_id} el1 < el2 {:?}", cmp_res);
    }
    result
}

fn solve_2(input: &str) -> usize {
    let lines_vec: Vec<&str> = input.lines().collect();
    let lines_len = (lines_vec.len()+1)/3;

    let mut elements: Vec<Element> = Vec::new();

    let mut pair_id = 0;

    let mut result = 0;
    while pair_id < lines_len {
        let mut id = 1;
        let el1 = read_el(&lines_vec[3*pair_id], &mut id);
        id = 1;
        let el2 = read_el(&lines_vec[3*pair_id+1], &mut id);
        pair_id+=1; // TODO consider replacing with for loop
        elements.push(el1);
        elements.push(el2);
    }
    let mut id: usize = 1;
    let raw_el2 = read_el("[[2]]", &mut id);
    id = 1;
    let raw_el6 = read_el("[[6]]", &mut id);
    elements.push(raw_el2.copyy());
    elements.push(raw_el6.copyy());

    elements.sort_by(|a, b| compare(a, b));
    //elements.iter().for_each(|x| println!("{:?}", x));

    let p2 = elements.iter().position(|el| compare(el, &raw_el2) == Ordering::Equal);
    let p6 = elements.iter().position(|el| compare(el, &raw_el6) == Ordering::Equal);
    println!("FOUND: {:?}", p2);
    println!("FOUND: {:?}", p6);
    (p2.unwrap()+1)*(p6.unwrap()+1)
}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_2(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
