use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // open file
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);
    let mut num_x: u32 = 0;
    let mut num_m: u32 = 0;
    let mut num_a: u32 = 0;
    let mut num_s: u32 = 0;
    let x: char = 'X';
    let m: char = 'M';
    let a: char = 'A';
    let s: char = 'S';
    for (_line_idx, line_res) in reader.lines().enumerate() {
        let line = line_res?;
        for (_char_idx, char) in line.chars().enumerate() {
            if char == x {
                num_x += 1;
            } else if char == m {
                num_m += 1;
            } else if char == a {
                num_a += 1;
            } else if char == s {
                num_s += 1;
            }
        }
    }
    if num_x < num_m && num_x < num_a && num_x < num_s {
        println!("# of times XMAS appears:{num_x}");
    }

    if num_m < num_x && num_m < num_a && num_m < num_s {
        println!("# of times XMAS appears:{num_m}");
    }

    if num_a < num_x && num_a < num_m && num_a < num_s {
        println!("# of times XMAS appears:{num_a}");
    }

    if num_s < num_x && num_s < num_m && num_s < num_a {
        println!("# of times XMAS appears:{num_s}");
    }
    Ok(())
}
