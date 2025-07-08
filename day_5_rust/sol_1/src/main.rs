use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn check_ordering(ordering: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let ordering_map: HashMap<u32, usize> = ordering
        .iter()
        .enumerate()
        .map(|(index, &element)| (element, index))
        .collect();

    //if ordering[0] == 87 && ordering[1] == 86 && ordering[2] == 37 {
    //    for (key, value) in &ordering_map {
    //        println!("key: {key}, value: {value}")
    //    }
    //}
    // iterate through all the rules
    for (rules_key, rules_vector) in rules {
        if ordering_map.contains_key(rules_key) {
            for element in rules_vector {
                if ordering_map.contains_key(element) {
                    if ordering_map.get(element) < ordering_map.get(rules_key) {
                        return false;
                    }
                } else {
                    continue;
                }
            }
        } else {
            continue;
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);
    let mut sum: u32 = 0;
    let mut in_rules = true;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    // Read line by line
    for (_line_idx, line) in reader.lines().enumerate() {
        let line = line?;

        if in_rules {
            if line.is_empty() {
                in_rules = false;
                continue;
            }
            let (left, right) = line.split_once('|').unwrap();
            let a: u32 = left.parse().unwrap();
            let b: u32 = right.parse().unwrap();

            // Inserts b into a's rules, and creates a new entry for a if no rules for a exist
            // yet
            rules.entry(a).or_insert_with(Vec::new).push(b);
        }
        if !in_rules {
            let ordering: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            let if_correct: bool = check_ordering(&ordering, &rules);

            if if_correct {
                println!("Correct ordering found");
                sum += ordering[ordering.len() / 2];
            } else {
                println!("Incorrect ordering found");
            }
        }
    }
    println!("{sum}");
    Ok(())
}
