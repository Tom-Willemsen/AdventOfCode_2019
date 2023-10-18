use clap::Parser;
use itertools::Itertools;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> (u32, u32) {
    raw_inp
        .trim()
        .split_once('-')
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .expect("invalid format")
}

fn p1_is_valid(bytes: &[u8; 6]) -> bool {
    bytes.iter().tuple_windows().all(|(x, y)| y >= x)
        && bytes.iter().tuple_windows().any(|(x, y)| x == y)
}

fn p2_is_valid(bytes: &[u8; 6]) -> bool {
    let mut arr: [u8; 10] = [0; 10];

    bytes.iter().for_each(|&b| {
        arr[usize::from(b)] += 1;
    });

    arr.contains(&2)
}

fn to_bytes(n: u32) -> [u8; 6] {
    [
        ((n / 100_000) % 10) as u8,
        ((n / 10_000) % 10) as u8,
        ((n / 1_000) % 10) as u8,
        ((n / 100) % 10) as u8,
        ((n / 10) % 10) as u8,
        (n % 10) as u8,
    ]
}

fn calculate(start: u32, end: u32) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;
    for n in start..=end {
        let b = to_bytes(n);
        if p1_is_valid(&b) {
            p1 += 1;
            if p2_is_valid(&b) {
                p2 += 1;
            }
        }
    }
    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (start, end) = parse(&inp);
    let (p1, p2) = calculate(start, end);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_04");

    #[test]
    fn test_p1_real() {
        let (start, end) = parse(&REAL_DATA);
        assert_eq!(calculate(start, end).0, 1675);
    }

    #[test]
    fn test_p2_real() {
        let (start, end) = parse(&REAL_DATA);
        assert_eq!(calculate(start, end).1, 1142);
    }
}
