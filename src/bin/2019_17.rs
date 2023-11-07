use advent_of_code_2019::intcode::IntCodeState;
use clap::Parser;
use itertools::intersperse;
use itertools::Itertools;
use ndarray::Array2;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fs;
use std::iter::zip;

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

fn parse_map(inp: Vec<u8>) -> Array2<u8> {
    let columns = inp.iter().position(|&c| c == b'\n').expect("no newline");

    let data = inp
        .iter()
        .filter(|&&c| c != b'\n')
        .copied()
        .collect::<Vec<u8>>();

    Array2::from_shape_vec((data.len() / columns, columns), data).expect("invalid array")
}

fn calculate_p1(view: &Array2<u8>) -> usize {
    let mut result = 0;
    for row in 1..(view.dim().0 - 1) {
        for col in 1..(view.dim().1 - 1) {
            let this = view[(row, col)] == b'#';
            let up = view[(row + 1, col)] == b'#';
            let down = view[(row - 1, col)] == b'#';
            let left = view[(row, col - 1)] == b'#';
            let right = view[(row, col + 1)] == b'#';

            if this && up && down && left && right {
                result += row * col;
            }
        }
    }
    result
}

const DIRS: [(isize, isize); 4] = [
    // (y, x)
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

fn get_start_location(view: &Array2<u8>) -> Option<(usize, usize, usize)> {
    for row in 1..(view.dim().0 - 1) {
        for col in 1..(view.dim().1 - 1) {
            let ch = view[(row, col)];

            match ch {
                b'^' => {
                    return Some((row, col, 0));
                }
                b'>' => {
                    return Some((row, col, 1));
                }
                b'v' => {
                    return Some((row, col, 2));
                }
                b'<' => {
                    return Some((row, col, 3));
                }
                _ => {}
            }
        }
    }
    None
}

fn w_add(x: usize, y: isize) -> usize {
    x.wrapping_add_signed(y)
}

fn get_path(view: &Array2<u8>) -> Vec<&str> {
    let start = get_start_location(view).expect("couldn't find start");
    let mut y = start.0;
    let mut x = start.1;
    let mut dir: usize = start.2;

    let mut path: Vec<&str> = vec![];

    loop {
        let left_dir = DIRS[w_add(dir, -1).rem_euclid(4)];
        let right_dir = DIRS[w_add(dir, 1).rem_euclid(4)];

        let left_cell = view.get((w_add(y, left_dir.0), w_add(x, left_dir.1)));
        let right_cell = view.get((w_add(y, right_dir.0), w_add(x, right_dir.1)));

        if left_cell == Some(&b'#') {
            path.push("L");
            dir = w_add(dir, -1);
        } else if right_cell == Some(&b'#') {
            path.push("R");
            dir = w_add(dir, 1);
        } else {
            break;
        }

        dir = dir.rem_euclid(4);

        let mut dist = 0;
        while view.get((w_add(y, DIRS[dir].0), w_add(x, DIRS[dir].1))) == Some(&b'#') {
            y = w_add(y, DIRS[dir].0);
            x = w_add(x, DIRS[dir].1);
            dist += 1;
        }

        path.push(dist.to_string().leak());
    }

    path
}

fn vec_starts_with(v: &[&str], other: &[&str]) -> bool {
    v.len() >= other.len() && zip(v, other).all(|(x, y)| x == y)
}

fn can_build_path_from_fragments(path: &[&str], a: &[&str], b: &[&str], c: &[&str]) -> bool {
    if path.is_empty() {
        true
    } else {
        for fragment in [a, b, c] {
            if vec_starts_with(path, fragment) {
                return can_build_path_from_fragments(&path[fragment.len()..], a, b, c);
            }
        }
        false
    }
}

fn fragment_is_legal(fragment: &[&str]) -> bool {
    fragment.iter().join(",").len() < 20
}

fn split_path<'a>(path: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>, Vec<&'a str>) {
    for a_length in (1..9).rev() {
        let a = &path[0..a_length];
        if !fragment_is_legal(a) {
            continue;
        }
        for b_start in a_length..(path.len() - 2) {
            for b_length in (1..9).rev() {
                let b = &path[b_start..min(b_start + b_length, path.len() - 1)];
                if !fragment_is_legal(b) {
                    continue;
                }
                for c_start in max(a_length, b_length)..(path.len() - 1) {
                    for c_length in (1..9).rev() {
                        let c = &path[c_start..min(c_start + c_length, path.len())];
                        if !fragment_is_legal(c) {
                            continue;
                        }

                        if can_build_path_from_fragments(path, a, b, c) {
                            return (a.to_vec(), b.to_vec(), c.to_vec());
                        }
                    }
                }
            }
        }
    }

    panic!("can't build path");
}

fn program_from_fragments(path: &[&str], a: &[&str], b: &[&str], c: &[&str]) -> Vec<&'static str> {
    let mut result = vec![];
    let mut path = path;

    while !path.is_empty() {
        if vec_starts_with(path, a) {
            result.push("A");
            path = &path[a.len()..];
        } else if vec_starts_with(path, b) {
            result.push("B");
            path = &path[b.len()..];
        } else if vec_starts_with(path, c) {
            result.push("C");
            path = &path[c.len()..];
        } else {
            panic!("invalid fragments");
        }
    }

    result
}

fn calculate(software: &[i64]) -> (usize, i64) {
    let mut prog_p1: IntCodeState = software.into();

    prog_p1.execute_until_halt_no_input();

    let view = parse_map(
        prog_p1
            .out_buffer
            .iter()
            .map(|&c| u8::try_from(c).expect("invalid char"))
            .collect::<Vec<u8>>(),
    );

    let mut prog_p2: IntCodeState = software.into();

    let path = get_path(&view);

    let (a, b, c) = split_path(&path);

    let prog = program_from_fragments(&path, &a, &b, &c);

    let mut p2_prog_inputs = intersperse(prog.iter(), &",")
        .chain(Some(&"\n"))
        .chain(intersperse(a.iter(), &","))
        .chain(Some(&"\n"))
        .chain(intersperse(b.iter(), &","))
        .chain(Some(&"\n"))
        .chain(intersperse(c.iter(), &","))
        .chain(Some(&"\n"))
        .chain(Some(&"n"))
        .chain(Some(&"\n"))
        .flat_map(|s| s.bytes())
        .map(i64::from)
        .collect::<VecDeque<_>>();

    prog_p2.set_mem(0, 2);
    prog_p2.execute_until_halt(|_| p2_prog_inputs.pop_front());

    (
        calculate_p1(&view),
        prog_p2.out_buffer.pop_back().expect("no output p2"),
    )
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums = parse(&inp);
    let (p1, p2) = calculate(&nums);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_17");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)).0, 7280);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)).1, 1045393);
    }
}
