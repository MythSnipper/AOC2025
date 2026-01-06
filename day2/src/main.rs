
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
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
            let miku: (&str, &str) = (&teto[0..teto.len()/2], &teto[teto.len()/2..teto.len()]);

            if miku.0 == miku.1{
                //println!("{} comp {}", miku.0, miku.1);
                ret += neru;
                //println!("Detected: {}", neru);
            }
        }
    }

    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    let ranges: Vec<_> = input
    .split(',')
    .map(|vel| vel.split('-'))
    .map(|mut vel| (vel.next().unwrap(), vel.next().unwrap()))
    .map(|vel| (vel.0.parse::<u64>().unwrap(), vel.1.parse::<u64>().unwrap()))
    .collect();

    let mut ret: u64 = 0;
    for (vel, vell) in ranges{ //loop through ranges
        //println!("Range: {}-{}", vel, vell);
        for neru in vel..=vell{ //loop through every number of the range
            //println!("-Number: {}", neru);
            let teto: String = format!("{}", neru); //current string to check
            for niko in 1..=teto.len()/2{ //check which lengths the string can be split into evenly
                if teto.len()%niko==0{ //if can evenly divide
                    //println!("--Valid divide length: {}", niko);
                    //split string into (teto.len()/niko) slices of niko length
                    let mut eep: Vec<String> = Vec::new();
                    for meow in 0..teto.len()/niko{
                        eep.push((teto[meow*niko..meow*niko+niko]).to_string());
                    }
                    //check if all splits are the same
                    let nya = eep[0].clone();
                    let mut love: bool = false;
                    for nap in eep{
                        if nap != nya{
                            love = true;
                            break;
                        }
                    }
                    if !love{
                        ret += neru;
                        break;
                    }
                    /*
                    println!("---Splits: ");
                    for nap in eep{
                        println!("----{}", nap);
                    }
                    */
                }
            }
        }
    }


    return format!("{}", ret);
}