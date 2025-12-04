use std::collections::HashSet;

const ROLL: char = '@';
#[rustfmt::skip]
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1)
];

fn count_adjacent(grid: &HashSet<(usize, usize)>, row: usize, col: usize) -> i32 {
    OFFSETS
        .iter()
        .filter(|(row_offset, col_offset)| {
            if let Some(row_to_check) = (row as isize).checked_add(*row_offset)
                && let Some(col_to_check) = (col as isize).checked_add(*col_offset)
            {
                let coord = (row_to_check as usize, col_to_check as usize);
                grid.contains(&coord)
            } else {
                false
            }
        })
        .count() as i32
}

pub fn part_1(input: &str) -> i32 {
    let grid: HashSet<_> = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.char_indices().filter_map(move |(col_idx, chr)| {
                if chr == ROLL {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    grid.iter()
        .filter(|(row, col)| count_adjacent(&grid, *row, *col) < 4)
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut grid: HashSet<_> = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.char_indices().filter_map(move |(col_idx, chr)| {
                if chr == ROLL {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();
    let start_len = grid.len();

    loop {
        let old_len = grid.len();
        let to_remove: Vec<_> = grid
            .iter()
            .filter_map(|(row, col)| {
                if count_adjacent(&grid, *row, *col) < 4 {
                    Some((*row, *col))
                } else {
                    None
                }
            })
            .collect();

        for coord in to_remove {
            grid.remove(&coord);
        }

        if old_len == grid.len() {
            return (start_len - old_len) as i32;
        }
    }
}

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
