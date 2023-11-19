use advent_of_code_2019::intcode::{parse_intcode_to_vec, IntCodeState};
use advent_of_code_2019::{Cli, Parser};
use std::fs;

fn calculate(nums: &[i64], noun: i64, verb: i64) -> i64 {
    let mut prog: IntCodeState = nums.into();

    prog.set_mem(1, noun);
    prog.set_mem(2, verb);

    prog.execute_until_halt_no_input();

    prog.get_mem(0)
}

fn calculate_p1(nums: &[i64]) -> i64 {
    calculate(nums, 12, 2)
}

fn calculate_p2(nums: &[i64]) -> i64 {
    for verb in 0..100 {
        for noun in 0..100 {
            if calculate(nums, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("p2: no solution found");
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");
    let inp_vec = parse_intcode_to_vec(&inp);
    let p1 = calculate_p1(&inp_vec);
    let p2 = calculate_p2(&inp_vec);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_02");

    fn test_in_out(inp: &[i64], out: &[i64]) {
        let mut prog: IntCodeState = inp.into();
        prog.execute_until_halt_no_input();
        for (idx, mem) in out.iter().enumerate() {
            assert_eq!(prog.get_mem(idx as i64), *mem);
        }
    }

    #[test]
    fn test_example_1() {
        test_in_out(
            &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn test_example_2() {
        test_in_out(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_example_3() {
        test_in_out(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_example_4() {
        test_in_out(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_example_5() {
        test_in_out(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse_intcode_to_vec(&REAL_DATA)), 9581917);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse_intcode_to_vec(&REAL_DATA)), 2505);
    }
}
