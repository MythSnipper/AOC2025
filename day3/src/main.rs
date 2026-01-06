
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;

    let input: Vec<String> = input
    .lines()
    .map(|vel| vel.to_string())
    .collect();

    for line in input{
        //println!("{}", line);
        let mut highest = 0;
        for i in 0..(line.len()){
            for j in (i+1)..(line.len()){
                let temp = format!("{}{}", line.bytes().nth(i).unwrap() as char, line.bytes().nth(j).unwrap() as char).parse::<u64>().unwrap();
                //print!("{},", temp);
                if temp > highest{
                    highest = temp;
                }
            }
        }
        //println!("Highest: {}", highest);
        ret += highest;
    }

    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    let mut ret: u64 = 0;
    let input: Vec<&str> = input
    .lines()
    .collect();

    for line in input{
        println!("{}", line);
        let mut vel: Vec<char> = Vec::new(); //result

        let line: Vec<char> = line.chars().collect(); //line to loop through
        let mut neru: usize = 0; //left pointer of window
        for i in (line.len()-12)..=(line.len()-1){ //from reserving 11 to reserving 0
            //println!("Window: {}-{}", neru, i);
            //loop through window and get highest char
            let mut highest: (usize, char) = (0, '\0');
            for j in neru..=i{ 
                if line[j] > highest.1{
                    highest = (j, line[j]);
                }
            }
            //update left window pointer and push result
            neru = highest.0+1;
            vel.push(highest.1);
        }
        let vel: u64 = vel.iter().fold(String::new(), |mut acc, new|{
            acc.push(*new);
            acc
        }).parse::<u64>().unwrap();
        println!("result: {}", vel);
        ret += vel;
    }
    return format!("{}", ret);
}