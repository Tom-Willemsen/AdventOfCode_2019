use advent_of_code_2019::intcode::{parse_intcode_to_vec, IntCodeState};
use advent_of_code_2019::{Cli, Parser};
use std::collections::VecDeque;
use std::fs;

fn calculate<const PART: u8>(software: &[i64]) -> i64 {
    let mut nics = (0..50)
        .map(|_| software.into())
        .collect::<Vec<IntCodeState<4096>>>();

    let mut input_buffers = (0..50)
        .map(|i| {
            let mut q = VecDeque::with_capacity(16);
            q.push_back(i);
            q
        })
        .collect::<Vec<_>>();

    let mut last_nat_y = None;

    let mut nat_x = None;
    let mut nat_y = None;

    loop {
        let mut all_waiting_for_input = true;

        for (i, nic) in nics.iter_mut().enumerate() {
            let mut waiting_for_input = false;

            while !waiting_for_input {
                nic.execute_single_step(|_| {
                    if let Some(inp) = input_buffers[i].pop_front() {
                        Some(inp)
                    } else {
                        waiting_for_input = true;
                        Some(-1)
                    }
                });
            }

            all_waiting_for_input = all_waiting_for_input && waiting_for_input;

            while nic.out_buffer.len() >= 3 {
                let dest = nic.out_buffer.pop_front().unwrap();
                let x = nic.out_buffer.pop_front().unwrap();
                let y = nic.out_buffer.pop_front().unwrap();

                if dest == 255 {
                    if PART == 1 {
                        return y;
                    } else {
                        nat_x = Some(x);
                        nat_y = Some(y);
                    }
                } else {
                    input_buffers[dest as usize].push_back(x);
                    input_buffers[dest as usize].push_back(y);
                }
            }
        }

        if PART == 2 && all_waiting_for_input && input_buffers.iter().all(|b| b.is_empty()) {
            if let Some(nat_y) = nat_y {
                if Some(nat_y) == last_nat_y {
                    return nat_y;
                }
                input_buffers[0].push_back(nat_x.expect("nat has no packet"));
                input_buffers[0].push_back(nat_y);
                last_nat_y = Some(nat_y);
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");
    let inp_vec = parse_intcode_to_vec(&inp);
    let p1 = calculate::<1>(&inp_vec);
    let p2 = calculate::<2>(&inp_vec);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_23");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&parse_intcode_to_vec(&REAL_DATA)), 23626);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&parse_intcode_to_vec(&REAL_DATA)), 19019);
    }
}
