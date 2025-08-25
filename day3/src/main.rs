use std::env;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
lazy_static! {
    static ref MUL_PATTERN: Regex = Regex::new(r"mul\(\d+,\d+\)").expect("whoops the regex pattern isn't valid");
    static ref NUM_PATTERN: Regex = Regex::new(r"\d+").expect("whoops the regex pattern isn't valid");
}

/*
 * if the string is in the format "mul(<int>,<int>)", return the product
 * of the two numbers
 */
fn calc_mul(string: &str) -> i32 {
    let match_nums: Vec<i32> = NUM_PATTERN.find_iter(string)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut out: i32 = match_nums[0];
    for m in match_nums.iter().skip(1) {
        out *= m;
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
 
    let file_to_read = fs::read_to_string(&args[1])
        .expect("oops we couldn't get that file.");
    
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        assert_eq!(calc_mul("mul(1,0)"), 0);
        assert_eq!(calc_mul("mul(2,3)"), 6);
        assert_eq!(calc_mul("mul(12,12)"), 144);
        assert_eq!(calc_mul("mul(50,22)"), 1100);
    }
}
