// https://adventofcode.com/2025/day/4

use aoc_common::Grid1D;

pub fn part_1(input: &str) -> i32 {
    let grid: Grid1D<bool> = Grid1D::from_str(input, '@');
    grid.iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .filter(|(idx, _)| grid.count_adjacent_1d(*idx, |value| *value) < 4)
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut grid: Grid1D<bool> = Grid1D::from_str(input, '@');
    let start_count = grid.count_by(|v| *v);

    loop {
        let new_values: Vec<_> = grid
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                if grid.get_from_1d_index(idx).is_some_and(|s| *s) {
                    match grid.count_adjacent_1d(idx, |value| *value) {
                        0..4 => false,
                        _ => true,
                    }
                } else {
                    false
                }
            })
            .collect();
        let old_count = grid.count_by(|v| *v);
        grid.values = new_values;
        let new_count = grid.count_by(|v| *v);
        if old_count == new_count {
            return (start_count - old_count) as i32;
        }
    }
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
