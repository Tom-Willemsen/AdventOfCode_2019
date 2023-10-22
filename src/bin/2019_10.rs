use clap::Parser;
use ndarray::{s, Array2};
use std::fs;
use num::integer::gcd;
use std::f64::consts::PI;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Array2<bool> {
    let vec = raw_inp
        .trim()
        .lines()
        .map(|line| line.trim())
        .flat_map(|line| line.bytes().map(|x| x == b'#'))
        .collect::<Vec<_>>();
        
    let y = raw_inp
        .trim()
        .lines()
        .count();
        
    let x = raw_inp
        .trim()
        .lines()
        .map(|line| line.trim().len())
        .max()
        .expect("at least one line should exist");
        
    Array2::from_shape_vec((y, x), vec).expect("wrong shape")
}

fn is_visible(monitor_y: usize, monitor_x: usize, asteroid_y: usize, asteroid_x: usize, data: &Array2<bool>) -> bool {
    let diff_y = isize::try_from(asteroid_y).unwrap() - isize::try_from(monitor_y).unwrap();
    let diff_x = isize::try_from(asteroid_x).unwrap() - isize::try_from(monitor_x).unwrap();
    
    let multiple = gcd(diff_y.abs(), diff_x.abs());
    
    let dy = diff_y / multiple;
    let dx = diff_x / multiple;
    
    let mut current_x = monitor_x.checked_add_signed(dx).unwrap();
    let mut current_y = monitor_y.checked_add_signed(dy).unwrap();
    let mut visible = true;
    
    while (current_x, current_y) != (asteroid_x, asteroid_y) {
        if data[(current_y, current_x)] {
            visible = false;
            break;
        }
        current_x = current_x.checked_add_signed(dx).unwrap();
        current_y = current_y.checked_add_signed(dy).unwrap();
    }
    
    visible
}

fn count_visible(monitor_y: usize, monitor_x: usize, data: &Array2<bool>) -> usize {
    data.indexed_iter()
        .filter(|&(_, &item)| item)
        .filter(|&((y, x), _)| (y, x) != (monitor_y, monitor_x))
        .filter(|&((asteroid_y, asteroid_x), _)| {
            is_visible(monitor_y, monitor_x, asteroid_y, asteroid_x, data)
        })
        .count()
}

fn calculate_p1(data: &Array2<bool>) -> (usize, usize, usize) {
    data.indexed_iter()
        .filter(|&(_, &item)| item)
        .map(|((y, x), _)| {
            (y, x, count_visible(y, x, data))
        })
        .max_by_key(|x| x.2)
        .expect("at least one asteroid should exist")
}

const P2_ASTEROID: usize = 200;

fn calculate_p2(mut data: Array2<bool>, monitor_y: usize, monitor_x: usize) -> usize {
    let mut destroyed = 0;
    
    loop {
        let mut visible = data.indexed_iter()
            .filter(|&(_, &item)| item)
            .filter(|&((y, x), _)| (y, x) != (monitor_y, monitor_x))
            .filter_map(|((asteroid_y, asteroid_x), _)| {
                if is_visible(monitor_y, monitor_x, asteroid_y, asteroid_x, &data) {
                    Some((asteroid_y, asteroid_x))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
            
        if destroyed + visible.len() < P2_ASTEROID {
            destroyed += visible.len();
            visible.iter().for_each(|&(y, x)| data[(y, x)] = false);
        } else {
            visible.sort_unstable_by(|&(y1, x1), &(y2, x2)| {
                let t1 = (-0.5*PI - (y1 as f64 - monitor_y as f64).atan2(x1 as f64 - monitor_x as f64)).rem_euclid(2.*PI);
                let t2 = (-0.5*PI - (y2 as f64 - monitor_y as f64).atan2(x2 as f64 - monitor_x as f64)).rem_euclid(2.*PI);
                t2.partial_cmp(&t1).expect("should not have NaNs")
            });
            println!("mon y {} mon x {}", monitor_y, monitor_x);
            println!("destroyed: {} len: {}", destroyed, visible.len());
            println!("{:?}", visible);
            visible.iter().for_each(|&(y1, x1)| {
                println!("visible {} {} tan {}", y1, x1, (-0.5*PI - (y1 as f64 - monitor_y as f64).atan2(x1 as f64 - monitor_x as f64)).rem_euclid(2.*PI));
            });
            let coord = visible[P2_ASTEROID - destroyed - 2];
            return 100 * coord.1 + coord.0;
        }
    }
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (monitor_y, monitor_x, p1) = calculate_p1(&data);
    println!("y {} x {}", monitor_y, monitor_x);
    let p2 = calculate_p2(data, monitor_y, monitor_x);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE_DATA_1: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
    
    const EXAMPLE_DATA_2: &str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
    
    const EXAMPLE_DATA_3: &str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
    
    const EXAMPLE_DATA_4: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
    
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_10");
    
    #[test]
    fn test_p1_example_1() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_1)).2, 33);
    }
    
    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_2)).2, 35);
    }
    
    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_3)).2, 41);
    }
    
    #[test]
    fn test_p1_example_4() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_4)).2, 210);
    }
    
    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)).2, 347);
    }
    
    #[test]
    fn test_p2_example() {
        let data = parse(&EXAMPLE_DATA_4);
        let (y, x, _) = calculate_p1(&data);
        assert_eq!(calculate_p2(data, y, x), 802);
    }
    
    #[test]
    fn test_p2_real() {
        let data = parse(&REAL_DATA);
        let (y, x, _) = calculate_p1(&data);
        assert!(calculate_p2(data, y, x) < 1518);
    }
}
 
