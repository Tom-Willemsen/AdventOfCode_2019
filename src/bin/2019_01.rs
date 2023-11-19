use advent_of_code_2019::{Cli, Parser};
use std::cmp::max;
use std::fs;

fn parse(raw_inp: &str) -> Vec<i64> {
    raw_inp.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn fuel_for(mass: &i64) -> i64 {
    max(mass / 3 - 2, 0)
}

fn recursive_fuel_for(mass: &i64) -> i64 {
    let mut additional_fuel = fuel_for(mass);
    let mut total_fuel = additional_fuel;

    while additional_fuel > 0 {
        additional_fuel = fuel_for(&additional_fuel);
        total_fuel += additional_fuel;
    }
    total_fuel
}

fn calculate_p1(nums: &[i64]) -> i64 {
    nums.iter().map(fuel_for).sum()
}

fn calculate_p2(nums: &[i64]) -> i64 {
    nums.iter().map(recursive_fuel_for).sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums: Vec<i64> = parse(&inp);
    let p1 = calculate_p1(&nums);
    let p2 = calculate_p2(&nums);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2019_01");
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_01");

    #[test]
    fn test_fuel_for() {
        assert_eq!(fuel_for(&12), 2);
        assert_eq!(fuel_for(&14), 2);
        assert_eq!(fuel_for(&1969), 654);
        assert_eq!(fuel_for(&100756), 33583);
    }

    #[test]
    fn test_recursive_fuel_for() {
        assert_eq!(recursive_fuel_for(&14), 2);
        assert_eq!(recursive_fuel_for(&1969), 966);
        assert_eq!(recursive_fuel_for(&100756), 50346);
    }

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA)), 33583 + 654 + 2 + 2);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA)), 50346 + 966 + 2 + 2);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 3232358);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 4845669);
    }
}
