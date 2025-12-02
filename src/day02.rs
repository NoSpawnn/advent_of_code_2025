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
            // For part 2, we need to check any possible combination of groupings
            ValidationMode::AtLeast => (self.start..=self.end)
                .filter(|i| *i >= 10)
                .filter(|i| {
                    let i_str = i.to_string();
                    (1..=i_str.len() / 2)
                        .map(|chunk_size| i_str.as_bytes().chunks(chunk_size))
                        .any(|mut collection| {
                            let first = collection.nth(0).unwrap();
                            collection.all(|c| c == first)
                        })
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
