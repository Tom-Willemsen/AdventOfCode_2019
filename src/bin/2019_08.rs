use clap::Parser;
use ndarray::{s, ArrayView};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<u8> {
    raw_inp.trim().bytes().map(|x| x - b'0').collect()
}

fn calculate_p1<const X_SIZE: usize, const Y_SIZE: usize>(data: &[u8]) -> usize {
    let z_size = data.len() / (X_SIZE * Y_SIZE);

    let arr = ArrayView::from_shape((z_size, Y_SIZE, X_SIZE), data).unwrap();

    (0..z_size)
        .map(|layer| arr.slice(s![layer, .., ..]))
        .map(|data| {
            (
                data.iter().filter(|&x| *x == 0).count(),
                data.iter().filter(|&x| *x == 1).count(),
                data.iter().filter(|&x| *x == 2).count(),
            )
        })
        .min_by_key(|&x| x.0)
        .map(|x| x.1 * x.2)
        .unwrap()
}

fn calculate_p2<const X_SIZE: usize, const Y_SIZE: usize>(data: &[u8]) -> String {
    let z_size = data.len() / (X_SIZE * Y_SIZE);

    let arr = ArrayView::from_shape((z_size, Y_SIZE, X_SIZE), data).unwrap();

    let mut result = vec![];

    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            let mut z = 0;
            while arr[(z, y, x)] == 2 {
                z += 1;
            }
            let c = if arr[(z, y, x)] == 1 { "█" } else { " " };
            result.push(c);
        }
        result.push("\n");
    }

    result.join("")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1::<25, 6>(&data);
    let p2 = calculate_p2::<25, 6>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_P1: &str = "123456789012";
    const EXAMPLE_DATA_P2: &str = "0222112222120000";
    const REAL_DATA: &str = include_str!("../../inputs/real/2019_08");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1::<3, 2>(&parse(&EXAMPLE_DATA_P1)), 1);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2::<2, 2>(&parse(&EXAMPLE_DATA_P2)), " █\n█ \n");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1::<25, 6>(&parse(&REAL_DATA)), 2413);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(
            calculate_p2::<25, 6>(&parse(&REAL_DATA)).trim(),
            "
███   ██  ███  ████ ███  
█  █ █  █ █  █    █ █  █ 
███  █    █  █   █  ███  
█  █ █    ███   █   █  █ 
█  █ █  █ █    █    █  █ 
███   ██  █    ████ ███  
"
            .trim()
        );
    }
}
