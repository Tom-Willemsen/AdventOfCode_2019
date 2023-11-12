use ahash::AHashMap;
use clap::Parser;
use mimalloc::MiMalloc;
use ndarray::Array2;
use std::collections::BinaryHeap;
use std::fs;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Array2<u8> {
    let columns = raw_inp
        .lines()
        .map(|line| line.len())
        .max()
        .expect("no lines?");

    let data = raw_inp.bytes().filter(|&c| c != b'\n').collect::<Vec<u8>>();

    Array2::from_shape_vec((data.len() / columns, columns), data).expect("invalid array")
}

// (y, x)
const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const POS_DIRS: [(isize, isize); 2] = [(0, 1), (1, 0)];
const NEG_DIRS: [(isize, isize); 2] = [(0, -1), (-1, 0)];

fn get_portal_key(data: &Array2<u8>, y: usize, x: usize) -> String {
    let portal1 = data.get((y, x)).expect("invalid portal1");

    let portal2_pos = POS_DIRS
        .iter()
        .map(|dir| (y.wrapping_add_signed(dir.0), x.wrapping_add_signed(dir.1)))
        .filter_map(|coord| data.get(coord))
        .find(|&itm| itm.is_ascii_uppercase());

    if let Some(p) = portal2_pos {
        return format!("{}{}", *portal1 as char, *p as char);
    }

    let portal2_neg = NEG_DIRS
        .iter()
        .map(|dir| (y.wrapping_add_signed(dir.0), x.wrapping_add_signed(dir.1)))
        .filter_map(|coord| data.get(coord))
        .find(|&itm| itm.is_ascii_uppercase());

    if let Some(p) = portal2_neg {
        return format!("{}{}", *p as char, *portal1 as char);
    }

    panic!("can't get portal key");
}

fn is_near_map(data: &Array2<u8>, y: usize, x: usize) -> bool {
    DIRS.iter()
        .map(|dir| (y.wrapping_add_signed(dir.0), x.wrapping_add_signed(dir.1)))
        .filter_map(|coord| data.get(coord))
        .any(|&itm| itm == b'.')
}

fn positions_of(data: &Array2<u8>, portal: &str) -> Vec<(usize, usize)> {
    data.indexed_iter()
        .filter(|&(_, itm)| itm.is_ascii_uppercase())
        .filter(|&(pos, _)| is_near_map(data, pos.0, pos.1))
        .map(|(pos, _)| (pos, get_portal_key(data, pos.0, pos.1)))
        .filter(|(_, itm)| itm == portal)
        .map(|(pos, _)| pos)
        .collect()
}

fn is_outer_portal(data: &Array2<u8>, y: usize, x: usize) -> bool {
    let (ydim, xdim) = data.dim();
    y == 0
        || y == 1
        || y == ydim - 1
        || y == ydim - 2
        || x == 0
        || x == 1
        || x == xdim - 1
        || x == xdim - 2
}

fn dijkstra<const PART: u8>(data: &Array2<u8>) -> i64 {
    let mut heap: BinaryHeap<(i64, (usize, usize), usize)> = BinaryHeap::new();

    let aa_positions = positions_of(data, "AA");
    let start_pos = aa_positions.first().expect("can't find start");

    let zz_positions = positions_of(data, "ZZ");
    let end_pos = zz_positions.first().expect("can't find end");

    heap.push((0, *start_pos, 0));

    let mut costs: AHashMap<((usize, usize), usize), i64> = AHashMap::with_capacity(1024);

    while let Some((cost, mut pos, mut level)) = heap.pop() {
        if cost < *costs.get(&(pos, level)).unwrap_or(&i64::MIN) {
            continue;
        }

        let this_tile = data.get(pos).expect("invalid current pos");

        if this_tile.is_ascii_uppercase() {
            // If standing in a portal, teleport to new location then continue pathfind.
            let pkey = get_portal_key(data, pos.0, pos.1);

            if PART == 2 && pkey != "AA" {
                if pkey == "ZZ" && level > 0 {
                    continue;
                }
                if is_outer_portal(data, pos.0, pos.1) {
                    if level >= 1 {
                        level -= 1;
                    } else {
                        continue;
                    }
                } else {
                    level += 1;
                }
            }

            if pkey != "AA" && pkey != "ZZ" {
                pos = *positions_of(data, &pkey)
                    .iter()
                    .find(|&p| p != &pos)
                    .expect("can't find corresponding portal");
            }
        }

        for dir in DIRS {
            let next_pos = (
                pos.0.checked_add_signed(dir.0).unwrap(),
                pos.1.checked_add_signed(dir.1).unwrap(),
            );

            let next_tile = data.get(next_pos).expect("invalid next pos");

            if next_tile == &b'#'
                || next_tile == &b' '
                || (this_tile.is_ascii_uppercase() && next_tile.is_ascii_uppercase())
            {
                continue;
            }

            let next_cost = if this_tile.is_ascii_uppercase() {
                cost
            } else {
                cost - 1
            };

            if next_cost > *costs.get(&(next_pos, level)).unwrap_or(&i64::MIN) {
                heap.push((next_cost, next_pos, level));
                costs.insert((next_pos, level), next_cost);

                if level == 0 && next_pos == *end_pos {
                    return -next_cost - 1;
                }
            }
        }
    }

    -costs.get(&(*end_pos, 0)).expect("couldn't pathfind to end") - 1
}

fn calculate_p1(data: &Array2<u8>) -> i64 {
    dijkstra::<1>(data)
}

fn calculate_p2(data: &Array2<u8>) -> i64 {
    dijkstra::<2>(data)
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2019_20");
    const EXAMPLE_DATA_SMALL: &str = include_str!("../../inputs/examples/2019_20_small");
    const EXAMPLE_DATA_P2_LARGE: &str = include_str!("../../inputs/examples/2019_20_p2_large");
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_20");

    #[test]
    fn test_p1_small_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_SMALL)), 23);
    }

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA)), 58);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 690);
    }

    #[test]
    fn test_p2_large_example() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_P2_LARGE)), 396);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 7976);
    }
}
