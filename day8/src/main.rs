
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;
    
    let input: Vec<(u64, u64, u64)> = input
    .lines()
    .map(|vel| vel.split(','))
    .map(|vel| (vel.next(), vel.next(), vel.next()))
    .collect()

    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    "I love you vel <3".to_string()
}

