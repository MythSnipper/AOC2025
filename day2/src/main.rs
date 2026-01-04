
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    //println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let ranges: Vec<_> = input
    .split(',')
    .map(|vel| vel.split('-'))
    .map(|mut vel| (vel.next().unwrap(), vel.next().unwrap()))
    .map(|vel| (vel.0.parse::<u64>().unwrap(), vel.1.parse::<u64>().unwrap()))
    .collect();

    let mut ret: u64 = 0;
    for (vel, vell) in ranges{
        //println!("Range: {}-{}", vel, vell);
        for neru in vel..=vell{
            let teto: String = format!("{}", neru);
            let miku: (&str, &str) = (&teto[0..=teto.len()/2], &teto[teto.len()/2..teto.len()]);
            if miku.0.len() != miku.1.len(){
                break;
            }
            if miku.0 == miku.1{
                ret += neru;
            }
        }
    }


    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    let movements: Vec<i32> = input
    .lines()
    .map(|x| (x.chars().nth(0).unwrap(), &x[1..x.len()]))
    .map(|x| (x.0, x.1.parse::<i32>().unwrap()))
    .map(|x| x.1 * (if x.0 == 'L'{-1} else{1}))
    .collect();
    
    let mut ret = 0;
    let mut position = 50;
    for mov in movements{
        //!("{} {}", position, mov);
        for _ in 0..(if mov<0{-mov}else{mov}){
            position += if mov<0{-1}else{1};
            if position == -1{position = 99;}
            if position == 100{position = 0;}
            if position == 0{ret+=1;}
        }
        //print!(" ={} {}\n", position, if position == 0{"True"}else{""});
    }
    return format!("{}", ret);
}