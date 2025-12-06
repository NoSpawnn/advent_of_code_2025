// https://adventofcode.com/2025/day/6

type Answer = i64;

#[derive(Debug)]
struct Problem {
    op: char,
    nums: Vec<i64>,
}

fn get_column<'a>(lines: &'a [String], col: &'a usize) -> impl Iterator<Item = char> {
    lines.iter().map(|line| line.chars().nth(*col).unwrap())
}

fn transpose(lines: &[String]) -> impl Iterator<Item = String> {
    let len = lines.first().unwrap().len();
    (0..len).map(|col| get_column(lines, &col).collect())
}

fn parse_problem(lines: &[String], col_from: usize, col_to: usize, column_wise: bool) -> Problem {
    let mut block: Vec<_> = lines
        .iter()
        .map(move |line| {
            line.chars()
                .skip(col_from)
                .take(col_to - col_from)
                .collect::<String>()
        })
        .collect();

    let op = block.last().unwrap().chars().nth(0).unwrap();

    if column_wise {
        block = transpose(&block[..block.len() - 1]).collect();
    } else {
        block.drain(block.len() - 1..);
    }

    let nums = block
        .iter()
        .map(|col| col.trim().parse::<i64>().unwrap())
        .collect();

    Problem { op, nums }
}

fn parse_problems<'a>(lines: &'a str, column_wise: bool) -> Vec<Problem> {
    let lines: Vec<_> = lines.lines().map(str::to_owned).collect();
    let col_end = lines[0].len();
    let mut block_start = 0;
    let mut problems = Vec::new();

    for col in 0..col_end {
        let column = get_column(&lines, &col).collect::<String>();
        if column.trim().is_empty() {
            let problem = parse_problem(&lines, block_start, col, column_wise);
            problems.push(problem);
            block_start = col + 1;
        }
    }

    problems.push(parse_problem(&lines, block_start, col_end, column_wise));
    problems
}

pub fn part_1(input: &str) -> Answer {
    parse_problems(input, false)
        .iter()
        .fold(0, |acc, problem| match problem.op {
            '*' => acc + problem.nums.iter().fold(1, |acc, n| acc * n),
            '+' => acc + problem.nums.iter().sum::<i64>(),
            _ => panic!("unknown operator {}", problem.op),
        })
}

pub fn part_2(input: &str) -> Answer {
    let problems = parse_problems(input, true);
    problems.iter().fold(0, |acc, problem| match problem.op {
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
