use advent_of_code_2019::{Cli, Parser};
use num::Integer;
use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Body {
    pos_x: i64,
    pos_y: i64,
    pos_z: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
}

impl Body {
    fn potential_energy(&self) -> i64 {
        self.pos_x.abs() + self.pos_y.abs() + self.pos_z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn pos_mut<const AXIS: char>(&mut self) -> &mut i64 {
        if AXIS == 'x' {
            &mut self.pos_x
        } else if AXIS == 'y' {
            &mut self.pos_y
        } else {
            &mut self.pos_z
        }
    }

    fn vel_mut<const AXIS: char>(&mut self) -> &mut i64 {
        if AXIS == 'x' {
            &mut self.vel_x
        } else if AXIS == 'y' {
            &mut self.vel_y
        } else {
            &mut self.vel_z
        }
    }

    fn pos<const AXIS: char>(&self) -> i64 {
        if AXIS == 'x' {
            self.pos_x
        } else if AXIS == 'y' {
            self.pos_y
        } else {
            self.pos_z
        }
    }

    fn vel<const AXIS: char>(&self) -> i64 {
        if AXIS == 'x' {
            self.vel_x
        } else if AXIS == 'y' {
            self.vel_y
        } else {
            self.vel_z
        }
    }
}

impl FromStr for Body {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .strip_prefix('<')
            .and_then(|s| s.strip_suffix('>'))
            .map(|s| {
                s.split(',')
                    .map(|itm| itm.trim())
                    .map(|itm| &itm[2..])
                    .filter_map(|itm| itm.parse().ok())
                    .collect::<Vec<i64>>()
            })
            .ok_or("failed to parse")?;

        Ok(Body {
            pos_x: *coords.first().ok_or("coord 0 missing")?,
            pos_y: *coords.get(1).ok_or("coord 1 missing")?,
            pos_z: *coords.get(2).ok_or("coord 2 missing")?,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        })
    }
}

fn apply_step_one_axis<const AXIS: char>(bodies: &mut [Body]) {
    for idx in 0..bodies.len() {
        for other_idx in idx + 1..bodies.len() {
            let adj = match bodies[idx]
                .pos::<AXIS>()
                .cmp(&bodies[other_idx].pos::<AXIS>())
            {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };

            *bodies[idx].vel_mut::<AXIS>() += adj;
            *bodies[other_idx].vel_mut::<AXIS>() -= adj;
        }
    }

    bodies
        .iter_mut()
        .for_each(|b| *b.pos_mut::<AXIS>() += b.vel::<AXIS>());
}

fn parse(raw_inp: &str) -> Vec<Body> {
    raw_inp
        .trim()
        .lines()
        .map(|line| line.parse().expect("parse fail"))
        .collect()
}

fn calculate_p1(data: &[Body]) -> i64 {
    let mut bodies = data.to_vec();
    for _ in 0..1000 {
        apply_step_one_axis::<'x'>(&mut bodies);
        apply_step_one_axis::<'y'>(&mut bodies);
        apply_step_one_axis::<'z'>(&mut bodies);
    }

    bodies.iter().map(|b| b.total_energy()).sum()
}

fn period_of<const AXIS: char>(data: &[Body]) -> usize {
    let mut bodies = data.to_vec();
    let initial_state: Vec<_> = bodies.clone();

    let mut pos_ctr = 1;
    apply_step_one_axis::<AXIS>(&mut bodies);

    while bodies != initial_state {
        apply_step_one_axis::<AXIS>(&mut bodies);
        pos_ctr += 1;
    }

    pos_ctr
}

fn calculate_p2(data: &[Body]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    rayon::scope(|s| {
        s.spawn(|_| x = period_of::<'x'>(data));
        s.spawn(|_| y = period_of::<'y'>(data));
        s.spawn(|_| z = period_of::<'z'>(data));
    });

    x.lcm(&y).lcm(&z)
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

    const EXAMPLE_DATA_1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

    const EXAMPLE_DATA_2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";

    const REAL_DATA: &str = include_str!("../../inputs/real/2019_12");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 6849);
    }

    #[test]
    fn test_p2_example_1() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_1)), 2772);
    }

    #[test]
    fn test_p2_example_2() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_2)), 4686774924);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 356658899375688);
    }
}
