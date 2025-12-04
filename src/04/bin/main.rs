// https://adventofcode.com/2025/day/4

// https://www.reddit.com/r/csharp/comments/1b5wglp/how_to_convert_a_2d_index_into_a_1d_index/

#[derive(Debug)]
struct Grid1D<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> From<&str> for Grid1D<T>
where
    T: From<char>,
{
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();
        let values = value
            .lines()
            .flat_map(|row| row.chars().map(T::from))
            .collect();

        Self {
            values,
            width: lines[0].len(),
            height: lines.len(),
        }
    }
}

impl<T> Grid1D<T> {
    #[rustfmt::skip]
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1)
    ];

    fn get_from_1d_index(&self, idx: usize) -> Option<&T> {
        self.values.get(idx)
    }

    fn get_from_2d_index(&self, row: usize, col: usize) -> Option<&T> {
        if let Some(idx) = self.get_1d_index(row, col) {
            self.values.get(idx)
        } else {
            None
        }
    }

    fn get_1d_index(&self, row: usize, col: usize) -> Option<usize> {
        let idx = (row * self.height) + col;
        if idx < self.values.len() {
            Some(idx)
        } else {
            None
        }
    }

    fn count_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        self.values.iter().filter(|v| f(*v)).count()
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

impl Grid1D<PaperRollState> {
    fn count_adjacent_2d(&self, row: usize, col: usize) -> i32 {
        let res = Self::OFFSETS
            .iter()
            .filter(|(row_offset, col_offset)| {
                let row_to_check = row as isize + row_offset;
                let col_to_check = col as isize + col_offset;

                if row_to_check < 0
                    || col_to_check < 0
                    || row_to_check as usize >= self.height
                    || col_to_check as usize >= self.width
                {
                    return false;
                }

                matches!(
                    self.get_from_2d_index(row_to_check as usize, col_to_check as usize),
                    Some(PaperRollState::Present)
                )
            })
            .count() as i32;

        res
    }

    fn count_adjacent_1d(&self, idx: usize) -> i32 {
        let row = idx / self.width;
        let col = idx % self.width;
        self.count_adjacent_2d(row, col)
    }
}

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

pub fn part_1(input: &str) -> i32 {
    let grid: Grid1D<PaperRollState> = Grid1D::from(input);
    grid.values
        .iter()
        .enumerate()
        .filter(|(idx, _)| {
            matches!(grid.get_from_1d_index(*idx), Some(PaperRollState::Present))
                && grid.count_adjacent_1d(*idx) < 4
        })
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let mut grid: Grid1D<PaperRollState> = Grid1D::from(input);
    let start_count = grid.count_by(|v| matches!(v, PaperRollState::Present));

    loop {
        let new_values: Vec<_> = grid
            .values
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                if matches!(grid.get_from_1d_index(idx), Some(PaperRollState::Present)) {
                    match grid.count_adjacent_1d(idx) {
                        0..4 => PaperRollState::Absent,
                        _ => PaperRollState::Present,
                    }
                } else {
                    PaperRollState::Absent
                }
            })
            .collect();
        let old_count = grid.count_by(|v| matches!(v, PaperRollState::Present));
        grid.values = new_values;
        let new_count = grid.count_by(|v| matches!(v, PaperRollState::Present));
        if old_count == new_count {
            return (start_count - old_count) as i32;
        }
    }
}

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
