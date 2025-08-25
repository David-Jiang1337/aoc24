use std::env;
use std::fs;
use regex::Regex;

static ref MUL_PATTERN: Regex = Regex::new(r"mul\(\d+,\d+\)");
static ref NUM_PATTERN: Regex = Regex::new(r"\d+");

/*
 * if the string is in the format "mul(<int>,<int>)", return the product
 * of the two numbers. otherwise, panic
 */
fn calc_mul(string: &str) -> i32 {
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
 
    let file_to_read = fs::read_to_string(&args[1])
        .expect("oops we couldn't get that file.");
    
    

}
