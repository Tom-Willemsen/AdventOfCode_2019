use advent_of_code_2019::intcode::IntCodeState;
use advent_of_code_2019::{Cli, Parser};
use std::fs;

fn calculate_p1(software: &str) -> i64 {
    let mut prog: IntCodeState = software.into();
    prog.execute_until_halt(|_| Some(1));
    prog.out_buffer.pop_back().unwrap()
}

fn calculate_p2(software: &str) -> i64 {
    let mut prog: IntCodeState = software.into();
    prog.execute_until_halt(|_| Some(5));
    prog.out_buffer.pop_back().unwrap()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let p1 = calculate_p1(&inp);
    let p2 = calculate_p2(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2019_05");
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_05");

    #[test]
    fn test_large_example() {
        for (val, expected) in [(7, 999), (8, 1000), (9, 1001)] {
            let mut prog: IntCodeState = EXAMPLE_DATA.into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(expected));
        }
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&EXAMPLE_DATA), 999);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&REAL_DATA), 13294380);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&REAL_DATA), 11460760);
    }
}
