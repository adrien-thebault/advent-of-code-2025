use advent_of_code_2025::*;
use itertools::Itertools;

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day1.txt"))
            .trim()
            .lines()
            .map(|line| {
                (if line.starts_with('R') { 1 } else { -1 }) * line[1..].parse::<i32>().unwrap()
            })
            .collect_vec();
    }

    let solver = |input: &[i32]| {
        input
            .iter()
            .fold((0, 50), |(n, acc), x| match (acc + x).rem_euclid(100) {
                0 => (n + 1, 0),
                x => (n, x),
            })
            .0
    };

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", solver(&input));
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            solver(
                &input
                    .iter()
                    .flat_map(|n| itertools::repeat_n(n.signum(), n.unsigned_abs() as usize))
                    .collect_vec()
            )
        );
    }
}
