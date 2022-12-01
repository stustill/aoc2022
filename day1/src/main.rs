use std::io::{prelude::*, BufReader};

fn main() {
    let number_of_elves = 3;

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elves: Vec<u32> = vec![0];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(0);
        } else {
            let calories: u32 = line.parse().unwrap();
            if let Some(count) = elves.last_mut() {
                *count += calories
            }
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    let calories: u32 = elves.iter().take(number_of_elves).sum();
    println!("{}", calories);
}
