// https://adventofcode.com/2025/day/4

// https://www.reddit.com/r/csharp/comments/1b5wglp/how_to_convert_a_2d_index_into_a_1d_index/
// array2D[A_LENGTH][B_LENGTH] -> array1D: index = (b_index * A_LENGTH) + a_index

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

    fn at(&self, row: usize, col: usize) -> Option<&T> {
        if let Some(idx) = self.idx(row, col)
            && let Some(value) = self.values.get(idx)
        {
            Some(value)
        } else {
            None
        }
    }

    fn idx(&self, row: usize, col: usize) -> Option<usize> {
        let idx = (col * self.height) + row;
        if idx < self.values.len() {
            Some(idx)
        } else {
            None
        }
    }
}

impl Grid1D<PaperRollState> {
    fn count_adjacent(&self, row: usize, col: usize) -> i32 {
        Self::OFFSETS
            .iter()
            .filter(|(row_offset, col_offset)| {
                let row_to_check = (row as isize) + row_offset;
                let col_to_check = (col as isize) + col_offset;

                if row_to_check < 0
                    || col_to_check < 0
                    || row_to_check as usize >= self.height
                    || col_to_check as usize >= self.width
                {
                    return false;
                }

                if let Some(idx) = self.idx(row_to_check as usize, col_to_check as usize) {
                    matches!(self.values[idx as usize], PaperRollState::Present)
                } else {
                    false
                }
            })
            .count() as i32
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
    let mut res = 0;
    for row in 0..input.lines().count() {
        for col in 0..input.lines().next().unwrap().len() {
            if !matches!(grid.at(row, col), Some(PaperRollState::Absent))
                && grid.count_adjacent(row, col) < 4
            {
                res += 1;
            }
        }
    }
    res
}

// pub fn part_2(input: &str) -> i32 {
//     let mut grid = make_grid(input);
//     let start_len = grid
//         .iter()
//         .filter(|(_, s)| matches!(s, PaperRollState::Present))
//         .count();

//     loop {
//         let new_grid: PaperRollCoords = grid
//             .iter()
//             .filter_map(|((row, col), _)| {
//                 (count_adjacent(&grid, *row, *col) >= 4)
//                     .then_some(((*row, *col), PaperRollState::Present))
//             })
//             .collect();
//         let old_len = grid
//             .iter()
//             .filter(|(_, s)| matches!(s, PaperRollState::Present))
//             .count();

//         if old_len == new_grid.len() {
//             return (start_len - old_len) as i32;
//         }

//         grid = new_grid;
//     }
// }

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}
