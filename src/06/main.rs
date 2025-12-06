// https://adventofcode.com/2025/day/6

type Answer = i64;

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

fn parse_problem(lines: &[String], col_from: usize, col_to: usize) -> impl Iterator<Item = String> {
    lines.iter().map(move |line| {
        line.chars()
            .skip(col_from)
            .take(col_to - col_from)
            .collect()
    })
}

fn parse_problems<'a>(lines: &'a str) -> Vec<Vec<String>> {
    let lines: Vec<_> = lines.lines().map(str::to_owned).collect();
    let col_end = lines[0].len();
    let mut block_start = 0;
    let mut problems = Vec::new();

    for col in 0..col_end {
        let column = get_column(&lines, &col).collect::<String>();
        if column.trim().is_empty() {
            let problem_lines = parse_problem(&lines, block_start, col);
            problems.push(problem_lines.collect());
            block_start = col + 1;
        }
    }

    problems.push(parse_problem(&lines, block_start, col_end).collect());
    problems
}

pub fn part_1(input: &str) -> Answer {
    parse_problems(input).iter().fold(0, |acc, problem| {
        let op = problem.last().unwrap().chars().nth(0).unwrap();
        let nums = problem[..problem.len() - 1]
            .iter()
            .map(|col| col.trim().parse::<i64>().unwrap());
        match op {
            '*' => acc + nums.fold(1, |acc, n| acc * n),
            '+' => acc + nums.sum::<i64>(),
            _ => panic!("unknown operator {}", op),
        }
    })
}

pub fn part_2(input: &str) -> Answer {
    let problems = parse_problems(input);
    problems
        .iter()
        .map(|p| transpose(&p).peekable())
        .fold(0, |acc, mut problem| {
            let op = problem.peek().unwrap().chars().last().unwrap();
            let nums = problem.map(|col| {
                col.chars()
                    .take(col.len() - 1)
                    .collect::<String>()
                    .trim()
                    .parse::<i64>()
                    .unwrap()
            });
            match op {
                '*' => acc + nums.fold(1, |acc, n| acc * n),
                '+' => acc + nums.sum::<i64>(),
                _ => panic!("unknown operator {}", op),
            }
        })
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
