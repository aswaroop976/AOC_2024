use std::fs::File;
use std::io::{self, BufRead};

fn check_line(nums: &[u32]) -> bool {
    let mut prev: u32 = 0;
    let mut increasing = false;
    let mut decreasing = false;
    for (idx, num) in nums.iter().enumerate() {
        if idx == 0 {
            prev = *num;
            continue;
        }
        let diff = num.abs_diff(prev);
        if (diff < 1 || diff > 3) {
            return false;
        }
        if (increasing && *num < prev) {
            increasing = false;
            return false;
        }
        if (decreasing && *num > prev) {
            decreasing = false;
            return false;
        }
        if (*num > prev) {
            increasing = true;
        } else if (*num < prev) {
            decreasing = true;
        }
        prev = *num;
    }
    return true;
}

fn main() -> io::Result<()> {
    // 1. Open the file
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);
    let mut num_safe: u32 = 0;
    // 2. Read line by line…
    for (_line_idx, line_res) in reader.lines().enumerate() {
        num_safe += 1;
        let line = line_res?; // String
        //println!("Line {}: {}", line_idx + 1, line);
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if !(check_line(&nums)) {
            let mut dampener = false;
            for idx in 0..nums.len() {
                let mut new_nums = nums.clone();
                new_nums.remove(idx);
                if check_line(&new_nums) {
                    dampener = true;
                    break;
                }
            }
            if (!dampener) {
                num_safe -= 1;
            }
        }
    }
    println!("number of safe levels: {num_safe}");

    Ok(())
}
