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
        .chars()
        .map(|x| i64::from(x.to_digit(10).unwrap()))
        .collect()
}

fn pat(offset: usize, pos: usize) -> i64 {
    let one_start = offset;
    let one_end = one_start + offset + 1;
    let minus_one_start = one_end + offset + 1;
    let minus_one_end = minus_one_start + offset + 1;

    let pos = pos % (minus_one_end + 1);

    if pos >= one_start && pos < one_end {
        1
    } else if pos >= minus_one_start && pos < minus_one_end {
        -1
    } else {
        0
    }
}

fn apply_phase(data: &[i64]) -> Vec<i64> {
    (0..data.len())
        .map(|i| {
            data.iter()
                .enumerate()
                .map(|(j, &m)| pat(i, j) * m)
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect()
}

fn calculate_p1(data: &[i64]) -> String {
    let mut data = data.to_vec();

    for _ in 0..100 {
        data = apply_phase(&data);
    }

    data.iter().take(8).map(|x| x.to_string()).join("")
}

fn calculate_p2(data: &[i64]) -> String {
    let mut real_signal = Vec::with_capacity(data.len() * 10000);

    for _ in 0..10000 {
        real_signal.extend(data);
    }

    for phase in 0..100 {
        println!("p2 phase {}", phase);
        real_signal = apply_phase(&real_signal);
    }

    let skip = data[6]
        + data[5] * 10
        + data[4] * 100
        + data[3] * 1000
        + data[2] * 10000
        + data[1] * 100000
        + data[0] * 1000000;

    real_signal
        .iter()
        .skip(usize::try_from(skip).unwrap())
        .take(8)
        .map(|x| x.to_string())
        .join("")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_EXAMPLE: &str = "12345678";
    const EXAMPLE_DATA_1: &str = "80871224585914546619083218645595";
    const EXAMPLE_DATA_2: &str = "19617804207202209144916044189917";
    const EXAMPLE_DATA_3: &str = "69317163492948606335995924319873";

    const EXAMPLE_DATA_4: &str = "03036732577212944063491565474664";
    const EXAMPLE_DATA_5: &str = "02935109699940807407585447034323";
    const EXAMPLE_DATA_6: &str = "03081770884921959731165446850517";

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_16");

    #[test]
    fn test_pat() {
        assert_eq!(pat(0, 0), 1);
        assert_eq!(pat(1, 0), 0);
        assert_eq!(pat(2, 0), 0);
        assert_eq!(pat(3, 0), 0);
        assert_eq!(pat(4, 0), 0);
        assert_eq!(pat(5, 0), 0);
        assert_eq!(pat(6, 0), 0);
        assert_eq!(pat(7, 0), 0);
        assert_eq!(pat(8, 0), 0);

        assert_eq!(pat(0, 1), 0);
        assert_eq!(pat(1, 1), 1);
        assert_eq!(pat(2, 1), 0);
        assert_eq!(pat(3, 1), 0);
        assert_eq!(pat(4, 1), 0);
        assert_eq!(pat(5, 1), 0);
        assert_eq!(pat(6, 1), 0);
        assert_eq!(pat(7, 1), 0);
        assert_eq!(pat(8, 1), 0);

        assert_eq!(pat(0, 2), -1);
        assert_eq!(pat(1, 2), 1);
        assert_eq!(pat(2, 2), 1);
        assert_eq!(pat(3, 2), 0);
        assert_eq!(pat(4, 2), 0);
        assert_eq!(pat(5, 2), 0);
        assert_eq!(pat(6, 2), 0);
        assert_eq!(pat(7, 2), 0);
        assert_eq!(pat(8, 2), 0);
    }

    #[test]
    fn test_p1_simple_example() {
        let data = parse(&SIMPLE_EXAMPLE).to_vec();

        let p1 = apply_phase(&data);
        assert_eq!(p1, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        let p2 = apply_phase(&p1);
        assert_eq!(p2, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        let p3 = apply_phase(&p2);
        assert_eq!(p3, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        let p4 = apply_phase(&p3);
        assert_eq!(p4, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_p1_example_1() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_1)), "24176176");
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_2)), "73745418");
    }

    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_3)), "52432133");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), "23135243");
    }

    //     #[test]
    //     fn test_p2_example_1() {
    //         assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_4)), "84462026");
    //     }
    //
    //     #[test]
    //     fn test_p2_example_2() {
    //         assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_5)), "78725270");
    //     }
    //
    //     #[test]
    //     fn test_p2_example_3() {
    //         assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_6)), "53553731");
    //     }
    //
    //     #[test]
    //     fn test_p2_real() {
    //         assert!(calculate_p2(&parse(&REAL_DATA)) != "23135243");  // too high
    //     }
}
