#[rustfmt::skip]
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1)
];
type PaperRollCoords = std::collections::HashMap<(usize, usize), PaperRollState>;

enum PaperRollState {
    Present,
    Absent,
}

fn count_adjacent(grid: &PaperRollCoords, row: usize, col: usize) -> i32 {
    OFFSETS
        .iter()
        .filter(|(row_offset, col_offset)| {
            let row_to_check = (row as isize) + row_offset;
            let col_to_check = (col as isize) + col_offset;
            let coord = (row_to_check as usize, col_to_check as usize);
            matches!(grid.get(&coord), Some(PaperRollState::Present))
        })
        .count() as i32
}

fn make_grid(lines: &str) -> PaperRollCoords {
    lines
        .lines()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.char_indices().filter_map(move |(col_idx, chr)| {
                (chr == '@').then(|| ((row_idx, col_idx), PaperRollState::Present))
            })
        })
        .collect()
}

pub fn part_1(input: &str) -> i32 {
    let grid = make_grid(input);
    grid.iter()
        .filter(|((row, col), _)| count_adjacent(&grid, *row, *col) < 4)
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut grid = make_grid(input);
    let start_len = grid
        .iter()
        .filter(|(_, s)| matches!(s, PaperRollState::Present))
        .count();

    loop {
        let new_grid: PaperRollCoords = grid
            .iter()
            .filter_map(|((row, col), _)| {
                (count_adjacent(&grid, *row, *col) >= 4)
                    .then_some(((*row, *col), PaperRollState::Present))
            })
            .collect();
        let old_len = grid
            .iter()
            .filter(|(_, s)| matches!(s, PaperRollState::Present))
            .count();

        if old_len == new_grid.len() {
            return (start_len - old_len) as i32;
        }

        grid = new_grid;
    }
}

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
