use std::env;
use std::fs;

/*
 * returns the string as integers within a vector separated by whitespaces.
 * if there are substrings that cannot be parsed as integers, panic.
 */
fn int_vec_from_str(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("reports must only contain integers"))
        .collect()
}
/*
 * if each consecutive pair of elements in ints satisifies the compare function,
 * returns -1, otherwise, returns the index of the first element in the element pair
 * that violates the condition.
 */
fn is_ordered(ints: &Vec<i32>, tolerance: i32, compare: fn(i32, i32) -> bool) -> isize {
    let mut fails = 0;
    for (i, val) in ints.iter().skip(1).enumerate() {
        if !compare(ints[i], *val){
            fails += 1;
            if fails > tolerance {
                return i.try_into().unwrap();
            }
        }
    }
    return -1;
}

/*
 * returns true if and only if b is greater than a by no less than 1
 * and no more than 3.
 */
fn greater_by_a_little(a: i32, b: i32) -> bool {
    let diff = b - a;
    diff >= 1 && diff <= 3
}

/*
 * returns true if and only if b is smaller than a by no less than 1
 * and no more than 3.
 */
fn smaller_by_a_little(a: i32, b: i32) -> bool {
    let diff = a - b;
    diff >= 1 && diff <= 3
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // ensures correct number of args
    if args.len() != 2 {
        println!("usage: {} <filepath>", args[0]);
        return;
    }

    let file_to_read = fs::read_to_string(&args[1])
        .expect("oops we couldn't get that file.");

    let reports: Vec<&str> = file_to_read.lines().collect();
    
    let mut safe_count_no_dampener: u32 = 0;
    let mut safe_count_w_dampener: u32 = 0;

    for line in reports.iter() {
        let levels: Vec<i32> = int_vec_from_str(line);
        let increase_ordered = is_ordered(&levels, 0, greater_by_a_little);
        let decrease_ordered = is_ordered(&levels, 0, smaller_by_a_little);

        if increase_ordered == -1 || decrease_ordered == -1 {
            safe_count_no_dampener += 1;
            safe_count_w_dampener += 1;
        } else {
            let mut increasing_damp_first = levels.clone();
            let mut increasing_damp_second = levels.clone();
            let mut decreasing_damp_first = levels.clone();
            let mut decreasing_damp_second = levels.clone();
            increasing_damp_first.remove(increase_ordered as usize);
            increasing_damp_second.remove(increase_ordered as usize + 1);
            decreasing_damp_first.remove(decrease_ordered as usize);
            decreasing_damp_second.remove(decrease_ordered as usize + 1);

            if is_ordered(&increasing_damp_first, 0, greater_by_a_little) == -1
                || is_ordered(&increasing_damp_second, 0, greater_by_a_little) == -1
                || is_ordered(&decreasing_damp_first, 0, smaller_by_a_little) == -1
                || is_ordered(&decreasing_damp_second, 0, smaller_by_a_little) == -1 {
                safe_count_w_dampener += 1;
            }
        }
    }

    println!("safe reports w/o problem dampener: {safe_count_no_dampener}");
    println!("safe reports w/ problem dampener: {safe_count_w_dampener}");
}
