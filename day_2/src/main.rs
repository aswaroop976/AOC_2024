use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // 1. Open the file
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);
    let mut num_safe: u32 = 0;

    // 2. Read line by line…
    for (line_idx, line_res) in reader.lines().enumerate() {
        num_safe += 1;
        let mut increasing = false;
        let mut decreasing = false;
        let line = line_res?; // String
        //println!("Line {}: {}", line_idx + 1, line);
        let mut prev: u32 = 0;

        // 3. Split on whitespace, parse each token
        for (token_idx, token) in line.split_whitespace().enumerate() {
            // parse::<T>() where T is your number type, e.g. i32 or f64
            let num: u32 = token
                .parse()
                .expect(&format!("invalid integer on line {}", line_idx + 1));
            if (token_idx == 0) {
                prev = num;
                continue;
            }
            let diff = num.abs_diff(prev);
            if (diff < 1 || diff > 3) {
                num_safe -= 1;
                break;
            }
            if (increasing && num < prev) {
                increasing = false;
                num_safe -= 1;
                break;
            }
            if (decreasing && num > prev) {
                decreasing = false;
                num_safe -= 1;
                break;
            }
            if (num > prev) {
                increasing = true;
            } else if (num < prev) {
                decreasing = true;
            }
            prev = num;
        }
    }
    println!("number of safe levels: {num_safe}");

    Ok(())
}
