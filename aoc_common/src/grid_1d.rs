#[derive(Debug)]
pub struct Grid1D<T> {
    pub values: Vec<T>,
    pub width: usize,
    pub height: usize,
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

    pub fn get_from_1d_index(&self, idx: usize) -> Option<&T> {
        self.values.get(idx)
    }

    pub fn get_from_2d_index(&self, row: usize, col: usize) -> Option<&T> {
        if let Some(idx) = self.get_1d_index(row, col) {
            self.values.get(idx)
        } else {
            None
        }
    }

    pub fn get_1d_index(&self, row: usize, col: usize) -> Option<usize> {
        // https://www.reddit.com/r/csharp/comments/1b5wglp/how_to_convert_a_2d_index_into_a_1d_index/
        let idx = (row * self.height) + col;
        if idx < self.values.len() {
            Some(idx)
        } else {
            None
        }
    }

    pub fn count_adjacent_2d<F: Fn(usize, usize) -> bool>(
        &self,
        row: usize,
        col: usize,
        f: F,
    ) -> i32 {
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

                f(row_to_check as usize, col_to_check as usize)
            })
            .count() as i32;

        res
    }

    pub fn count_adjacent_1d<F: Fn(usize, usize) -> bool>(&self, idx: usize, f: F) -> i32 {
        let row = idx / self.width;
        let col = idx % self.width;
        self.count_adjacent_2d(row, col, f)
    }

    pub fn count_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        self.values.iter().filter(|v| f(*v)).count()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
