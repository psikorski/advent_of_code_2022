use sscanf::sscanf;

pub struct Object {
    pub order: usize,
    pub value: i64,
}

impl Object {
    pub fn new(o: usize, v: i64) -> Self {
        Object{order: o, value: v}
    }
}

type Objects = Vec<Object>;

fn read_lines(input: &str) -> Objects {
    let mut values: Objects = Vec::new();
    let mixer = 811589153;
    //let mut ordered: Objects = Vec::new();
    let lines = input.lines();
    let mut order = 0 as usize;
    for line in lines {
        let parsed = sscanf!(line, "{}", i64);
        values.push(Object::new(order, parsed.unwrap()*mixer));
        //values.push(parsed.unwrap());
        //ordered.push(order);
        order += 1;
    }
    values
}

fn solve_1(input: &str) -> i64 {
    let mut values = read_lines(input);
    let mut values_placements: Vec<usize> = Vec::new();
    let len = values.len();
    let leni = len as i64;

    for id in 0..len{
        values_placements.push(id);
    }

    print!("Initial: ");
    for v in &values {
        print!("{} ", v.value);
    }
    print!("\n");

    println!("Len {len}");
    for round in 0..10 {
        for id in 0..len {
            //println!("\nid == {id}, val {}", values[id].value);

            let moves = values[id].value;
            if moves == 0 {
                // nothing
                continue;
            }

            let current_placement = values[id].order as i64;
            let mut new_id = current_placement + moves;
            
            let removed = values_placements.remove(current_placement as usize);        
            for next_id in (current_placement as usize)..len-1 {
                values[values_placements[next_id]].order = next_id;
            }

            new_id %= (leni-1);
            //print!("new_id: ({new_id})\n\t");
            if new_id == 0 {
                //print!("new_id == 0\n");
                new_id = leni -1;
            }
            else if new_id == leni - 1 {
                //print!("new_id == leni -1\n");
                println!("TODO??");
            }
            else if new_id < 0 {
                //print!("new_id({new_id}) < 0");
                //new_id %= (leni-1);
                //while new_id < 0 {
                    //print!(" => new_id({new_id}) < 0");
                    new_id = leni - 1 + new_id;
                    //print!("\t ==> new_id({new_id})\n");
                //}
            }
            else if new_id > leni - 1 { // TODO -1 or -2
                //print!("new_id({new_id}) > len -1 ");
                //new_id %= (leni-1);
                //while new_id > leni -1 {
                    //print!(" => new_id({new_id}) > len -1 ");
                    //new_id = new_id - leni + 1;
                    //print!("\t ==> new_id({new_id})\n");
                //}
            }
            values_placements.insert(new_id as usize, removed);

            for next_id in new_id as usize..len {
                values[values_placements[next_id]].order = next_id ;
            } 
        }

        print!("After {round} round: ");
        for v in &values_placements {
            print!("{} ", values[*v].value);
        }
        print!("\n");
    }

    let mut th1000 = 1000; 
    let mut th2000 = 2000;
    let mut th3000 = 3000;

    let zero = values.iter().find(|x| x.value == 0);
    let zero_id = zero.unwrap().order;
    th1000 += zero_id;
    th2000 += zero_id;
    th3000 += zero_id;

    th1000 %= len;
    th2000 %= len;
    th3000 %= len;

    let th1000_el = values[values_placements[th1000]].value;
    let th2000_el = values[values_placements[th2000]].value;
    let th3000_el = values[values_placements[th3000]].value;
    println!(
        "[{th1000}] = {}, [{th2000}] = {}, [{th3000}] = {} ==> sum = ", 
        th1000_el,
        th2000_el,
        th3000_el
    );
    th1000_el + th2000_el + th3000_el

}

fn main() {
    let input = include_str!("../input.txt");
    let result1 = solve_1(input);
    println!("result1 = {result1}");
    //let result2 = solve_2(input);
    //println!("result2 = {result2}");
}
