use std::env;
use std::fs;

fn int_vec_from_str(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("reports must only contain integers"))
        .collect()
}

fn is_ordered(ints: &Vec<i32>, compare: fn(i32, i32) -> bool) -> bool {
    for (i, val) in ints.iter().skip(1).enumerate() {
        if !compare(ints[i - 1], *val){
            return false;
        }
    }
    true
}

fn is_increasing(ints: &Vec<i32>) -> bool {
    is_ordered(ints, |a: i32, b: i32| a < b)
}

fn is_decreasing(ints: &Vec<i32>) -> bool {
    is_ordered(ints, |a: i32, b: i32| a > b)
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

    let mut reports: Vec<&str> = file_to_read.lines().collect();

}
