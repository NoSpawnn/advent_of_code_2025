// https://adventofcode.com/2025/day/4

use aoc_common::Grid1D;

#[derive(Debug, PartialEq, Eq)]
enum PaperRollState {
    Present,
    Absent,
}

impl From<char> for PaperRollState {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Present,
            _ => Self::Absent,
        }
    }
}

impl PaperRollState {
    fn count_adjacent_in_grid(grid: &Grid1D<Self>, idx: usize) -> i32 {
        grid.count_adjacent_1d(idx, |row, col| {
            matches!(
                grid.get_from_2d_index(row, col),
                Some(PaperRollState::Present)
            )
        })
    }
}

pub fn part_1(input: &str) -> i32 {
    let grid: Grid1D<PaperRollState> = Grid1D::from(input);
    grid.values
        .iter()
        .enumerate()
        .filter(|(idx, _)| {
            matches!(grid.get_from_1d_index(*idx), Some(PaperRollState::Present))
                && PaperRollState::count_adjacent_in_grid(&grid, *idx) < 4
        })
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut grid: Grid1D<PaperRollState> = Grid1D::from(input);
    let start_count = grid.count_by(|v| matches!(v, PaperRollState::Present));

    loop {
        let mut rolls_next = 0usize;
        let next_values: Vec<_> = grid
            .values
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                let is_present = matches!(v, PaperRollState::Present);
                let adj = PaperRollState::count_adjacent_in_grid(&grid, idx);

                let next = if is_present && adj < 4 {
                    PaperRollState::Absent
                } else {
                    PaperRollState::Present
                };

                if matches!(next, PaperRollState::Present) {
                    rolls_next += 1;
                }

                next
            })
            .collect();

        let rolls_now = grid
            .values
            .iter()
            .filter(|v| matches!(v, PaperRollState::Present))
            .count();

        if rolls_now == rolls_next {
            return start_count as i32 - rolls_now as i32;
        }

        grid.values = next_values;
    }
}

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
