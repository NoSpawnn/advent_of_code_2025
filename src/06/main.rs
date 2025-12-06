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

        let op = lines.last().expect("input empty")[col_from..col_to]
            .chars()
            .next()
            .expect("operator line empty");

        if column_wise {
            let nums = transpose(&block)
                .map(|col| col.trim().parse::<i64>().unwrap())
                .collect();

            return Self { op, nums };
        }

        let nums = block
            .iter()
            .map(|col| col.trim().parse::<i64>().unwrap())
            .collect();

        Self { op, nums }
    }

    fn parse_many<'a>(lines: &'a str, column_wise: bool) -> Vec<Self> {
        let lines: Vec<_> = lines.lines().collect();
        let col_end = lines[0].len();
        let mut block_start = 0;
        let mut problems = Vec::with_capacity(1000); // there's 1000 problems on the real input, this is a 10-15µs time save

        for col in 0..col_end {
            if get_column(&lines, &col).all(|c| c.is_whitespace()) {
                let problem = Self::parse_one(&lines, block_start, col, column_wise);
                problems.push(problem);
                block_start = col + 1;
            }
        }

        problems.push(Self::parse_one(&lines, block_start, col_end, column_wise));
        problems
    }
}

fn get_column<'a>(lines: &'a [&str], col: &'a usize) -> impl Iterator<Item = char> {
    lines.iter().map(|line| char::from(line.as_bytes()[*col]))
}

fn transpose<'a>(lines: &'a [&str]) -> impl Iterator<Item = String> {
    let len = lines.first().unwrap().len();
    (0..len).map(|col| get_column(lines, &col).collect())
}

pub fn part_1(input: &str) -> Answer {
    Problem::parse_many(input, false)
        .iter()
        .fold(0, |acc, problem| match problem.op {
            '*' => acc + problem.nums.iter().fold(1, |acc, n| acc * n),
            '+' => acc + problem.nums.iter().sum::<i64>(),
            _ => panic!("unknown operator {}", problem.op),
        })
}

pub fn part_2(input: &str) -> Answer {
    Problem::parse_many(input, true)
        .iter()
        .fold(0, |acc, problem| match problem.op {
            '*' => acc + problem.nums.iter().fold(1, |acc, n| acc * n),
            '+' => acc + problem.nums.iter().sum::<i64>(),
            _ => panic!("unknown operator {}", problem.op),
        })
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
