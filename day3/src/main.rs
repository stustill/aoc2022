use std::{collections::HashSet, fs::File, io::Read};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    file: String,
    #[arg(short, long)]
    badges: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file = File::open(&args.file)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let result: u32 = if args.badges {
        let rucksacks: Vec<HashSet<char>> = input
            .lines()
            .map(to_compartments)
            .map(to_contents)
            .collect();
        rucksacks
            .chunks(3)
            .map(|sacks| {
                let sack0 = &sacks[0];
                let sack1 = &sacks[1];
                let sack2 = &sacks[2];
                let partial_intersection: HashSet<char> =
                    sack0.intersection(sack1).copied().collect();
                let full_intersection: HashSet<char> =
                    partial_intersection.intersection(sack2).copied().collect();
                let it = full_intersection.iter().next();
                it.unwrap().clone()
            })
            .map(to_priority)
            .sum()
    } else {
        input
            .lines()
            .map(to_compartments)
            .map(to_duplicate)
            .map(to_priority)
            .sum()
    };
    println!("{}", result);
    Ok(())
}

fn to_compartments(line: &str) -> (&str, &str) {
    let compartments = line.split_at(line.len() / 2);
    (compartments.0, compartments.1)
}

fn to_contents(rucksack: (&str, &str)) -> HashSet<char> {
    rucksack.0.chars().chain(rucksack.1.chars()).collect()
}

fn to_duplicate(rucksack: (&str, &str)) -> char {
    let comp1: HashSet<char> = rucksack.0.chars().collect();
    let comp2: HashSet<char> = rucksack.1.chars().collect();
    let mut dupes = comp1.intersection(&comp2);
    dupes.next().unwrap().to_owned()
}

fn to_priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        u32::from(item) - u32::from('A') + 27
    } else {
        u32::from(item) - u32::from('a') + 1
    }
}
