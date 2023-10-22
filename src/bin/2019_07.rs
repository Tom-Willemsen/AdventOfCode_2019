use advent_of_code_2019::intcode::IntCodeState;
use clap::Parser;
use itertools::Itertools;
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

fn run_amplifier_p1(software: &[i64], input: i64, phase: i64) -> i64 {
    let mut prog: IntCodeState = software.into();
    let mut inputs = vec![input, phase];
    prog.execute_until_halt(|_| inputs.pop());
    prog.out_buffer.pop_back().unwrap()
}

fn run_amplifier_sequence_p1(software: &[i64], phases: &[i64]) -> i64 {
    let mut inp = 0;
    for p in phases.iter() {
        inp = run_amplifier_p1(software, inp, *p);
    }
    inp
}

fn calculate_p1(software: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|perm| run_amplifier_sequence_p1(software, &perm))
        .max()
        .expect("at least one item should exist")
}

fn run_amplifiers_p2(software: &[i64], phases: &[i64]) -> i64 {
    let mut amp1: IntCodeState = software.into();
    let mut amp2: IntCodeState = software.into();
    let mut amp3: IntCodeState = software.into();
    let mut amp4: IntCodeState = software.into();
    let mut amp5: IntCodeState = software.into();

    let mut a1_inputs = vec![0, phases[0]];
    let mut a2_inputs = vec![phases[1]];
    let mut a3_inputs = vec![phases[2]];
    let mut a4_inputs = vec![phases[3]];
    let mut a5_inputs = vec![phases[4]];

    loop {
        amp1.execute_single_step(|_| a1_inputs.pop().or(amp5.out_buffer.pop_front()));
        amp2.execute_single_step(|_| a2_inputs.pop().or(amp1.out_buffer.pop_front()));
        amp3.execute_single_step(|_| a3_inputs.pop().or(amp2.out_buffer.pop_front()));
        amp4.execute_single_step(|_| a4_inputs.pop().or(amp3.out_buffer.pop_front()));
        let halt = amp5.execute_single_step(|_| a5_inputs.pop().or(amp4.out_buffer.pop_front()));

        if halt {
            break;
        }
    }

    amp5.out_buffer.pop_front().expect("p2: no solution")
}

fn calculate_p2(software: &[i64]) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|perm| run_amplifiers_p2(software, &perm))
        .max()
        .expect("at least one item should exist")
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

    const EXAMPLE_DATA_P1_1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    const EXAMPLE_DATA_P1_2: &str =
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    const EXAMPLE_DATA_P1_3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

    const EXAMPLE_DATA_P2_1: &str =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    const EXAMPLE_DATA_P2_2: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_07");

    #[test]
    fn test_p1_example_1() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_P1_1)), 43210);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_P1_2)), 54321);
    }

    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_P1_3)), 65210);
    }

    #[test]
    fn test_p2_example_1() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_P2_1)), 139629729);
    }

    #[test]
    fn test_p2_example_2() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_P2_2)), 18216);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 11828);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 1714298);
    }
}
