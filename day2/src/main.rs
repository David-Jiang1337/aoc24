use std::env;
use std::fs;

fn int_vec_from_str(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("reports must only contain integers"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
 
    let file_to_read = fs::read_to_string(&args[1])
        .expect("oops we couldn't get that file.");

    let mut reports: Vec<&str> = file_to_read.lines().collect();

}
