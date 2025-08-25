use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
 
    let file_to_read = fs::read_to_string(&args[1])
        .expect("oops we couldn't get that file.");
}
