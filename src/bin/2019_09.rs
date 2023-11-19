use advent_of_code_2019::intcode::IntCodeState;
use advent_of_code_2019::{Cli, Parser};
use std::fs;

fn calculate<const MODE: i64>(software: &str) -> i64 {
    let mut prog: IntCodeState<2048> = software.into();
    prog.execute_until_halt(|_| Some(MODE));
    prog.out_buffer.pop_back().unwrap()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let p1 = calculate::<1>(&inp);
    let p2 = calculate::<2>(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_09");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&REAL_DATA), 3345854957);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&REAL_DATA), 68938);
    }
}
