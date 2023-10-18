use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct IntCodeState {
    state: Vec<i64>,
    instruction_ptr: usize,
}

impl From<&[i64]> for IntCodeState {
    fn from(item: &[i64]) -> Self {
        IntCodeState {
            state: item.to_vec(),
            instruction_ptr: 0,
        }
    }
}

impl From<Vec<i64>> for IntCodeState {
    fn from(item: Vec<i64>) -> Self {
        IntCodeState {
            state: item,
            instruction_ptr: 0,
        }
    }
}

fn parse(raw_inp: &str) -> Vec<i64> {
    raw_inp
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn offset_to_ref(prog: &IntCodeState, offset: usize) -> usize {
    let pos = *prog
        .state
        .get(prog.instruction_ptr + offset)
        .expect("invalid offset");
    usize::try_from(pos).expect("can't convert to usize")
}

fn deref(prog: &IntCodeState, offset: usize) -> &i64 {
    let pos = offset_to_ref(prog, offset);
    prog.state.get(pos).expect("invalid reference")
}

fn deref_mut(prog: &mut IntCodeState, offset: usize) -> &mut i64 {
    let pos = offset_to_ref(prog, offset);
    prog.state.get_mut(pos).expect("invalid reference")
}

fn handle_add(prog: &mut IntCodeState) {
    let src1 = *deref(prog, 1);
    let src2 = *deref(prog, 2);

    let dest = deref_mut(prog, 3);
    *dest = src1 + src2;

    prog.instruction_ptr += 4;
}

fn handle_mul(prog: &mut IntCodeState) {
    let src1 = *deref(prog, 1);
    let src2 = *deref(prog, 2);

    let dest = deref_mut(prog, 3);
    *dest = src1 * src2;

    prog.instruction_ptr += 4;
}

fn execute_single_step(prog: &mut IntCodeState) -> bool {
    let ins_ptr: usize = prog.instruction_ptr;
    let instruction: i64 = *prog
        .state
        .get(ins_ptr)
        .expect("invalid instruction pointer");

    if instruction == 99 {
        return true;
    } else if instruction == 1 {
        handle_add(prog);
    } else if instruction == 2 {
        handle_mul(prog);
    }

    debug_assert!(prog.instruction_ptr != ins_ptr);
    false
}

fn execute_until_halt(prog: &mut IntCodeState) {
    loop {
        let halt = execute_single_step(prog);
        if halt {
            break;
        }
    }
}

fn calculate(nums: &[i64], noun: i64, verb: i64) -> i64 {
    let mut prog: IntCodeState = nums.into();

    prog.state[1] = noun;
    prog.state[2] = verb;

    execute_until_halt(&mut prog);

    prog.state[0]
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

    let nums: Vec<i64> = parse(&inp);
    let p1 = calculate_p1(&nums);
    let p2 = calculate_p2(&nums);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_02");

    fn test_in_out(inp: &[i64], out: &[i64]) {
        let mut prog = inp.into();
        execute_until_halt(&mut prog);
        assert_eq!(prog.state, out);
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
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 9581917);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 2505);
    }
}
