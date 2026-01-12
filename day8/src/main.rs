
#![recursion_limit = "2000"]
fn main() {
    let input = include_str!("test_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;
    
    let input: Vec<(u64, u64, u64)> = input
    .lines()
    .map(|vel| vel.split(','))
    .map(|mut vel| (vel.next().unwrap(), vel.next().unwrap(), vel.next().unwrap()))
    .map(|vel| (vel.0.parse::<u64>().unwrap(), vel.1.parse::<u64>().unwrap(), vel.2.parse::<u64>().unwrap()))
    .map(|vel| (vel.0, vel.1, vel.2))
    .collect();

    for i in input{
        println!("{:?}", i);
    }

    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    "I love you vel <3".to_string()
}

