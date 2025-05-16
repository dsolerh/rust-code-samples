use std::env;

use anyhow::{anyhow, Context};
use aoc::SolutionsMap;
use clap::{Parser, ValueEnum};
use dotenvy::dotenv;

#[derive(Clone, Debug)]
enum SolutionPart {
    Part1,
    Part2,
}

impl ValueEnum for SolutionPart {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Part1, Self::Part2]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            SolutionPart::Part1 => Some(clap::builder::PossibleValue::new("part1")),
            SolutionPart::Part2 => Some(clap::builder::PossibleValue::new("part2")),
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    year: usize,
    #[arg(short, long)]
    day: usize,
    #[arg(short, long)]
    part: SolutionPart,
}

fn main() -> anyhow::Result<()> {
    dotenv()?;
    let aoc_session = env::var("AOC_SESSION").expect("AOC_SESSION must be set");
    let args = Args::parse();
    let solutions = SolutionsMap::init();

    let key = format!("{}_{}", args.year, args.day);
    let solver = solutions
        .get(&key)
        .ok_or(anyhow!("the solution is not present"))?;

    let data =
        aoc::get_data(&aoc_session, args.year, args.day).context("failed to fetch the data")?;

    let ans = match args.part {
        SolutionPart::Part1 => solver.solve_part1(data),
        SolutionPart::Part2 => solver.solve_part2(data),
    };

    println!("answer: {ans}");
    Ok(())
}
