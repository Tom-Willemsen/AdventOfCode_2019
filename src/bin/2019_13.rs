use advent_of_code_2019::intcode::IntCodeState;
use advent_of_code_2019::{Cli, Parser};
use ahash::AHashSet;
use std::cmp::Ordering;
use std::fs;

fn calculate_p1(software: &str) -> usize {
    let mut blocks: AHashSet<(i64, i64)> = AHashSet::with_capacity(1024);

    let mut prog: IntCodeState<4096> = software.into();
    prog.execute_until_halt_no_input();

    while !prog.out_buffer.is_empty() {
        let x = prog.out_buffer.pop_front().expect("x should exist");
        let y = prog.out_buffer.pop_front().expect("y should exist");
        let tile = prog.out_buffer.pop_front().expect("tile should exist");
        if tile == 2 {
            blocks.insert((x, y));
        } else {
            blocks.remove(&(x, y));
        }
    }

    blocks.len()
}

fn calculate_p2(software: &str) -> i64 {
    let mut blocks: AHashSet<(i64, i64)> = AHashSet::with_capacity(1024);

    let mut prog: IntCodeState<4096> = software.into();
    prog.set_mem(0, 2);

    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        while prog.out_buffer.len() < 3 {
            prog.execute_single_step(|_| {
                Some(match paddle_x.cmp(&ball_x) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                })
            });
        }

        let x = prog.out_buffer.pop_front().expect("x should exist");
        let y = prog.out_buffer.pop_front().expect("y should exist");
        let tile_or_score = prog
            .out_buffer
            .pop_front()
            .expect("tile or score should exist");

        if x == -1 {
            if blocks.is_empty() {
                return tile_or_score;
            }
        } else if tile_or_score == 4 {
            ball_x = x;
        } else if tile_or_score == 3 {
            paddle_x = x;
        } else if tile_or_score == 2 {
            blocks.insert((x, y));
        } else if tile_or_score == 0 {
            blocks.remove(&(x, y));
        }
    }
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_13");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&REAL_DATA), 432);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&REAL_DATA), 22225);
    }
}
