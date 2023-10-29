use advent_of_code_2019::intcode::IntCodeState;
use ahash::AHashMap;
use clap::Parser;
use std::collections::BinaryHeap;
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

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Space {
    Unknown,
    Empty,
    Wall,
    DeadEnd,
}

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const DIR_NORTH: usize = 0;
const DIR_EAST: usize = 1;
const DIR_SOUTH: usize = 2;
const DIR_WEST: usize = 3;

const COMMAND_NORTH: i64 = 1;
const COMMAND_SOUTH: i64 = 2;
const COMMAND_WEST: i64 = 3;
const COMMAND_EAST: i64 = 4;

fn dijkstra(map: &AHashMap<(i64, i64), Space>, sensor_loc: (i64, i64)) -> (i64, i64) {
    let mut heap = BinaryHeap::new();
    heap.push((0, sensor_loc));

    let mut costs: AHashMap<(i64, i64), i64> = AHashMap::with_capacity(map.len());

    while let Some((cost, pos)) = heap.pop() {
        if cost < *costs.get(&pos).unwrap_or(&i64::MIN) {
            continue;
        }

        for dir in DIRS {
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            let next_cost = cost - 1;

            if map.get(&next_pos).unwrap() != &Space::Wall {
                let next = (next_cost, next_pos);

                if next_cost > *costs.get(&next_pos).unwrap_or(&i64::MIN) {
                    heap.push(next);
                    costs.insert(next_pos, next_cost);
                }
            }
        }
    }

    let p1 = *costs.get(&(0, 0)).unwrap();
    let p2 = *costs.values().min().unwrap();
    (-p1, -p2)
}

fn try_move(
    north: Space,
    south: Space,
    east: Space,
    west: Space,
    move_to: Space,
) -> Option<(usize, i64)> {
    if north == move_to {
        Some((DIR_NORTH, COMMAND_NORTH))
    } else if east == move_to {
        Some((DIR_EAST, COMMAND_EAST))
    } else if south == move_to {
        Some((DIR_SOUTH, COMMAND_SOUTH))
    } else if west == move_to {
        Some((DIR_WEST, COMMAND_WEST))
    } else {
        None
    }
}

fn calculate(software: &[i64]) -> (i64, i64) {
    let mut prog: IntCodeState = software.into();

    let mut known: AHashMap<(i64, i64), Space> = AHashMap::with_capacity(1024);
    known.insert((0, 0), Space::Empty);

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut dir: usize = DIR_NORTH;
    let mut sensor_pos = None;
    let mut fully_explored = false;

    while !fully_explored {
        prog.execute_single_step(|s| {
            if let Some(output) = s.out_buffer.pop_front() {
                if output == 1 || output == 2 {
                    x += DIRS[dir].0;
                    y += DIRS[dir].1;
                    known.insert((x, y), Space::Empty);
                    if output == 2 {
                        sensor_pos = Some((x, y));
                    }
                } else if output == 0 {
                    known.insert((x + DIRS[dir].0, y + DIRS[dir].1), Space::Wall);
                }

                let north = *known.get(&(x, y + 1)).unwrap_or(&Space::Unknown);
                let south = *known.get(&(x, y - 1)).unwrap_or(&Space::Unknown);
                let east = *known.get(&(x + 1, y)).unwrap_or(&Space::Unknown);
                let west = *known.get(&(x - 1, y)).unwrap_or(&Space::Unknown);

                if let Some((d, command)) = try_move(north, south, east, west, Space::Unknown) {
                    dir = d;
                    return Some(command);
                }

                let wall_count = [north, south, east, west]
                    .iter()
                    .filter(|&x| *x == Space::Wall || *x == Space::DeadEnd)
                    .count();

                if wall_count == 3 {
                    known.insert((x, y), Space::DeadEnd);
                }

                if let Some((d, command)) = try_move(north, south, east, west, Space::Empty) {
                    dir = d;
                    return Some(command);
                }

                fully_explored = true;
                None
            } else {
                Some(COMMAND_NORTH)
            }
        });
    }

    dijkstra(&known, sensor_pos.unwrap())
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_15");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)).0, 374);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)).1, 482);
    }
}
