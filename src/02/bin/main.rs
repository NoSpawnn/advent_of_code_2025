// https://adventofcode.com/2025/day/2

#[derive(Debug)]
struct IdRange {
    start: i64,
    end: i64,
}

enum ValidationMode {
    Exact,
    AtLeast,
}

impl IdRange {
    fn sum_invalid(&self, mode: ValidationMode) -> i64 {
        match mode {
            // For part 1, all we're doing is checking if each half of the number (as a string) is equal
            ValidationMode::Exact => (self.start..=self.end)
                .filter(|i| {
                    let i_str = i.to_string();
                    let i_bytes = i_str.as_bytes();
                    let half = i_bytes.len() / 2;
                    i_bytes[half..] == i_bytes[..half]
                })
                .sum(),
            ValidationMode::AtLeast => (self.start..=self.end)
                .filter(|i| {
                    let i_str = i.to_string();
                    let i_bytes = i_str.as_bytes();
                    (1..=i_str.len() / 2).any(|chunk_size| {
                        let mut chunks = i_bytes.chunks(chunk_size);
                        let first = chunks.next().unwrap(); // this'll never be empty
                        chunks.all(|c| c == first)
                    })

                    // Alternate solution, thanks @5ak1r, I'm unsure which is actually faster
                    // Duplicate the string (i_str + i_str), drop the first and last chars, check if the original i_str is present
                    // let i_str = i.to_string();
                    // i_str.repeat(2)[1..=(i_str.len() * 2 - 2)].contains(&i_str)
                })
                .sum(),
        }
    }
}

impl From<&str> for IdRange {
    fn from(value: &str) -> Self {
        let bounds: Vec<_> = value.split('-').collect();

        assert_eq!(
            bounds.len(),
            2,
            "Failed to parse '{value}' into Range: expected 2 bounds, got {}",
            bounds.len()
        );

        let start = bounds[0].parse().unwrap();
        let end = bounds[1].parse().unwrap();

        Self { start, end }
    }
}

pub fn part_1(input: &str) -> i64 {
    input
        .split(',')
        .map(IdRange::from)
        .fold(0i64, |acc, r| acc + r.sum_invalid(ValidationMode::Exact))
}

pub fn part_2(input: &str) -> i64 {
    input
        .split(',')
        .map(IdRange::from)
        .fold(0i64, |acc, r| acc + r.sum_invalid(ValidationMode::AtLeast))
}

fn main() {
    let input = std::fs::read_to_string("input/02.example").unwrap();
    // let input = std::fs::read_to_string("input/02.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
