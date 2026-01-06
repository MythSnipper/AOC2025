
fn main() {
    let input = include_str!("puzzle_input");
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> String {
    let mut ret: u64 = 0;
    let input: Vec<&str> = input
    .split("\n\n")
    .collect();

    let ranges: Vec<(u64, u64)> = input[0]
    .lines()
    .map(|vel| 
        (
            vel.split('-').nth(0).unwrap(), 
            vel.split('-').nth(1).unwrap()
        )
    )
    .map(|(vel, vell)|
        (
            vel.parse::<u64>().unwrap(),
            vell.parse::<u64>().unwrap()
        )
    )
    .collect();

    let ids: Vec<u64> = input[1]
    .lines()
    .map(|vel| vel.parse::<u64>().unwrap())
    .collect();

    for id in ids{
        let mut spoiled = true;
        for range in &ranges{
            if id >= range.0 && id <= range.1{
                //not spoiled
                spoiled = false;
                break;
            }
        }
        if !spoiled{ //counting fresh ingredients
            ret += 1;
        }
    }

    return format!("{}", ret);
}

fn solve_part2(input: &str) -> String {
    let mut ret: u64 = 0;
    let input: Vec<&str> = input
    .split("\n\n")
    .collect();
    let mut ranges: Vec<(u64, u64)> = input[0]
    .lines()
    .map(|vel| 
        (
            vel.split('-').nth(0).unwrap(), 
            vel.split('-').nth(1).unwrap()
        )
    )
    .map(|(vel, vell)|
        (
            vel.parse::<u64>().unwrap(),
            vell.parse::<u64>().unwrap()
        )
    )
    .collect();

    ranges.sort();
    let mut new_ranges: Vec<(u64, u64)> = Vec::new();

    let mut i = 1; //new range index
    let mut lastrange = ranges[0];
    loop{
        if i == ranges.len(){
            new_ranges.push(lastrange);
            break;
        }
        //get the new range to check
        let currrange = ranges[i];
        //no overlap
        if lastrange.1 < currrange.0{
            //push result
            new_ranges.push(lastrange);
            lastrange = currrange;
            i += 1;
        }
        //fully contained
        else if lastrange.0 <= currrange.0 && lastrange.1 >= currrange.1{
            //GET OUT
            i += 1;
        }
        //overlaps
        else if lastrange.0 <= currrange.0 && currrange.1 >= lastrange.1{
            //merge
            lastrange = (lastrange.0, currrange.1);
            i += 1;
        }
    }
    //println!("size1: {}", ranges.len());
    //println!("size2: {}", new_ranges.len());
    for r in new_ranges{
        ret += r.1 - r.0 + 1;
    }

    return format!("{}", ret);
}