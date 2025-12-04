use advent_of_code_2025::*;
use itertools::Itertools;

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day3.txt"))
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|x| x as u64)
                    .collect_vec()
            })
            .collect_vec();
    }

    let solver = |input: &[Vec<u64>], nb| {
        input
            .iter()
            .map(|bank| {
                let mut last_pos = 0;
                (1..=nb)
                    .rev()
                    .filter_map(|n| {
                        let slice = &bank[last_pos..=bank.len() - n];
                        let max = slice.iter().max()?;
                        last_pos = last_pos + slice.iter().position(|x| x == max)? + 1;

                        Some(max * 10u64.pow((n - 1) as u32))
                    })
                    .sum::<u64>()
            })
            .sum::<u64>()
    };

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", solver(&input, 2));
    }

    // part 2
    {
        timer!("part 2");
        println!("part 2 : {}", solver(&input, 12));
    }
}
