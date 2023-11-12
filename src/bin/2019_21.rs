use advent_of_code_2019::intcode::IntCodeState;
use clap::Parser;
use std::collections::VecDeque;
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

fn run_with_logic(software: &[i64], logic: &str) -> i64 {
    let mut prog: IntCodeState = software.into();

    let mut inputs = logic.bytes().map(|x| x as i64).collect::<VecDeque<_>>();

    prog.execute_until_halt(|_| inputs.pop_front());

    prog.out_buffer.pop_back().expect("no output")
}

fn calculate_p1(software: &[i64]) -> i64 {
    const LOGIC: &str = "OR A T
AND B T
AND C T
NOT T J
AND D J
WALK
";

    run_with_logic(software, LOGIC)
}

fn calculate_p2(software: &[i64]) -> i64 {
    const LOGIC: &str = "OR A T
AND B T
AND C T
NOT T J
AND D J
AND H J
NOT A T
OR T J
RUN
";

    run_with_logic(software, LOGIC)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_21");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 19353619);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 1142785329);
    }
}
