use advent_of_code_2019::intcode::IntCodeState;
use advent_of_code_2019::{Cli, Parser};
use std::collections::VecDeque;
use std::fs;

fn run_with_logic(software: &str, logic: &str) -> i64 {
    let mut prog: IntCodeState<4096> = software.into();

    let mut inputs = logic.bytes().map(|x| x as i64).collect::<VecDeque<_>>();

    prog.execute_until_halt(|_| inputs.pop_front());

    prog.out_buffer.pop_back().expect("no output")
}

fn calculate_p1(software: &str) -> i64 {
    const LOGIC: &str = "OR A T
AND B T
AND C T
NOT T J
AND D J
WALK
";

    run_with_logic(software, LOGIC)
}

fn calculate_p2(software: &str) -> i64 {
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

    let p1 = calculate_p1(&inp);
    let p2 = calculate_p2(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_21");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&REAL_DATA), 19353619);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&REAL_DATA), 1142785329);
    }
}
