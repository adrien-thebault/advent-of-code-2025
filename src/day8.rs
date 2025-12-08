use advent_of_code_2025::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Box {
    x: isize,
    y: isize,
    z: isize,
}

impl Box {
    fn dist(&self, Box { x, y, z }: &Box) -> f64 {
        (((self.x - x).pow(2) + (self.y - y).pow(2) + (self.z - z).pow(2)) as f64).sqrt()
    }
}

fn main() {
    timer!("total");
    let (boxes, mut combinations);

    {
        timer!("prepare");

        boxes = String::from_utf8_lossy(include_bytes!("../inputs/day8.txt"))
            .trim()
            .lines()
            .filter_map(|line| {
                line.split(',')
                    .filter_map(|c| c.parse().ok())
                    .next_tuple()
                    .map(|(x, y, z)| Box { x, y, z })
            })
            .collect_vec();

        combinations = boxes
            .iter()
            .tuple_combinations()
            .map(|(b1, b2)| (b1, b2, b1.dist(b2)))
            .collect::<Vec<_>>()
            .into_iter()
            .sorted_by(|(_, _, d1), (_, _, d2)| d2.partial_cmp(d1).unwrap())
            .collect_vec();
    }

    let solver =
        |combinations: &mut Vec<(_, _, _)>, map: &mut HashMap<_, _>, sizes: &mut Vec<_>| {
            combinations.pop().map(|(b1, b2, _)| {
                match (map.get(b1).copied(), map.get(b2).copied()) {
                    (Some(c1), Some(c2)) if c1 != c2 => {
                        sizes[c1] += sizes[c2];
                        sizes[c2] = 0;
                        map.iter_mut()
                            .filter(|(_, c)| **c == c2)
                            .for_each(|(_, c)| *c = c1);
                    }
                    (None, Some(circuit)) | (Some(circuit), None) => {
                        map.insert(b1, circuit);
                        map.insert(b2, circuit);
                        sizes[circuit] += 1;
                    }
                    (None, None) => {
                        let circuit_id = sizes.len();
                        map.insert(b1, circuit_id);
                        map.insert(b2, circuit_id);
                        sizes.push(2);
                    }
                    _ => {}
                }

                (b1, b2)
            })
        };

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", {
            let (mut sizes, mut map, mut combinations) =
                (vec![], HashMap::new(), combinations.clone());

            (0..1000).for_each(|_| {
                solver(&mut combinations, &mut map, &mut sizes);
            });

            sizes.sort();
            sizes[sizes.len() - 3..].iter().product::<usize>()
        });
    }

    // part 2
    {
        timer!("part 2");
        println!("part 2 : {:?}", {
            let (mut sizes, mut map, mut last) = (vec![], HashMap::new(), None);

            while map.len() < boxes.len() {
                last = solver(&mut combinations, &mut map, &mut sizes);
            }

            last.map(|(Box { x: x1, .. }, Box { x: x2, .. })| x1 * x2)
        });
    }
}
