use std::env;
use std::fs;

fn distance(a: u32, b: u32) -> u32 {
    let displace : i64 = b as i64 - a as i64;
    if displace >=0 {
        displace.try_into().unwrap()
    } else {
        (0 - displace).try_into().unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut locations1 = Vec::new();
    let mut locations2 = Vec::new();

    if args.len() != 2 {
        println!("usage: {} <filepath>", args[0]);
        return;
    }
    /*
     * the text file should have the format of two numbers per line
     * with one or more spaces between them. the number on the left
     * represents an id in the left list and the number on the right 
     * represents an id in the right list.
     */ 
    let file_to_read = fs::read_to_string(&args[1])
        .expect("oops we couldn't get that file.");

    let id_line_vec = file_to_read.lines();

    for line in id_line_vec {
        let nums: Vec<u32> = line.split_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>().expect("the file must contain numbers only"))
            .collect();
        if nums.len() != 2 {
            println!("oops, the file was not formatted properly (i.e. 2 columns of numbers separated by spaces)");
            return;
        }
        locations1.push(nums[0]);
        locations2.push(nums[1]);
        
    }

    locations1.sort();
    locations2.sort();
    let zipped_locations: Vec<(&u32, &u32)> = locations1.iter()
        .zip(locations2.iter()).collect();
    
    let mut sum = 0;
    for (l1, l2) in zipped_locations {
        sum += distance(*l1, *l2);
    }
    println!("distance: {sum}");

    let mut similar = 0;

    for l1 in &locations1 {
        let mut hits = 0;
        for l2 in &locations2 {
            if l2 == l1 {
                hits += 1;
            } else {
                if hits > 0 {
                    break;
                }
            }
        }
        similar += l1 * hits;
    }

    println!("similarity: {similar}");
}
