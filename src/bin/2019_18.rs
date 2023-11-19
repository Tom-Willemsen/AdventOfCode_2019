use advent_of_code_2019::{Cli, Parser};
use ahash::AHashMap;
use ndarray::{s, Array2, ArrayView2};
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::fs;
use std::sync::{Arc, RwLock};

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

fn position_of(data: &ArrayView2<u8>, ch: u8) -> (usize, usize) {
    data.indexed_iter()
        .filter(|(_, &itm)| itm == ch)
        .map(|(pos, _)| pos)
        .next()
        .expect("can't find pos")
}

fn make_pos_map(data: &ArrayView2<u8>) -> AHashMap<u8, (usize, usize)> {
    data.indexed_iter()
        .filter(|(_, &itm)| itm != b'.' && itm != b'#')
        .map(|(pos, itm)| (*itm, pos))
        .collect()
}

/// Dijkstra
fn reachable_keys(
    data: &ArrayView2<u8>,
    pos_map: &AHashMap<u8, (usize, usize)>,
    held_keys: &[u8],
    start_at: u8,
) -> Vec<(u8, i64)> {
    let mut heap = BinaryHeap::new();
    let start_pos = *pos_map.get(&start_at).expect("invalid start pos");
    heap.push((0, start_pos));

    let mut costs = Array2::from_elem(data.dim(), i64::MIN);

    let mut result = Vec::with_capacity(26);

    while let Some((cost, pos)) = heap.pop() {
        for dir in DIRS {
            let next_pos = (
                pos.0.checked_add_signed(dir.0).unwrap(),
                pos.1.checked_add_signed(dir.1).unwrap(),
            );
            let next_cost = cost - 1;

            let next_tile = data[next_pos];
            let is_wall = next_tile == b'#';

            if is_wall {
                continue;
            }

            let is_closed_door = next_tile.is_ascii_uppercase()
                && !held_keys.contains(&next_tile.to_ascii_lowercase());

            if is_closed_door {
                continue;
            }

            let next = (next_cost, next_pos);

            if next_cost > costs[next_pos] {
                let is_unheld_key =
                    next_tile.is_ascii_lowercase() && !held_keys.contains(&next_tile);
                if is_unheld_key {
                    result.push((next_tile, -next_cost));
                } else {
                    heap.push(next);
                }
                costs[next_pos] = next_cost;
            }
        }
    }

    result
}

type PathCache<const AGENTS: usize> = AHashMap<(Vec<u8>, [u8; AGENTS], u8, usize), i64>;
type DijkstraCache = AHashMap<(Vec<u8>, usize, u8), Vec<(u8, i64)>>;

fn recursive_best_path<const AGENTS: usize>(
    data: &[&ArrayView2<u8>; AGENTS],
    pos_maps: &[&AHashMap<u8, (usize, usize)>; AGENTS],
    cost_so_far: i64,
    held_keys: &[u8],
    positions: &[u8; AGENTS],
    cache: &Arc<RwLock<PathCache<AGENTS>>>,
    dijkstra_cache: &Arc<RwLock<DijkstraCache>>,
) -> i64 {
    (0..AGENTS)
        .filter_map(|agent| {
            let pos = positions[agent];

            let dijkstra_cache_key = (held_keys.to_vec(), agent, pos);

            let paths = {
                let dc = dijkstra_cache.read().expect("can't lock for read");
                dc.get(&dijkstra_cache_key).cloned()
            };

            let paths = match paths {
                Some(p) => p,
                None => {
                    let result = reachable_keys(data[agent], pos_maps[agent], held_keys, pos);
                    let mut dc = dijkstra_cache.write().expect("can't lock for write");
                    dc.insert(dijkstra_cache_key, result.clone());
                    result
                }
            };

            paths
                .into_par_iter()
                .map(|(k, cost)| {
                    let mut keys: Vec<u8> = held_keys.iter().chain(Some(&k)).copied().collect();
                    keys.sort_unstable();

                    let base_cost = cost_so_far + cost;

                    let cache_key = (held_keys.to_vec(), *positions, k, agent);

                    let cached_additional_cost = {
                        let c = cache.read().expect("can't lock for read");
                        c.get(&cache_key).copied()
                    };

                    match cached_additional_cost {
                        None => {
                            let mut new_positions = *positions;
                            new_positions[agent] = k;
                            let result = recursive_best_path::<AGENTS>(
                                data,
                                pos_maps,
                                base_cost,
                                &keys,
                                &new_positions,
                                cache,
                                dijkstra_cache,
                            );
                            let additional_cost = result - base_cost;
                            {
                                let mut c = cache.write().expect("can't lock for write");
                                c.insert(cache_key, additional_cost);
                            }
                            result
                        }
                        Some(c) => base_cost + c,
                    }
                })
                .min()
        })
        .min()
        .unwrap_or(cost_so_far)
}

const CACHE_SIZE: usize = 32768;
const DIJKSTRA_CACHE_SIZE: usize = 8092;

fn calculate_p1(data: &Array2<u8>) -> i64 {
    recursive_best_path::<1>(
        &[&data.view()],
        &[&make_pos_map(&data.view())],
        0,
        &[],
        &[b'@'],
        &Arc::new(RwLock::new(AHashMap::with_capacity(CACHE_SIZE))),
        &Arc::new(RwLock::new(AHashMap::with_capacity(DIJKSTRA_CACHE_SIZE))),
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
        &[&q1, &q2, &q3, &q4],
        &[
            &make_pos_map(&q1),
            &make_pos_map(&q2),
            &make_pos_map(&q3),
            &make_pos_map(&q4),
        ],
        0,
        &[],
        &[b'@', b'@', b'@', b'@'],
        &Arc::new(RwLock::new(AHashMap::with_capacity(CACHE_SIZE))),
        &Arc::new(RwLock::new(AHashMap::with_capacity(DIJKSTRA_CACHE_SIZE))),
    )
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2(data.clone()));
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
