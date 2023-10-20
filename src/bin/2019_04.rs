use clap::Parser;
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

/// returns (p1_valid, p2_valid)
fn is_valid(bytes: &[u32; 6]) -> (bool, bool) {
    let mut arr: [u8; 10] = [0; 10];

    bytes.iter().for_each(|&b| {
        arr[usize::try_from(b).unwrap()] += 1;
    });

    // 2 or more of the same number -> p1 is valid
    // *exactly* 2 of the same number -> p2 is valid
    (arr.iter().any(|&x| x >= 2), arr.contains(&2))
}

fn to_num(b1: u32, b2: u32, b3: u32, b4: u32, b5: u32, b6: u32) -> u32 {
    100_000 * b1 + 10_000 * b2 + 1000 * b3 + 100 * b4 + 10 * b5 + b6
}

fn calculate(start: u32, end: u32) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;

    let b1_start = (start / 100_000) % 10;
    let b1_end = (end / 100_000) % 10;

    // Messy, but a lot faster than the naive guess-and-check approach.
    // This enforces the non-decreasing digits rule
    for b1 in b1_start..=b1_end {
        for b2 in b1..=9 {
            for b3 in b2..=9 {
                for b4 in b3..=9 {
                    for b5 in b4..=9 {
                        for b6 in b5..=9 {
                            let n = to_num(b1, b2, b3, b4, b5, b6);
                            if n > end {
                                return (p1, p2);
                            }
                            if n >= start {
                                let (p1_is_valid, p2_is_valid) =
                                    is_valid(&[b1, b2, b3, b4, b5, b6]);
                                if p1_is_valid {
                                    p1 += 1;
                                }
                                if p2_is_valid {
                                    p2 += 1;
                                }
                            }
                        }
                    }
                }
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
