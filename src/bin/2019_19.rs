use advent_of_code_2019::intcode::IntCodeState;
use clap::Parser;
use rayon::prelude::*;
use std::cmp::max;
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

fn is_in_beam(software: &[i64], x: usize, y: usize) -> bool {
    let mut prog: IntCodeState = software.into();
    let mut inputs = vec![y as i64, x as i64];

    prog.execute_until_halt(|_| inputs.pop());

    prog.out_buffer.pop_front().unwrap() == 1
}

const SANTA_SHIP_SIZE: usize = 100;

fn calculate_p1(software: &[i64]) -> usize {
    (0..50)
        .into_par_iter()
        .map(|y| {
            (0..50)
                .into_par_iter()
                .filter(|&x| is_in_beam(software, x, y))
                .count()
        })
        .sum()
}

fn calculate_p2(software: &[i64]) -> usize {
    let mut starts_ends: Vec<(usize, usize)> = vec![];
    let mut y: usize = 0;

    loop {
        let (mut start, mut end) = if y == 0 { (0, 0) } else { starts_ends[y - 1] };

        while !is_in_beam(software, start, y) {
            start += 1;

            // Annoying breakout if the beam isn't detected at all on
            // a particular row. Apparently not needed for some inputs?
            // definitely needed for mine.
            if start > 5 * y {
                start = 0;
                break;
            }
        }

        end = max(start + 1, end);

        while is_in_beam(software, end, y) {
            end += 1;
        }

        if y > SANTA_SHIP_SIZE {
            if let Some((_, top_end)) = starts_ends.get(y - SANTA_SHIP_SIZE + 1) {
                if top_end.saturating_sub(SANTA_SHIP_SIZE) >= start {
                    return (y - SANTA_SHIP_SIZE + 1) + 10000 * start;
                }
            }
        }

        starts_ends.push((start, end));
        y += 1;
    }
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2(&data));
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_19");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 131);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 15231022);
    }
}
