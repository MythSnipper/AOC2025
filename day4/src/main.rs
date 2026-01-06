
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;
    let input: Vec<&str> = input.lines().collect();

    for i in 0..input.len(){
        for j in 0..input[0].len(){
            //check if it is a roll
            if input[i].as_bytes()[j] as char == '@'{
                //check number of rolls around it
                let mut miku = 0;
                for vel in (if i==0{i}else{i-1})..=(if i==input.len()-1{i}else{i+1}){
                    for vell in (if j==0{j}else{j-1})..=(if j==input[0].len()-1{j}else{j+1}){
                        if input[vel].as_bytes()[vell] as char == '@'{
                            miku += 1;
                        }
                    }
                }
                //if fewer than 5 rolls(4+1 including the roll itself)
                if miku < 5{
                    ret += 1;
                }
            }
        }
    }
    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    let mut ret: u64 = 0;
    let mut input: Vec<String> = input.lines().map(|vel| vel.to_string()).collect();

    let mut teto: bool = true; //check by default

    while teto{
        teto = false; //assume no need to check
        for i in 0..input.len(){
            for j in 0..input[0].len(){
                //check if it is a roll
                if input[i].as_bytes()[j] as char == '@'{
                    //check number of rolls around it
                    let mut miku = 0;
                    for vel in (if i==0{i}else{i-1})..=(if i==input.len()-1{i}else{i+1}){
                        for vell in (if j==0{j}else{j-1})..=(if j==input[0].len()-1{j}else{j+1}){
                            let currchar = input[vel].as_bytes()[vell] as char;
                            if currchar == '@' || currchar == 'x'{
                                miku += 1;
                            }
                        }
                    }
                    //if fewer than 5 rolls(4+1 including the roll itself)
                    if miku < 5{
                        ret += 1;
                        teto = true; //mark for another check
                        //remove it
                        let bytes = unsafe{input[i].as_bytes_mut()};
                        bytes[j] = b'x';
                    }
                }
            }
        }
        //clear all the x's in the grid
        for i in 0..input.len(){
            for j in 0..input[0].len(){
                //print!("{}", input[i].as_bytes()[j] as char);
                //check if it is a marked for remove roll
                if input[i].as_bytes()[j] as char == 'x'{
                    //remove it
                    let bytes = unsafe{input[i].as_bytes_mut()};
                    bytes[j] = b'.';
                }
            }
            //println!();
        }
        //print!("\n\n\n");
    }
    return format!("{}", ret);
}