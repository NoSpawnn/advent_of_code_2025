// https://adventofcode.com/2025/day/7

use std::collections::HashSet;

type Answer = usize;
type Coord = (usize, usize);

fn find_splitters(input: &[Vec<char>]) -> HashSet<Coord> {
    let row_bound = input.len();
    let col_bound = input[0].len();
    let mut splitters = HashSet::new();

    for row in 0..row_bound {
        for col in 0..col_bound {
            if input[row][col] == '^' {
                splitters.insert((row, col));
            }
        }
    }

    splitters
}

fn check_splitters_below(
    splitters: &HashSet<Coord>,
    beams: HashSet<Coord>,
    max_depth: usize,
    depth: usize,
) -> Answer {
    let mut next_beams = HashSet::new();
    let mut split = 0;

    for beam in beams {
        let (row, col) = beam;
        if splitters.contains(&(row + 1, col)) {
            next_beams.insert((row + 1, col - 1));
            next_beams.insert((row + 1, col + 1));
            split += 1;
        } else {
            next_beams.insert((row + 1, col));
        }
    }

    if depth > max_depth {
        return split;
    }

    split + check_splitters_below(splitters, next_beams, max_depth, depth + 1)
}

pub fn part_1(input: &str) -> Answer {
    let lines: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let splitters = find_splitters(&lines);
    let start = (1, lines[0].iter().position(|c| *c == 'S').unwrap());
    check_splitters_below(&splitters, HashSet::from([start]), lines.len(), 0)
}

pub fn part_2(input: &str) -> Answer {
    todo!("day 7 part 2")
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}
