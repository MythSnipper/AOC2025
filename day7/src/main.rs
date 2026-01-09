
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;
    let mut input: Vec<Vec<char>> = input
    .lines()
    .map(|vel| vel.chars().collect())
    .collect();
    
    //find the S
    let mut root: (usize, usize) = (0, 0);
    for i in 0..input[0].len(){
        if input[0][i] == 'S'{
            root = (1, i);
            break;
        }
    }
    
    //call the recursive helper to do thing
    solve_part1_helper1(&mut input, root);

    for vel in 0..input.len(){
        for vell in 0..input[0].len(){
            print!("{}", input[vel][vell]);

            if input[vel][vell] == '^' && input[vel-1][vell] == '|'{
                ret+=1;
            }
        }
        println!();
    }

    return format!("{}", ret);
}

//simulate beam travelling down, recursively calls itself when split
fn solve_part1_helper1(vel: &mut Vec<Vec<char>>, mut pos: (usize, usize)) {
    while solve_part1_helper2(vel, pos){
        if vel[pos.0][pos.1] == '^'{
            /*
            println!("Split at ({}, {})", pos.0, pos.1);

            
            for a in 0..vel.len(){
                for b in 0..vel[0].len(){
                    print!("{}", vel[a][b]);
                }
                println!();
            }
            println!();
            */

            solve_part1_helper1(vel, (pos.0, pos.1-1));
            solve_part1_helper1(vel, (pos.0, pos.1+1));
            return;
        }
        else if vel[pos.0][pos.1] == '.'{
            vel[pos.0][pos.1] = '|';
            pos.0 += 1;
        }
        else{
            return;
        }
    }
}
//checks if pos is valid
fn solve_part1_helper2(vel: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if pos.0 >= vel.len() || pos.1 >= vel[0].len(){
        false
    }
    else{
        true
    }
}







fn solve_part2(input: &str) -> String {
    let mut input: Vec<Vec<(char, Option<u64>)>> = input
    .lines()
    .map(|vel| vel
        .chars()
        .map(|uwu| (uwu, None))
        .collect())
    .collect();
    
    //find the S
    let mut root: (usize, usize) = (0, 0);
    for i in 0..input[0].len(){
        if input[0][i].0 == 'S'{
            root = (1, i);
            break;
        }
    }
    
    //call the recursive helper to do thing
    let ret = solve_part2_helper1(&mut input, root);

    return format!("{}", ret);
}

//simulate beam travelling down, recursively calls itself when split
fn solve_part2_helper1(vel: &mut Vec<Vec<(char, Option<u64>)>>, mut pos: (usize, usize)) -> u64 {
    while solve_part2_helper2(vel, pos){
        if vel[pos.0][pos.1].0 == '^'{
            /*
            println!("Split at ({}, {})", pos.0, pos.1);

            
            for a in 0..vel.len(){
                for b in 0..vel[0].len(){
                    print!("{}", vel[a][b]);
                }
                println!();
            }
            println!();
            */
            if let Some(vell) = vel[pos.0][pos.1].1{
                return vell;
            }
            else{
                let mut acc: u64 = 0;
                acc += solve_part2_helper1(vel, (pos.0, pos.1-1));
                acc += solve_part2_helper1(vel, (pos.0, pos.1+1));
                vel[pos.0][pos.1].1 = Some(acc);
                return acc;
            }
        }
        vel[pos.0][pos.1].0 = '|';
        pos.0 += 1;
        
    }
    return 1;
}
//checks if pos is valid
fn solve_part2_helper2(vel: &Vec<Vec<(char, Option<u64>)>>, pos: (usize, usize)) -> bool {
    !(pos.0 >= vel.len() || pos.1 >= vel[0].len())
}


