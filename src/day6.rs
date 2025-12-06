use advent_of_code_2025::*;
use itertools::Itertools;
use std::ops::{Add, Mul};

fn transpose<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::with_capacity(input.len()); input[0].len()];

    input.into_iter().for_each(|row| {
        row.into_iter()
            .enumerate()
            .for_each(|(j, x)| transposed[j].push(x))
    });

    transposed
}

fn main() {
    timer!("total");

    let columns;
    let (mut signs, mut sizes) = (vec![], vec![]);

    {
        timer!("prepare");

        let file = String::from_utf8_lossy(include_bytes!("../inputs/day6.txt"));
        let mut lines = file.lines().collect_vec();

        lines.pop().unwrap().chars().for_each(|c| match c {
            '+' | '*' => {
                sizes.push(1);
                signs.push(c);
            }
            ' ' => {
                if let Some(n) = sizes.last_mut() {
                    *n += 1;
                }
            }
            _ => unreachable!(),
        });

        columns = transpose(
            lines
                .iter()
                .map(|l| {
                    let mut i = 0;
                    sizes
                        .iter()
                        .map(|col_size| {
                            i += col_size;
                            l[i - col_size..i].chars().collect_vec()
                        })
                        .collect_vec()
                })
                .collect_vec(),
        );
    }

    let solver = |columns: &[Vec<Vec<char>>]| -> usize {
        columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                col.iter()
                    .map(|v| v.iter().join("").trim().to_string())
                    .filter_map(|v| v.parse::<usize>().ok())
                    .fold(
                        match signs[i] {
                            '+' => 0,
                            '*' => 1,
                            _ => unreachable!(),
                        },
                        match signs[i] {
                            '+' => Add::add,
                            '*' => Mul::mul,
                            _ => unreachable!(),
                        },
                    )
            })
            .sum::<usize>()
    };

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", solver(&columns));
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            solver(&columns.into_iter().map(transpose).collect_vec())
        )
    }
}
