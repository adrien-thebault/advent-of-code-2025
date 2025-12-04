use advent_of_code_2025::*;
use itertools::Itertools;

const ADJACENT: [(isize, isize); 8] = [
    (0, -1),  // ↑
    (0, 1),   // ↓
    (-1, 0),  // ←
    (1, 0),   // →
    (-1, -1), // ↑ + ←
    (-1, 1),  // ↑ + →
    (1, -1),  // ↓ + ←
    (1, 1),   // ↓ + →
];

fn solver(input: &mut [Vec<bool>], height: isize, width: isize, remove: bool) -> usize {
    let removable = (0..height)
        .cartesian_product(0..width)
        .filter(|&(y, x)| {
            input[y as usize][x as usize]
                && ADJACENT
                    .iter()
                    .map(|&(dy, dx)| (y + dy, x + dx))
                    .filter(|&(ny, nx)| {
                        ny >= 0
                            && ny < height
                            && nx >= 0
                            && nx < width
                            && input[ny as usize][nx as usize]
                    })
                    .count()
                    < 4
        })
        .collect_vec();

    let count = removable.len();
    if remove && count > 0 {
        removable.into_iter().for_each(|(y, x)| {
            input[y as usize][x as usize] = false;
        });
        count + solver(input, height, width, remove)
    } else {
        count
    }
}

fn main() {
    timer!("total");
    let mut input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day4.txt"))
            .trim()
            .lines()
            .map(|l| l.chars().map(|x| x == '@').collect_vec())
            .collect_vec();
    }

    let (height, width) = (input.len() as isize, input[0].len() as isize);

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", solver(&mut input, height, width, false));
    }

    // part 2
    {
        timer!("part 2");
        println!("part 2 : {}", solver(&mut input, height, width, true));
    }
}
