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
 * returns true if every element in the vector fulfills the comparison condition
 * with the next element in the vector, or if the list is empty or has only one
 * element. otherwise, returns false.
 */
fn is_ordered(ints: &Vec<i32>, tolerance: i32, compare: fn(i32, i32) -> bool) -> bool {
    for (i, val) in ints.iter().skip(1).enumerate() {
        if !compare(ints[i], *val){
            return false;
        }
    }
    true
}

/*
 * returns true if every element is greater than the last, or if the vector has
 * only one element or is empty. otherwise, return false.
 */
fn is_increasing(ints: &Vec<i32>) -> bool {
    is_ordered(ints, |a: i32, b: i32| a < b)
}

/*
 * returns true if every element is smaller than the last, or if the vector has
 * only one element or is empty. otherwise, return false.
 */
fn is_decreasing(ints: &Vec<i32>) -> bool {
    is_ordered(ints, |a: i32, b: i32| a > b)
}

fn largest_abs_change(ints: &Vec<i32>) -> i32 {
    let mut largest: i32 = 0;
    for (i, val) in ints.iter().skip(1).enumerate() {
        let val = *val;
        let prev_val = ints[i];
        let chg = if prev_val > val {
            prev_val - val
        } else {
            val - prev_val
        };
        if chg > largest {
            largest = chg;
        }
    }
    largest
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
    
    let mut safe_count: u32 = 0;

    for line in reports {
        let levels: Vec<i32> = int_vec_from_str(line);
        let chg = largest_abs_change(&levels);
        if (is_increasing(&levels) || is_decreasing(&levels))
            && (chg >= 1 && chg <= 3) {
                safe_count += 1;
        }
    }

    println!("safe reports: {safe_count}");
}
