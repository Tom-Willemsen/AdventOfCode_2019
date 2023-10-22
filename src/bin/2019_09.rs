use advent_of_code_2019::intcode::IntCodeState;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<i64> {
    raw_inp
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calculate<const MODE: i64>(software: &[i64]) -> i64 {
    let mut prog: IntCodeState = software.into();
    prog.execute_until_halt(|_| Some(MODE));
    prog.out_buffer.pop_back().unwrap()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums: Vec<i64> = parse(&inp);
    let p1 = calculate::<1>(&nums);
    let p2 = calculate::<2>(&nums);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_09");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&parse(&REAL_DATA)), 3345854957);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&parse(&REAL_DATA)), 68938);
    }
}
