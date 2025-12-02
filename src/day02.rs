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
            // For part 1, all we're doing is checking if the number is a palindrome
            ValidationMode::Exact => (self.start..=self.end)
                .filter(|i| {
                    let i_str = i.to_string();
                    let half = i_str.len() / 2;
                    i_str.chars().take(half).eq(i_str.chars().skip(half))
                })
                .sum(),
            ValidationMode::AtLeast => {
                let mut sum = 0i64;
                for i in self.start..=self.end {
                    // Single digits can't be invalid
                    if i < 10 {
                        continue;
                    }
                    let i_str = i.to_string();
                    let mut chunk_size = 1usize;
                    loop {
                        let mut chunks = i_str.as_bytes().chunks(chunk_size);
                        let first = chunks.nth(0).unwrap();
                        if chunks.all(|c| c == first) {
                            sum += i;
                            break;
                        }

                        chunk_size += 1;
                        if chunk_size > i_str.len() / 2 {
                            break;
                        }
                    }
                }

                sum
            }
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
