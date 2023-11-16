use ahash::AHashSet;
use clap::Parser;
use ndarray::{Array2, Array3};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

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

fn step_p1(data: &Array2<bool>) -> Array2<bool> {
    Array2::from_shape_fn(data.dim(), |(y, x)| {
        let count = DIRS
            .iter()
            .filter_map(|&(yd, xd)| Some((y.checked_add_signed(yd)?, x.checked_add_signed(xd)?)))
            .filter_map(|pos| data.get(pos))
            .filter(|&&x| x)
            .count();

        if data[(y, x)] && count != 1 {
            false
        } else if !data[(y, x)] && (count == 1 || count == 2) {
            true
        } else {
            data[(y, x)]
        }
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

fn step_p2(data: &Array3<bool>) -> Array3<bool> {
    Array3::from_shape_fn(data.dim(), |(z, y, x)| {
        if y == 2 && x == 2 {
            // Middle tile always special/unoccupied
            return false;
        }

        let mut count = DIRS
            .iter()
            .filter_map(|&(yd, xd)| Some((y.checked_add_signed(yd)?, x.checked_add_signed(xd)?)))
            .filter_map(|(yd, xd)| data.get((z, yd, xd)))
            .filter(|&&x| x)
            .count();

        // next bigger layer:
        if z + 1 < data.dim().0 {
            if y == 0 && data[(z + 1, 1, 2)] {
                count += 1
            }

            if y == 4 && data[(z + 1, 3, 2)] {
                count += 1
            }

            if x == 0 && data[(z + 1, 2, 1)] {
                count += 1
            }

            if x == 4 && data[(z + 1, 2, 3)] {
                count += 1;
            }
        }

        // smaller inner layer:
        if z > 0 {
            if x == 2 && y == 1 {
                for inner_x in 0..5 {
                    if data[(z - 1, 0, inner_x)] {
                        count += 1;
                    }
                }
            }

            if x == 2 && y == 3 {
                for inner_x in 0..5 {
                    if data[(z - 1, 4, inner_x)] {
                        count += 1;
                    }
                }
            }

            if x == 1 && y == 2 {
                for inner_y in 0..5 {
                    if data[(z - 1, inner_y, 0)] {
                        count += 1;
                    }
                }
            }

            if x == 3 && y == 2 {
                for inner_y in 0..5 {
                    if data[(z - 1, inner_y, 4)] {
                        count += 1;
                    }
                }
            }
        }

        if data[(z, y, x)] && count != 1 {
            false
        } else if !data[(z, y, x)] && (count == 1 || count == 2) {
            true
        } else {
            data[(z, y, x)]
        }
    })
}

fn calculate_p2<const STEPS: usize>(data: &Array2<bool>) -> usize {
    let mut data = Array3::from_shape_fn((STEPS * 2, 5, 5), |(z, y, x)| {
        if z == STEPS {
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
