// https://adventofcode.com/2025/day/6

type Answer = i64;

#[derive(Debug)]
struct Problem {
    op: char,
    nums: Vec<i64>,
}

impl Problem {
    fn parse_one(lines: &[&str], col_from: usize, col_to: usize, column_wise: bool) -> Self {
        let block: Vec<_> = lines
            .iter()
            .take(lines.len() - 1) // we care not about the last line for numbers parsing
            .map(move |line| &line[col_from..col_to])
            .collect();

        let op = lines[lines.len() - 1][col_from..col_to]
            .chars()
            .next()
            .expect("operator line should not be empty");

        if column_wise {
            let nums = transpose(&block)
                .map(|col| col.trim().parse::<i64>().unwrap())
                .collect();

            return Self { op, nums };
        }

        let nums = block
            .iter()
            .map(|col| col.trim().parse().unwrap())
            .collect();

        Self { op, nums }
    }

    fn parse_many<'a>(lines: &'a str, column_wise: bool) -> Vec<Self> {
        let lines: Vec<_> = lines.lines().collect();
        let col_end = lines[0].len();
        let mut block_start = 0;
        let mut problems = Vec::with_capacity(1000); // there's 1000 problems on the real input, this is a 10-15µs time save

        for col in 0..col_end {
            if get_column(&lines, &col).all(char::is_whitespace) {
                let problem = Self::parse_one(&lines, block_start, col, column_wise);
                problems.push(problem);
                block_start = col + 1;
            }
        }

        problems.push(Self::parse_one(&lines, block_start, col_end, column_wise));
        problems
    }

    fn aggregate_by_op(&self) -> Answer {
        match self.op {
            '*' => self.nums.iter().product(),
            '+' => self.nums.iter().sum(),
            _ => panic!("unknown operator {}", self.op),
        }
    }
}

fn get_column<'a>(lines: &'a [&str], col: &'a usize) -> impl Iterator<Item = char> {
    lines.iter().map(|line| char::from(line.as_bytes()[*col])) // Only works on ASCII strings, but that's fine
}

fn transpose<'a>(lines: &'a [&str]) -> impl Iterator<Item = String> {
    (0..lines[0].len()).map(|col| get_column(lines, &col).collect())
}

pub fn part_1(input: &str) -> Answer {
    Problem::parse_many(input, false)
        .iter()
        .map(Problem::aggregate_by_op)
        .sum()
}

pub fn part_2(input: &str) -> Answer {
    Problem::parse_many(input, true)
        .iter()
        .map(Problem::aggregate_by_op)
        .sum()
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
