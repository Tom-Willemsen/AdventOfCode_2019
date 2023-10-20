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

fn calculate_p1(nums: &[i64]) -> i64 {
    let mut prog: IntCodeState = nums.into();
    prog.execute_until_halt(|| Some(1));
    prog.out_buffer.pop_back().unwrap()
}

fn calculate_p2(nums: &[i64]) -> i64 {
    let mut prog: IntCodeState = nums.into();
    prog.execute_until_halt(|| Some(5));
    prog.out_buffer.pop_back().unwrap()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums: Vec<i64> = parse(&inp);
    let p1 = calculate_p1(&nums);
    let p2 = calculate_p2(&nums);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2019_05");
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_05");

    #[test]
    fn test_large_example() {
        for (val, expected) in [(7, 999), (8,1000), (9, 1001)] {
            let mut prog: IntCodeState = parse(&EXAMPLE_DATA).into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(expected));
        }
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA)), 999);
    }
    
    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 13294380);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 11460760);
    }

}
