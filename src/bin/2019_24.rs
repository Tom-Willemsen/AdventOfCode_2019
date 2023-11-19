use advent_of_code_2019::{Cli, Parser};
use ahash::AHashSet;
use ndarray::{Array2, Array3};
use std::fs;

fn parse(raw_inp: &str) -> Array2<bool> {
    let v = raw_inp
        .trim()
        .bytes()
        .filter(|&b| b == b'#' || b == b'.')
        .map(|b| b == b'#')
        .collect::<Vec<bool>>();

    Array2::from_shape_vec((5, 5), v).expect("wrong input shape")
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn next_bug_state(current_bug_state: bool, count: usize) -> bool {
    if current_bug_state && count != 1 {
        false
    } else if !current_bug_state && (count == 1 || count == 2) {
        true
    } else {
        current_bug_state
    }
}

fn step_p1(data: &Array2<bool>) -> Array2<bool> {
    Array2::from_shape_fn(data.dim(), |(y, x)| {
        let count = DIRS
            .iter()
            .filter_map(|&(yd, xd)| Some((y.checked_add_signed(yd)?, x.checked_add_signed(xd)?)))
            .filter_map(|pos| data.get(pos))
            .filter(|&&x| x)
            .count();

        next_bug_state(data[(y, x)], count)
    })
}

fn calculate_p1(data: &Array2<bool>) -> u32 {
    let mut seen = AHashSet::default();
    let mut data = data.clone();
    loop {
        if !seen.insert(data.clone()) {
            break;
        }
        data = step_p1(&data);
    }

    data.indexed_iter()
        .filter(|&(_, &itm)| itm)
        .map(|((y, x), _)| 2u32.pow(u32::try_from(5 * y + x).unwrap()))
        .sum()
}

fn count_outer_neighbours(data: &Array3<bool>, z: usize, y: usize, x: usize) -> usize {
    let mut count = 0;
    if z + 1 < data.dim().0 {
        if (y == 0 && data[(z + 1, 1, 2)]) || (y == 4 && data[(z + 1, 3, 2)]) {
            count += 1;
        }
        if (x == 0 && data[(z + 1, 2, 1)]) || (x == 4 && data[(z + 1, 2, 3)]) {
            count += 1;
        }
    }
    count
}

fn count_inner_neighbours(data: &Array3<bool>, z: usize, y: usize, x: usize) -> usize {
    let mut count = 0;
    if z == 0 {
        return 0;
    }

    if x == 2 && y == 1 {
        for inner_x in 0..5 {
            if data[(z - 1, 0, inner_x)] {
                count += 1;
            }
        }
    } else if x == 2 && y == 3 {
        for inner_x in 0..5 {
            if data[(z - 1, 4, inner_x)] {
                count += 1;
            }
        }
    } else if x == 1 && y == 2 {
        for inner_y in 0..5 {
            if data[(z - 1, inner_y, 0)] {
                count += 1;
            }
        }
    } else if x == 3 && y == 2 {
        for inner_y in 0..5 {
            if data[(z - 1, inner_y, 4)] {
                count += 1;
            }
        }
    }

    count
}

fn step_p2(data: &Array3<bool>) -> Array3<bool> {
    Array3::from_shape_fn(data.dim(), |(z, y, x)| {
        if y == 2 && x == 2 {
            // Middle tile always special/unoccupied
            return false;
        }

        let count = DIRS
            .iter()
            .filter_map(|&(yd, xd)| Some((y.checked_add_signed(yd)?, x.checked_add_signed(xd)?)))
            .filter_map(|(yd, xd)| data.get((z, yd, xd)))
            .filter(|&&x| x)
            .count()
            + count_outer_neighbours(data, z, y, x)
            + count_inner_neighbours(data, z, y, x);

        next_bug_state(data[(z, y, x)], count)
    })
}

fn calculate_p2<const STEPS: usize>(data: &Array2<bool>) -> usize {
    debug_assert!(STEPS % 2 == 0);

    let midpoint: usize = STEPS / 2;

    // If bugs spread at maximal rate, can infest an extra layer every 2 steps
    // So STEPS/2 in each direction (outward and inward) are sufficient.
    // i.e. STEPS+1 in total (+1 for initial state).
    let mut data = Array3::from_shape_fn((STEPS + 1, 5, 5), |(z, y, x)| {
        if z == midpoint {
            data[(y, x)]
        } else {
            false
        }
    });

    for _ in 0..STEPS {
        data = step_p2(&data);
    }

    data.iter().filter(|&&itm| itm).count()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2::<200>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2019_24");
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_24");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA)), 2129920);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2::<10>(&parse(&EXAMPLE_DATA)), 99);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 17863711);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2::<200>(&parse(&REAL_DATA)), 1937);
    }
}
