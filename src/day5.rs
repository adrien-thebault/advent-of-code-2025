use advent_of_code_2025::*;
use std::{collections::HashSet, ops::RangeInclusive};

struct RangeUnion(Vec<RangeInclusive<usize>>);

impl RangeUnion {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, mut range: RangeInclusive<usize>) {
        self.0.retain(|r| {
            if r.start() <= range.end() && range.start() <= r.end() {
                range = *range.start().min(r.start())..=*range.end().max(r.end());
                false
            } else {
                true
            }
        });

        self.0.push(range);
    }

    pub fn contains(&self, value: &usize) -> bool {
        self.0.iter().any(|r| r.contains(value))
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

fn main() {
    timer!("total");
    let mut input = (RangeUnion::new(), HashSet::new());

    {
        timer!("prepare");
        let mut fresh = true;
        String::from_utf8_lossy(include_bytes!("../inputs/day5.txt"))
            .trim()
            .lines()
            .for_each(|l| match l {
                "" => fresh = false,
                l if fresh => {
                    let (start, end) = l.split_once('-').unwrap();
                    input
                        .0
                        .insert(start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap());
                }
                l => {
                    input.1.insert(l.parse::<usize>().unwrap());
                }
            });
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            input.1.iter().filter(|&i| input.0.contains(i)).count()
        );
    }

    // part 2
    {
        timer!("part 2");
        println!("part 2 : {}", input.0.len());
    }
}
