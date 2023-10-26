use ahash::AHashMap;
use clap::Parser;
use num::integer::div_ceil;
use std::cmp::min;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct Reaction<'a> {
    inputs: AHashMap<&'a str, i64>,
    output_type: &'a str,
    output_count: i64,
}

impl<'a> From<&'a str> for Reaction<'a> {
    fn from(s: &'a str) -> Reaction<'a> {
        let (reactants, product) = s.split_once(" => ").expect("failed to split reaction");

        let (product_count, product) = product
            .trim()
            .split_once(' ')
            .expect("failed to split product");
        let product_count: i64 = product_count
            .parse()
            .expect("failed to parse product count");

        let mut inputs: AHashMap<&'a str, i64> = AHashMap::default();

        reactants
            .trim()
            .split(", ")
            .map(|r| r.split_once(' ').expect("failed to split reaction"))
            .for_each(|(count, reactant)| {
                inputs.insert(
                    reactant,
                    count.parse().expect("failed to parse reactant count"),
                );
            });

        Reaction {
            inputs,
            output_type: product,
            output_count: product_count,
        }
    }
}

fn parse(raw_inp: &str) -> Vec<Reaction> {
    raw_inp.trim().lines().map(|line| line.into()).collect()
}

fn recursive_ore_necessary<'a>(
    data: &'a [Reaction],
    stockpile: &mut AHashMap<&'a str, i64>,
    reagent: &'a str,
    count: i64,
) -> i64 {
    if reagent == "ORE" {
        return count;
    }

    let mut count = count;
    let stockpiled = *stockpile.get(reagent).unwrap_or(&0);
    if stockpiled > 0 {
        let from_stockpile = min(count, stockpiled);
        count -= from_stockpile;
        stockpile
            .entry(reagent)
            .and_modify(|e| *e -= from_stockpile);
    }

    if count <= 0 {
        return 0;
    }

    let reaction = data
        .iter()
        .find(|r| r.output_type == reagent)
        .expect("no reaction?");

    let reaction_count = div_ceil(count, reaction.output_count);
    let wastage = (reaction_count * reaction.output_count) - count;

    stockpile
        .entry(reagent)
        .and_modify(|x| *x += wastage)
        .or_insert(wastage);

    reaction
        .inputs
        .iter()
        .map(|(k, &v)| recursive_ore_necessary(data, stockpile, k, reaction_count * v))
        .sum()
}

fn calculate_p1(data: &[Reaction]) -> i64 {
    let mut stockpile = AHashMap::default();
    recursive_ore_necessary(data, &mut stockpile, "FUEL", 1)
}

const P2_ORE: i64 = 1000000000000;

fn calculate_p2(data: &[Reaction], p1_ore_per_fuel: i64) -> i64 {
    let mut stockpile = AHashMap::default();

    let mut lower_bound = P2_ORE / p1_ore_per_fuel;
    let mut upper_bound = P2_ORE;

    loop {
        stockpile.clear();
        let midpoint = (upper_bound + lower_bound) / 2;

        if midpoint == lower_bound || midpoint == upper_bound {
            return lower_bound;
        }

        let result = recursive_ore_necessary(data, &mut stockpile, "FUEL", midpoint);

        if result > P2_ORE {
            upper_bound = midpoint;
        } else {
            lower_bound = midpoint;
        }
    }
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data, p1);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const EXAMPLE_DATA_2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const EXAMPLE_DATA_3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const EXAMPLE_DATA_4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const EXAMPLE_DATA_5: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_14");

    #[test]
    fn test_p1_example_1() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_1)), 31);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_2)), 165);
    }

    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_3)), 13312);
    }

    #[test]
    fn test_p1_example_4() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_4)), 180697);
    }

    #[test]
    fn test_p1_example_5() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_5)), 2210736);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 378929);
    }

    #[test]
    fn test_p2_example_3() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_3), 13312), 82892753);
    }

    #[test]
    fn test_p2_example_4() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_4), 180697), 5586022);
    }

    #[test]
    fn test_p2_example_5() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_5), 2210736), 460664);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA), 378929), 3445249);
    }
}
