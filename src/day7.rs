use advent_of_code_2025::*;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Simulation {
    pub height: usize,
    pub start_x: usize,
    pub splitters: Vec<Vec<bool>>,
    pub cache: HashMap<(usize, usize), usize>,
}

impl Simulation {
    pub fn count_splits(&self) -> usize {
        (0..self.height)
            .fold((vec![self.start_x], 0), |(beams, mut splits), line| {
                (
                    beams
                        .into_iter()
                        .flat_map(|x| {
                            if self.splitters[line][x] {
                                splits += 1;
                                vec![x - 1, x + 1]
                            } else {
                                vec![x]
                            }
                        })
                        .sorted()
                        .dedup()
                        .collect_vec(),
                    splits,
                )
            })
            .1
    }

    pub fn compute_timelines(&mut self, x: usize, y: usize) -> usize {
        match self.cache.get(&(x, y)) {
            Some(cached) => *cached,
            None => {
                if y == self.height {
                    1
                } else {
                    let result = if self.splitters[y][x] {
                        self.compute_timelines(x - 1, y + 1) + self.compute_timelines(x + 1, y + 1)
                    } else {
                        self.compute_timelines(x, y + 1)
                    };

                    self.cache.insert((x, y), result);
                    result
                }
            }
        }
    }
}

fn main() {
    timer!("total");
    let mut simulation;

    {
        timer!("prepare");

        let file = String::from_utf8_lossy(include_bytes!("../inputs/day7.txt"));
        let lines = file.trim().lines().collect_vec();

        simulation = Simulation {
            height: lines.len(),
            start_x: lines
                .first()
                .and_then(|l| l.chars().position(|c| c == 'S'))
                .unwrap(),
            splitters: lines
                .into_iter()
                .map(|l| l.chars().map(|c| c == '^').collect_vec())
                .collect_vec(),
            cache: HashMap::new(),
        }
    }

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", simulation.count_splits());
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            simulation.compute_timelines(simulation.start_x, 0)
        );
    }
}
