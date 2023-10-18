use clap::Parser;
use std::cmp::{max, min};
use std::fs;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    dist: i32,
    dir: char,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instruction {
            dist: s[1..].parse().or(Err("int parse failed"))?,
            dir: s.chars().next().ok_or("dir parse failed")?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LineSegment {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
    start_steps: i32,
}

impl LineSegment {
    fn intersection_with(&self, other: &LineSegment) -> Option<(i32, i32)> {
        let self_min_x = min(self.start_x, self.end_x);
        let self_max_x = max(self.start_x, self.end_x);
        let other_min_x = min(other.start_x, other.end_x);
        let other_max_x = max(other.start_x, other.end_x);

        let self_min_y = min(self.start_y, self.end_y);
        let self_max_y = max(self.start_y, self.end_y);
        let other_min_y = min(other.start_y, other.end_y);
        let other_max_y = max(other.start_y, other.end_y);

        if self.start_x != self.end_x && other.start_y != other.end_y {
            // self horizontal, other vertical
            let other_x = other_min_x;
            let self_y = self_min_y;

            if other_x >= self_min_x
                && other_x < self_max_x
                && self_y >= other_min_y
                && self_y < other_max_y
            {
                Some((other_x, self_y))
            } else {
                None
            }
        } else if self.start_y != self.end_y && other.start_x != other.end_x {
            // self vertical, other horizontal
            let self_x = self_min_x;
            let other_y = other_min_y;

            if other_y >= self_min_y
                && other_y < self_max_y
                && self_x >= other_min_x
                && self_x < other_max_x
            {
                Some((self_x, other_y))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn parse(raw_inp: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    raw_inp
        .trim()
        .split_once('\n')
        .map(|(a, b)| (parse_line(a), parse_line(b)))
        .expect("invalid input format")
}

fn parse_line(raw_inp: &str) -> Vec<Instruction> {
    raw_inp
        .trim()
        .split(',')
        .map(|s| s.parse().expect("failed parse"))
        .collect()
}

fn instructions_to_line_segments(instructions: &[Instruction]) -> Vec<LineSegment> {
    let mut x = 0;
    let mut y = 0;
    let mut steps: i32 = 0;

    let mut segments = Vec::with_capacity(instructions.len());

    for instruction in instructions {
        let (start_x, start_y) = (x, y);
        if instruction.dir == 'U' {
            y += instruction.dist;
        } else if instruction.dir == 'D' {
            y -= instruction.dist;
        } else if instruction.dir == 'R' {
            x += instruction.dist;
        } else if instruction.dir == 'L' {
            x -= instruction.dist;
        }
        let segment = LineSegment {
            start_x,
            start_y,
            end_x: x,
            end_y: y,
            start_steps: steps,
        };

        segments.push(segment);
        steps += instruction.dist;
    }

    segments
}

fn calculate(line1: &[Instruction], line2: &[Instruction]) -> (i32, i32) {
    let line1_segments = instructions_to_line_segments(line1);
    let line2_segments = instructions_to_line_segments(line2);

    let mut cheapest_steps = i32::MAX;
    let mut cheapest_dist = i32::MAX;

    for segment1 in line1_segments.iter() {
        for segment2 in line2_segments.iter() {
            if let Some(intersect) = segment1.intersection_with(segment2) {
                let (intersect_x, intersect_y) = intersect;

                let steps = segment1.start_steps
                    + segment2.start_steps
                    + (segment1.start_x - intersect_x).abs()
                    + (segment1.start_y - intersect_y).abs()
                    + (segment2.start_x - intersect_x).abs()
                    + (segment2.start_y - intersect_y).abs();

                let dist = intersect_x.abs() + intersect_y.abs();

                if steps != 0 {
                    cheapest_steps = min(cheapest_steps, steps);
                    cheapest_dist = min(cheapest_dist, dist);
                }
            }
        }
    }

    (cheapest_dist, cheapest_steps)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (line1, line2) = parse(&inp);
    let (p1, p2) = calculate(&line1, &line2);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2019_03");
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_03");

    const TEST_DATA_1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    const TEST_DATA_2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_p1_example() {
        let (line1, line2) = parse(&EXAMPLE_DATA);
        assert_eq!(calculate(&line1, &line2).0, 6);
    }

    #[test]
    fn test_p1_test_data_1() {
        let (line1, line2) = parse(&TEST_DATA_1);
        assert_eq!(calculate(&line1, &line2).0, 159);
    }

    #[test]
    fn test_p1_test_data_2() {
        let (line1, line2) = parse(&TEST_DATA_2);
        assert_eq!(calculate(&line1, &line2).0, 135);
    }

    #[test]
    fn test_p1_real() {
        let (line1, line2) = parse(&REAL_DATA);
        assert_eq!(calculate(&line1, &line2).0, 8015);
    }

    #[test]
    fn test_p2_example() {
        let (line1, line2) = parse(&EXAMPLE_DATA);
        assert_eq!(calculate(&line1, &line2).1, 30);
    }

    #[test]
    fn test_p2_test_data_1() {
        let (line1, line2) = parse(&TEST_DATA_1);
        assert_eq!(calculate(&line1, &line2).1, 610);
    }

    #[test]
    fn test_p2_test_data_2() {
        let (line1, line2) = parse(&TEST_DATA_2);
        assert_eq!(calculate(&line1, &line2).1, 410);
    }

    #[test]
    fn test_p2_real() {
        let (line1, line2) = parse(&REAL_DATA);
        assert_eq!(calculate(&line1, &line2).1, 163676);
    }
}
