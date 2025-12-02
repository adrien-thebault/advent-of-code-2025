use advent_of_code_2025::*;
use fancy_regex::Regex;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day2.txt"))
            .trim()
            .split(',')
            .flat_map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
            })
            .collect_vec();
    }

    let solver = |input: &[usize], regex: Regex| {
        input
            .par_iter()
            .filter(|id| regex.is_match(&id.to_string()).unwrap())
            .sum::<usize>()
    };

    // Part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            solver(&input, Regex::new(r"^(\d+)\1$").unwrap())
        );
    }

    // Part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            solver(&input, Regex::new(r"^(\d+)\1+$").unwrap())
        );
    }
}
