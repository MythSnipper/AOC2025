
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
    .map(|vel| vel.trim())
    .map(|vel| vel.to_string())
    .map(|vel| vel.replace("  ", " "))
    .map(|vel| vel.replace("  ", " "))
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

     


    return format!("{}", ret);
}