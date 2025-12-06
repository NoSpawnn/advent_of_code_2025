// https://adventofcode.com/2025/day/6

type Answer = i64;

pub fn part_1(input: &str) -> Answer {
    let mut operands: Vec<Vec<Answer>> = Vec::new();
    let mut operators: Option<Vec<char>> = None;

    for line in input.lines() {
        if line.chars().next().is_some_and(|c| matches!(c, '*' | '+')) {
            operators = Some(line.split_whitespace().flat_map(|s| s.chars()).collect());
            break;
        } else {
            for (idx, n) in line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .enumerate()
            {
                if let Some(v) = operands.get_mut(idx) {
                    v.push(n);
                } else {
                    operands.push(Vec::from([n]));
                }
            }
        }
    }

    if operators.is_none() {
        panic!("Didn't find operators line");
    }

    let operators = operators.unwrap();

    let mut ans = 0;
    for (operands, operator) in operands.iter().zip(operators) {
        ans += operands
            .iter()
            .skip(1)
            .fold(operands[0], |acc, next| match operator {
                '*' => acc * next,
                '+' => acc + next,
                _ => panic!("Unknown operator"),
            });
    }

    ans
}

pub fn part_2(input: &str) -> Answer {
    todo!()
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
