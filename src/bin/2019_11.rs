use advent_of_code_2019::intcode::IntCodeState;
use advent_of_code_2019::{Cli, Parser};
use ahash::AHashMap;
use itertools::Itertools;
use std::fs;

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn paint<const INITIAL_TILE: bool>(software: &str) -> AHashMap<(i64, i64), bool> {
    let mut prog: IntCodeState<2048> = software.into();

    let mut painted: AHashMap<(i64, i64), bool> = AHashMap::default();

    let mut x = 0;
    let mut y = 0;
    let mut dir: i64 = 0;

    painted.insert((0, 0), INITIAL_TILE);

    prog.execute_until_halt(|state| {
        if !state.out_buffer.is_empty() {
            let colour = state.out_buffer.pop_front().expect("missing colour");
            let dir_instruction = state.out_buffer.pop_front().expect("missing dir");

            if dir_instruction == 1 {
                dir += 1;
            } else {
                dir -= 1;
            }

            let paint = colour == 1;
            painted.insert((x, y), paint);

            x += DIRS[dir.rem_euclid(4) as usize].0;
            y += DIRS[dir.rem_euclid(4) as usize].1;
        }
        if *painted.get(&(x, y)).unwrap_or(&false) {
            Some(1)
        } else {
            Some(0)
        }
    });

    painted
}

fn calculate_p1(software: &str) -> usize {
    let painted = paint::<false>(software);
    painted.len()
}

fn calculate_p2(software: &str) -> String {
    let painted = paint::<true>(software);

    let (min_x, max_x) = painted.keys().map(|p| p.0).minmax().into_option().unwrap();
    let (min_y, max_y) = painted.keys().map(|p| p.1).minmax().into_option().unwrap();

    let mut result = vec![];

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result.push(if *painted.get(&(x, y)).unwrap_or(&false) {
                "█"
            } else {
                " "
            });
        }
        result.push("\n");
    }

    result.join("")
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_11");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&REAL_DATA), 1967);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(
            calculate_p2(&REAL_DATA).trim(),
            "
 █  █ ███  █  █ ████  ██  ████ ███  █  █   
 █ █  █  █ █  █ █    █  █    █ █  █ █ █    
 ██   ███  █  █ ███  █      █  ███  ██     
 █ █  █  █ █  █ █    █ ██  █   █  █ █ █    
 █ █  █  █ █  █ █    █  █ █    █  █ █ █    
 █  █ ███   ██  ████  ███ ████ ███  █  █
"
            .trim()
        );
    }
}
