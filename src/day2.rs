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

    // regex-based solution, it works but its slow
    #[allow(unused)]
    let regex_solver = |input: &[usize], regex: Regex| {
        input
            .par_iter()
            .filter(|id| regex.is_match(&id.to_string()).unwrap())
            .sum::<usize>()
    };

    // not regex-based solution, way faster
    let solver = |input: &[usize], part: usize| {
        input
            .par_iter()
            .filter(|id| {
                let id_str = id.to_string();
                let (len, bytes) = (id_str.len(), id_str.as_bytes());

                match part {
                    1 if (len % 2 == 0) => (len / 2)..(len / 2 + 1),
                    2 => 1..len,
                    _ => 0..0,
                }
                .any(|pattern_size| {
                    pattern_size > 0
                        && len % pattern_size == 0
                        && bytes.chunks(pattern_size).all_equal()
                })
            })
            .sum::<usize>()
    };

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            // regex_solver(&input, Regex::new(r"^(\d+)\1$").unwrap())
            solver(&input, 1)
        );
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            // regex_solver(&input, Regex::new(r"^(\d+)\1+$").unwrap())
            solver(&input, 2)
        );
    }
}
