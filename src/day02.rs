// https://adventofcode.com/2025/day/2

#[derive(Debug)]
struct IdRange {
    start: i64,
    end: i64,
}

impl IdRange {
    fn sum_invalid(&self) -> i64 {
        let mut sum = 0i64;
        for i in self.start..self.end + 1 {
            let i_str = i.to_string();

            // It's impossible for odd-digit numbers to be invalid?
            if i_str.len() % 2 != 0 {
                continue;
            }

            let mut p2: usize = 1;
            loop {
                let pat = i_str.chars().take(p2);
                let rest = i_str.chars().skip(p2);

                if rest.eq(pat) {
                    sum += i;
                    break;
                }

                p2 += 1;
                if p2 > i_str.len() / 2 {
                    break;
                }
            }
        }
        sum
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
        .fold(0i64, |acc, r| acc + r.sum_invalid())
}
