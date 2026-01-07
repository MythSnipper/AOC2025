
fn main() {
    let input = include_str!("test_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;

    let mut input: Vec<String> = input
    .lines()
    .map(|vel| vel.trim())
    .map(|vel| vel.to_string())
    .map(|vel| vel.replace("  ", " "))
    .map(|vel| vel.replace("  ", " "))
    .map(|vel| vel.replace("  ", " "))
    .map(|vel| vel.replace("  ", " "))
    .map(|vel| vel.replace("  ", " "))
    .collect();

    let operations: Vec<char> = input
    .pop()
    .unwrap()
    .split(' ')
    .map(|vel| vel.parse::<char>().unwrap())
    .collect();

    let mut numbers: Vec<Vec<u64>> = input
    .iter()
    .fold(Vec::new(), |mut acc, vel| {
        acc.push(
            vel
        .split(' ')
        .into_iter()
        .map(|vel| vel.trim())
        .map(|vel| vel.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        );
        acc
    });

    for i in 0..operations.len(){
        let op = operations[i];

        let mut acc: u64 = if op == '*'{1}else if op == '+'{0}else{panic!()};
        for uwu in 0..numbers.len(){
            acc = if op == '*'{acc * numbers[uwu][i]}else if op == '+'{acc + numbers[uwu][i]}else{panic!()};
        }
        ret += acc;

        //println!("{}", acc);
    }
    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    let mut ret: u64 = 0;

    let mut input: Vec<String> = input
    .lines()
    .map(|vel| {
        let ret: String = vel.chars().rev().collect();
        ret + " "
    })
    .collect();

    let mut operations: Vec<char> = input
    .pop()
    .unwrap()
    .trim()
    .replace("  ", " ")
    .replace("  ", " ")
    .replace("  ", " ")
    .replace("  ", " ")
    .replace("  ", " ")
    .replace("  ", " ")
    .replace("  ", " ")
    .split(' ')
    .map(|vel| vel.parse::<char>().unwrap())
    .collect();

    let input: Vec<Vec<char>> = input
    .iter()
    .map(|vel| vel.chars().collect())
    .collect();

    let mut i: usize = 0;
    for op in &operations{
        let operands = solve_part2_helper2(&input, &mut i);
        let acc: u64 = if *op == '*'{1}else if *op == '+'{0}else{panic!()};
        for o in operands{
            print!("{}, ", o);
        }
        println!("  {}\n", op);
    }

    return format!("{}", ret);
}

//determines if all lines in input are spaces
fn solve_part2_helper1(input: &Vec<Vec<char>>, index: &mut usize) -> bool {
    for line in input{
        if line[*index] != ' '{
            println!("Nein!");
            return false;
        }
    }
    println!("Ja!");
    return true;
}
//scans for one problem
fn solve_part2_helper2(input: &Vec<Vec<char>>, index: &mut usize) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();
    while !solve_part2_helper1(input, index){
        let mut num: String = String::new();
        for line in input{
            num.push(line[*index]);
        }
        ret.push(num
            .trim()
            .parse::<u64>()
            .unwrap()
        );
        *index += 1;
    }
    *index += 1;
    ret
}