use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // 1. Open the file
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);

    // 2. Read line by line…
    for (line_idx, line_res) in reader.lines().enumerate() {
        let mut increasing = true;
        let mut decreasing = true;
        let line = line_res?; // String
        println!("Line {}: {}", line_idx + 1, line);

        // 3. Split on whitespace, parse each token
        for (token_idx, token) in line.split_whitespace().enumerate() {
            // parse::<T>() where T is your number type, e.g. i32 or f64
            let num: i32 = token
                .parse()
                .expect(&format!("invalid integer on line {}", line_idx + 1));
            println!("  Token {} → {}", token_idx + 1, num);
            // …or do whatever you need with `num` here
        }
    }

    Ok(())
}
