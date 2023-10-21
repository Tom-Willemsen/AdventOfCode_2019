use ahash::AHashMap;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<(&str, &str)> {
    raw_inp
        .trim()
        .lines()
        .map(|line| line.split_once(')').expect("invalid format"))
        .collect()
}

fn calculate_p1(data: &[(&str, &str)]) -> usize {
    let mut orbits: AHashMap<&str, usize> = AHashMap::with_capacity(data.len() + 1);

    let mut dependents: AHashMap<&str, Vec<&str>> = AHashMap::with_capacity(data.len() + 1);

    data.iter().for_each(|(orbited, body)| {
        dependents
            .entry(orbited)
            .and_modify(|v| v.push(body))
            .or_insert(vec![body]);
    });

    let mut next = vec!["COM"];
    let mut depth = 0;

    while orbits.len() < data.len() + 1 {
        let current = next.clone();
        next.clear();
        for item in current {
            orbits.insert(item, depth);
            if let Some(deps) = dependents.get(item) {
                next.extend(deps);
            }
        }
        depth += 1;
    }

    orbits.values().sum()
}

fn calculate_p2(data: &[(&str, &str)]) -> usize {
    let mut ancestors: AHashMap<&str, &str> = AHashMap::with_capacity(data.len() + 1);
    data.iter().for_each(|&(k, v)| {
        ancestors.insert(v, k);
    });

    let mut san_ancestors: AHashMap<&str, usize> = AHashMap::with_capacity(data.len() + 1);
    let mut depth = 0;
    let mut ancestor = ancestors.get("SAN").expect("santa doesn't exist :(");
    while ancestor != &"COM" {
        san_ancestors.insert(ancestor, depth);
        ancestor = ancestors.get(ancestor).unwrap();
        depth += 1;
    }

    let mut depth = 0;
    let mut ancestor = ancestors.get("YOU").unwrap();
    while ancestor != &"COM" {
        if let Some(san_depth) = san_ancestors.get(ancestor) {
            return depth + san_depth;
        }
        ancestor = ancestors.get(ancestor).unwrap();
        depth += 1;
    }

    panic!("no solution found");
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

    const EXAMPLE_DATA_P1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    const EXAMPLE_DATA_P2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_06");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_P1)), 42);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_P2)), 4);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 312697);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 466);
    }
}
