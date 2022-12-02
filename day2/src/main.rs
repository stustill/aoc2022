use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    file: String,
    #[arg(short, long)]
    is_outcome: bool,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}
impl TryFrom<&str> for Outcome {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Action {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Action {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper) => Outcome::Lose,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Lose,
            (Self::Scissors, Self::Rock) => Outcome::Lose,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (_, _) => Outcome::Draw,
        }
    }

    fn action_for(self, outcome: Outcome) -> Action {
        match (self, outcome) {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Lose) => Self::Scissors,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Lose) => Self::Rock,
            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Lose) => Self::Paper,
            (_, _) => self,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.file)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|o| {
        let line = o.unwrap();
        line.split_once(' ')
            .map(|p| (p.0.to_string(), p.1.to_string()))
            .unwrap()
    });

    let result = if args.is_outcome {
        let strategy = input.map(|(their_action, outcome)| {
            (
                Action::try_from(their_action.as_ref()).unwrap(),
                Outcome::try_from(outcome.as_ref()).unwrap(),
            )
        });
        calculate_by_outcome(strategy)
    } else {
        let strategy = input.map(|(their_action, my_action)| {
            (
                Action::try_from(their_action.as_ref()).unwrap(),
                Action::try_from(my_action.as_ref()).unwrap(),
            )
        });
        calculate_by_action(strategy)
    };
    println!("{}", result);
    Ok(())
}

fn calculate_by_action(strategy: impl Iterator<Item = (Action, Action)>) -> u32 {
    strategy
        .map(|(their_action, my_action)| my_action.value() + my_action.play(&their_action).value())
        .sum()
}

fn calculate_by_outcome(strategy: impl Iterator<Item = (Action, Outcome)>) -> u32 {
    strategy
        .map(|(their_action, outcome)| outcome.value() + their_action.action_for(outcome).value())
        .sum()
}
