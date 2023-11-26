use advent_of_code_2019::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use mimalloc::MiMalloc;
use ndarray::{s, Array2, ArrayView2};
use std::collections::BinaryHeap;
use std::fs;
use rayon::prelude::*;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

type PathCache<const AGENTS: usize> = AHashMap<(Vec<u8>, [u8; AGENTS], u8, usize), i64>;
type DijkstraCache = AHashMap<(Vec<u8>, usize, u8), Vec<(u8, i64)>>;

const CACHE_SIZE: usize = 32768;
const DIJKSTRA_CACHE_SIZE: usize = 8092;

fn parse(raw_inp: &str) -> Array2<u8> {
    let columns = raw_inp
        .bytes()
        .position(|c| c == b'\n')
        .expect("no newline");

    let data = raw_inp
        .trim()
        .bytes()
        .filter(|&c| c != b'\n')
        .collect::<Vec<u8>>();

    Array2::from_shape_vec((data.len() / columns, columns), data).expect("invalid array")
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[inline(never)]
fn position_of(data: &ArrayView2<u8>, ch: u8) -> (usize, usize) {
    data.indexed_iter()
        .filter(|(_, &itm)| itm == ch)
        .map(|(pos, _)| pos)
        .next()
        .expect("can't find pos")
}

/// Dijkstra, but over a map of direct connections rather than the "raw" map.
#[inline(never)]
fn reachable_keys(data: &AHashMap<u8, Vec<(u8, i64)>>, held_keys: &[u8], start_at: u8) -> Vec<(u8, i64)> {
    
    let mut heap = BinaryHeap::new();
    heap.push((0, start_at));

    let mut costs: AHashMap<u8, i64> = AHashMap::with_capacity(8);

    while let Some((cost, pos)) = heap.pop() {
        let neighbours = data.get(&pos).expect("invalid pos");
        
        for (n_pos, n_cost) in neighbours.iter() {

            if *n_pos == start_at {
                continue;
            }
            
            let is_closed_door = n_pos.is_ascii_uppercase()
                && !held_keys.contains(&n_pos.to_ascii_lowercase());
                
            if is_closed_door {
                continue;
            }
            
            let next_cost = cost - n_cost;

            if next_cost > *costs.get(n_pos).unwrap_or(&i64::MIN) {
                heap.push((next_cost, *n_pos));
                costs.insert(*n_pos, next_cost);
            }
        }
    }
    
    let res = costs.into_iter()
        .filter(|&(pos, _)| pos.is_ascii_lowercase() && !held_keys.contains(&pos))
        .map(|(pos, cost)| (pos, -cost))
        .collect();
        
    res
}

#[inline(never)]
fn dijkstra(data: ArrayView2<u8>, start_pos: (usize, usize)) -> Vec<(u8, i64)> {
    // Dijkstra over the "raw" map.
    let mut heap = BinaryHeap::new();
    heap.push((0, start_pos));

    let mut costs: AHashMap<(usize, usize), i64> = AHashMap::with_capacity(1024);

    let mut result = vec![];

    while let Some((cost, pos)) = heap.pop() {
        for dir in DIRS {
            let next_pos = (
                pos.0.checked_add_signed(dir.0).unwrap(),
                pos.1.checked_add_signed(dir.1).unwrap(),
            );
            let next_cost = cost - 1;

            let next_tile = *data.get(next_pos).expect("invalid next pos");
            let is_wall = next_tile == b'#';

            if is_wall || next_pos == start_pos {
                continue;
            }

            let next = (next_cost, next_pos);

            if next_cost > *costs.get(&next_pos).unwrap_or(&i64::MIN) {
                if next_tile != b'.' {
                    result.push((next_tile, -next_cost));
                } else {
                    heap.push(next);
                }
                costs.insert(next_pos, next_cost);
            }
        }
    }

    result
}

// Costs to go from a node to it's directly reachable neighbouring nodes.
#[inline(never)]
fn make_cost_map(data: ArrayView2<u8>) -> AHashMap<u8, Vec<(u8, i64)>> {
    data.indexed_iter()
        .filter(|&(_, itm)| itm != &b'.' && itm != &b'#')
        .map(|(idx, itm)| (*itm, dijkstra(data, idx)))
        .collect()
}

#[inline(never)]
fn recursive_best_path<const AGENTS: usize>(
    data: &[&AHashMap<u8, Vec<(u8, i64)>>; AGENTS],
    cost_so_far: i64,
    held_keys: &[u8],
    positions: &[u8; AGENTS],
    cache: &mut PathCache<AGENTS>,
    dijkstra_cache: &mut DijkstraCache,
) -> i64 {
    
    (0..AGENTS)
        .filter_map(|agent| {
            let pos = positions[agent];
            
            let mut possible = false;
            
            for q in b'a'..=b'z' {
                if data[agent].contains_key(&q) && !held_keys.contains(&q) {
                    possible = true;
                    break;
                }
            }
            
            if !possible {
                return None;
            }

            dijkstra_cache
                .entry((held_keys.to_vec(), agent, pos))
                .or_insert_with(|| reachable_keys(data[agent], held_keys, pos))
                .clone()
                .into_iter()
                .map(|(k, cost)| {
                    let mut keys: Vec<u8> = held_keys.iter().chain(Some(&k)).copied().collect();
                    keys.sort_unstable();

                    let base_cost = cost_so_far + cost;
                    
                    let cache_key = (keys.clone(), *positions, k, agent);
                    let cached_additional_cost = cache.get(&cache_key);
                    
                    match cached_additional_cost {
                        None => {
                            let mut new_positions = *positions;
                            new_positions[agent] = k;
                            let result = recursive_best_path::<AGENTS>(
                                data,
                                base_cost,
                                &keys,
                                &new_positions,
                                cache,
                                dijkstra_cache,
                            );
                            let additional_cost = result - base_cost;
                            cache.insert(cache_key, additional_cost);
                            result
                        }
                        Some(&c) => base_cost + c,
                    }
                })
                .min()
        })
        .min()
        .unwrap_or(cost_so_far)
}

fn calculate_p1(data: &Array2<u8>) -> i64 {
    
    recursive_best_path::<1>(
        &[&make_cost_map(data.view())],
        0,
        &[],
        &[b'@'],
        &mut AHashMap::with_capacity(CACHE_SIZE),
        &mut AHashMap::with_capacity(DIJKSTRA_CACHE_SIZE),
    )
}

fn calculate_p2(mut data: Array2<u8>) -> i64 {
    
    let entrance_pos = position_of(&data.view(), b'@');

    data[(entrance_pos.0 - 1, entrance_pos.1 - 1)] = b'@';
    data[(entrance_pos.0 + 1, entrance_pos.1 - 1)] = b'@';
    data[(entrance_pos.0 - 1, entrance_pos.1 + 1)] = b'@';
    data[(entrance_pos.0 + 1, entrance_pos.1 + 1)] = b'@';

    data[(entrance_pos.0, entrance_pos.1)] = b'#';
    data[(entrance_pos.0, entrance_pos.1 - 1)] = b'#';
    data[(entrance_pos.0, entrance_pos.1 + 1)] = b'#';
    data[(entrance_pos.0 + 1, entrance_pos.1)] = b'#';
    data[(entrance_pos.0 - 1, entrance_pos.1)] = b'#';

    let q1 = data.slice(s![0..=entrance_pos.0, 0..=entrance_pos.1]);
    let q2 = data.slice(s![entrance_pos.0.., 0..=entrance_pos.1]);
    let q3 = data.slice(s![0..=entrance_pos.0, entrance_pos.1..]);
    let q4 = data.slice(s![entrance_pos.0.., entrance_pos.1..]);

    recursive_best_path::<4>(
        &[&make_cost_map(q1), &make_cost_map(q2), &make_cost_map(q3), &make_cost_map(q4)],
        0,
        &[],
        &[b'@', b'@', b'@', b'@'],
        &mut AHashMap::with_capacity(CACHE_SIZE),
        &mut AHashMap::with_capacity(DIJKSTRA_CACHE_SIZE),
    )
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_18");

    const P1_EXAMPLE_1: &str = "#########
#b.A.@.a#
#########";

    const P1_EXAMPLE_2: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const P1_EXAMPLE_3: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const P1_EXAMPLE_4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const P1_EXAMPLE_5: &str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    const P2_EXAMPLE_1: &str = "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######";

    const P2_EXAMPLE_3: &str = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

    #[test]
    fn test_p1_example_1() {
        assert_eq!(calculate_p1(&parse(&P1_EXAMPLE_1)), 8);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(&P1_EXAMPLE_2)), 86);
    }

    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(&P1_EXAMPLE_3)), 132);
    }

    #[test]
    fn test_p1_example_4() {
        assert_eq!(calculate_p1(&parse(&P1_EXAMPLE_4)), 136);
    }

    #[test]
    fn test_p1_example_5() {
        assert_eq!(calculate_p1(&parse(&P1_EXAMPLE_5)), 81);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 4406);
    }

    #[test]
    fn test_p2_example_1() {
        assert_eq!(calculate_p2(parse(&P2_EXAMPLE_1)), 8);
    }

    #[test]
    fn test_p2_example_3() {
        assert_eq!(calculate_p2(parse(&P2_EXAMPLE_3)), 72);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(parse(&REAL_DATA)), 1964);
    }
}
