use advent_of_code_2019::intcode::IntCodeState;
use clap::Parser;
use std::collections::VecDeque;
use std::fs;
use itertools::Itertools;
use ahash::AHashSet;

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

// Items that will terminate the game if we take them
const BAD_ITEMS: [&str; 5] = [
    "molten lava",  // too hot
    "giant electromagnet",  // softlock
    "photons",  // eaten by a grue
    "infinite loop",  // infinite loop
    "escape pod",  // ejected
];

fn parse_location(out: &str) -> Option<&str> {
    out.lines()
        .filter_map(|line| line.strip_prefix("== "))
        .filter_map(|line| line.strip_suffix(" =="))
        .next()
}

fn is_dir(s: &str) -> bool {
    s == "east" || s == "south" || s == "west" || s == "north"
}

fn parse_collectable_items(out: &str) -> Vec<&str> {
    out.lines()
        .filter_map(|line| line.strip_prefix("- "))
        .filter(|&line| !is_dir(line))
        .filter(|&itm| !BAD_ITEMS.contains(&itm))
        .collect()
}

fn parse_allowed_dirs(out: &str) -> Vec<&str> {
    out.lines()
        .filter_map(|line| line.strip_prefix("- "))
        .filter(|&line| is_dir(line))
        .collect()
}

#[derive(Eq, PartialEq)]
enum Phase {
    Exploring,
    PastSensor,
}

const DIRS: [&str; 4] = ["north", "east", "south", "west"];

fn calculate(software: &[i64]) -> String {
    let mut prog: IntCodeState = software.into();
    let mut inp_buffer = VecDeque::new();
    
    let mut phase = Phase::Exploring;
    let mut current_dir: usize = 0;
    let mut inventory = AHashSet::default();
    let mut checkpoint_count = 0;
    
    prog.execute_until_halt(|state| {
        
        if let Some(c) = inp_buffer.pop_front() {
            return Some(c);
        }
        
        let mut out_vec = vec![];
        
        while let Some(c) = state.out_buffer.pop_front() {
            out_vec.push((c as u8) as char);
        }
        
        let out_str = out_vec.iter().join("");
        
        let location = parse_location(&out_str);
        let allowed_dirs = parse_allowed_dirs(&out_str);
        let items = parse_collectable_items(&out_str);
        
        if location == Some("Security Checkpoint") {
            checkpoint_count += 1;
        }
        
        if checkpoint_count >= 3 {
            phase = Phase::PastSensor;
        }
        
        if phase == Phase::Exploring {
            // Overly simple algorithm - just follow left wall to visit all rooms.
            items.iter().for_each(|item| {
                    "take ".bytes()
                        .chain(item.bytes())
                        .chain(Some(b'\n'))
                        .for_each(|b| inp_buffer.push_back(b as i64));
                        
                    inventory.insert(item.to_string());
                });
            
            for dir_diff in 1..=4 {
                let next_dir = (current_dir + dir_diff + 2).rem_euclid(4);
                if allowed_dirs.contains(&DIRS[next_dir]) {
                    DIRS[next_dir]
                        .bytes()
                        .chain(Some(b'\n'))
                        .for_each(|b| inp_buffer.push_back(b as i64));
                    current_dir = next_dir;
                    break;
                }
            }
        } else {
            
            // Santa is not in the direction we just came from
            let santa_direction = allowed_dirs
                .iter()
                .find(|&&dir| dir != DIRS[(current_dir + 2).rem_euclid(4)])
                .expect("invalid santa direction");

            // Ignore any feedback from the door system, 
            // just bruteforce the powerset of combinations
            for s in inventory.iter().powerset() {
                inventory.iter()
                    .filter(|item| !s.contains(item))
                    .for_each(|item| {
                        "drop ".bytes()
                            .chain(item.bytes())
                            .chain(Some(b'\n'))
                            .for_each(|b| inp_buffer.push_back(b as i64));
                    });
                        
                s.iter().for_each(|item| {
                        "take ".bytes()
                            .chain(item.bytes())
                            .chain(Some(b'\n'))
                            .for_each(|b| inp_buffer.push_back(b as i64));
                    });
                        
                santa_direction.bytes()
                    .chain(Some(b'\n'))
                    .for_each(|b| inp_buffer.push_back(b as i64));
            }
        }
        
        inp_buffer.pop_front()
    });
    
    prog.out_buffer.iter()
        .map(|&c| (c as u8) as char)
        .join("")
        .lines()
        .map(|line| line.trim())
        .filter_map(|line| line.strip_prefix("\"Oh, hello! You should be able to get in by typing "))
        .filter_map(|line| line.strip_suffix(" on the keypad at the main airlock.\""))
        .next()
        .expect("couldn't force door open")
        .to_string()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate(&data);
    println!("{}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_25");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)), "34095120");
    }
}
