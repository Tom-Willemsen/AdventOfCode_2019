use advent_of_code_2019::{Cli, Parser};
use mod_exp::mod_exp;
use modinverse::modinverse;
use std::fs;

const CUT: u8 = b'c';
const DEAL: u8 = b'd';
const REVERSE: u8 = b'r';

fn parse(raw_inp: &str) -> Vec<(u8, i64)> {
    raw_inp
        .trim()
        .lines()
        .map(|line| {
            if let Some(n) = line.strip_prefix("cut ") {
                let n: i64 = n.parse().unwrap();
                (CUT, n)
            } else if let Some(n) = line.strip_prefix("deal with increment ") {
                let n: i64 = n.parse().unwrap();
                (DEAL, n)
            } else {
                (REVERSE, 0)
            }
        })
        .collect()
}

const P1_POSITION: i64 = 2019;
const P2_POSITION: i64 = 2020;
const P1_CARDS: i64 = 10007;
const P2_CARDS: i64 = 119315717514047;
const P2_NUM_SHUFFLES: i64 = 101741582076661;

fn forwards_step<const NUM_CARDS: i64>(data: &[(u8, i64)], pos: i64) -> i64 {
    let mut position = pos;

    for (typ, n) in data.iter() {
        if typ == &CUT {
            position -= n;
            position = position.rem_euclid(NUM_CARDS);
        } else if typ == &DEAL {
            position *= n;
            position = position.rem_euclid(NUM_CARDS);
        } else {
            position = NUM_CARDS - 1 - position;
        }
    }

    position
}

fn calc_coefficients<const NUM_CARDS: i64>(data: &[(u8, i64)]) -> (i64, i64) {
    // Express the shuffle as an equation of the form:
    // f(x) = ax + b
    // Where f(x) gives the new position of element x after one round of shuffling.

    let mut a = 1;
    let mut b = 0;

    for (typ, n) in data.iter() {
        if typ == &CUT {
            b -= n;
        } else if typ == &DEAL {
            b *= n;
            a *= n;
        } else {
            a *= -1;
            b = NUM_CARDS - 1 - b;
        }

        b = b.rem_euclid(NUM_CARDS);
        a = a.rem_euclid(NUM_CARDS);
    }

    (a, b)
}

fn calculate_p1(data: &[(u8, i64)]) -> i64 {
    forwards_step::<P1_CARDS>(data, P1_POSITION)
}

fn n_forwards_steps<const NUM_CARDS: i64>(data: &[(u8, i64)], x: i64, n: i64) -> i64 {
    // From https://en.wikipedia.org/wiki/Iterated_function
    // We have a closed form solution for nth iterate of ax + b
    // which is:
    // a^n * x + ((a^n - 1)/(a-1)) * b
    // First get the coefficients a and b:
    let (a, b) = calc_coefficients::<P2_CARDS>(data);

    // We need to multiply together ~46 bit numbers in a few places below
    // So use 128-bit types within this function.
    let x = x as i128;
    let a = a as i128;
    let b = b as i128;
    let n = n as i128;
    let m = NUM_CARDS as i128;

    // Fast modular exponentiation of a^n (mod m)
    let an_mod_m = mod_exp(a, n, m);

    // (a^n)*x
    let anx = an_mod_m * x;

    // Directly computing b*(a^n - 1)/(a - 1) isn't possible. We need to express
    // the computation in terms of (a^n (mod m)).
    // To do that we need the modinverse of (a-1), so we can multiply top and bottom
    // of fraction by it.
    let inv_a_minus_1 = modinverse(a - 1, m).expect("can't calculate modinverse");

    // Take modulo at end to keep numbers inside 128-bit range.
    let inv_a_minus_1_times_b = (b * inv_a_minus_1).rem_euclid(m);

    let result = anx + inv_a_minus_1_times_b * (an_mod_m - 1);

    // Ensure final result is within range 0 <= result < m as required. Now safe
    // to go back to normal 64-bit types.
    result.rem_euclid(m).try_into().unwrap()
}

fn calculate_p2(data: &[(u8, i64)]) -> i64 {
    // Going "backwards" by N shuffles is the same as going forwards CARDS - N - 1 shuffles
    // by the pigeonhole principle.
    n_forwards_steps::<P2_CARDS>(data, P2_POSITION, P2_CARDS - P2_NUM_SHUFFLES - 1)
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_22");

    #[test]
    fn test_p1_simple_example_1() {
        let mut result = vec![0; 10];

        let data = parse(
            "deal with increment 7
deal into new stack
deal into new stack",
        );

        for i in 0..10 {
            result[forwards_step::<10>(&data, i) as usize] = i;
        }

        assert_eq!(result, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_p1_simple_example_2() {
        let mut result = vec![0; 10];

        let data = parse(
            "cut 6
deal with increment 7
deal into new stack",
        );

        for i in 0..10 {
            result[forwards_step::<10>(&data, i) as usize] = i;
        }

        assert_eq!(result, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_p1_simple_example_3() {
        let mut result = vec![0; 10];

        let data = parse(
            "deal with increment 7
deal with increment 9
cut -2",
        );

        for i in 0..10 {
            result[forwards_step::<10>(&data, i) as usize] = i;
        }

        assert_eq!(result, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_p1_simple_example_4() {
        let mut result = vec![0; 10];

        let data = parse(
            "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
        );

        for i in 0..10 {
            result[forwards_step::<10>(&data, i) as usize] = i;
        }

        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 6326);
    }

    #[test]
    fn test_forward_n_gives_same_result_as_iterated_f() {
        let start = 2020;
        let mut p = start;

        let data = parse(&REAL_DATA);

        // Check iteration gives same answers as direct calculation
        // for first 1000 rounds.
        for n in 1..=1000 {
            p = forwards_step::<P2_CARDS>(&data, p);

            assert_eq!(p, n_forwards_steps::<P2_CARDS>(&data, 2020, n));
        }
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 40522432670594);
    }
}
