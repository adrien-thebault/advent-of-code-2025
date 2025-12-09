use advent_of_code_2025::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::iter;

fn main() {
    timer!("total");
    let (red_tiles, combinations);

    {
        timer!("prepare");

        rayon::ThreadPoolBuilder::new().build_global().unwrap();

        red_tiles = String::from_utf8_lossy(include_bytes!("../inputs/day9.txt"))
            .trim()
            .lines()
            .filter_map(|line| {
                line.split(',')
                    .filter_map(|c| c.parse::<isize>().ok())
                    .next_tuple()
            })
            .collect_vec();

        combinations = red_tiles
            .iter()
            .tuple_combinations()
            .map(|((x1, y1), (x2, y2))| {
                let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

                (
                    (min_x, min_y),
                    (max_x, max_y),
                    (min_x.abs_diff(*max_x) + 1) * (min_y.abs_diff(*max_y) + 1),
                )
            })
            .collect_vec();
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {:?}",
            combinations.iter().map(|&(_, _, d)| d).max()
        );
    }

    // part 2
    {
        timer!("part 2");
        println!("part 2 : {:?}", {
            combinations
                .par_iter()
                .filter_map(|((x1, y1), (x2, y2), d)| {
                    red_tiles
                        .iter()
                        .chain(iter::once(&red_tiles[0]))
                        .tuple_windows()
                        .all(|((ex1, ey1), (ex2, ey2))| {
                            ey1 <= y1 && ey2 <= y1
                                || ey1 >= y2 && ey2 >= y2
                                || ex1 <= x1 && ex2 <= x1
                                || ex1 >= x2 && ex2 >= x2
                        })
                        .then_some(d)
                })
                .max()
        });
    }
}
