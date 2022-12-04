use std::{
    fs::File,
    io::{Read, Result},
    ops::RangeInclusive,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    file: String,
    #[arg(short, long)]
    part2: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = read_input(&args.file)?;
    let predicate = if args.part2 {
        is_overlapped
    } else {
        is_contained
    };
    let result = input.lines().map(to_ranges).filter(predicate).count();
    println!("{}", result);
    Ok(())
}

fn read_input(file: &str) -> Result<String> {
    let mut file = File::open(file)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn to_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let (first, second) = line.split_once(',').unwrap();
    (to_range(first), to_range(second))
}

fn to_range(range: &str) -> RangeInclusive<u32> {
    let (start_idx, end_idx) = range.split_once('-').unwrap();
    let start_idx: u32 = start_idx.parse().unwrap();
    let end_idx: u32 = end_idx.parse().unwrap();
    start_idx..=end_idx
}

fn is_overlapped(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    ranges.0.contains(ranges.1.start())
        || ranges.0.contains(ranges.1.end())
        || ranges.1.contains(ranges.0.start())
        || ranges.1.contains(ranges.0.end())
}

fn fully_contains(range0: &RangeInclusive<u32>, range1: &RangeInclusive<u32>) -> bool {
    (range0.start() <= range1.start()) && (range0.end() >= range1.end())
}

fn is_contained(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    fully_contains(&ranges.0, &ranges.1) || fully_contains(&ranges.1, &ranges.0)
}
