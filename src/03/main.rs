// https://adventofcode.com/2025/day/3

fn sum_max_jolts(input: &str, max_batteries: usize) -> i64 {
    input
        .lines()
        .map(|line| find_max_jolt(line, max_batteries))
        .sum()
}

fn find_max_jolt(bank: &str, max_batteries: usize) -> i64 {
    let mut max_bank = String::from(bank);

    while max_bank.len() > max_batteries {
        // new function discovered
        if let Some(idx) = max_bank.as_bytes().windows(2).position(|w| w[0] < w[1]) {
            max_bank.remove(idx);
        } else {
            max_bank.truncate(max_batteries);
            break;
        }
    }

    max_bank
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("Input is invalid, got: {}", max_bank))
}

pub fn part_1(input: &str) -> i64 {
    sum_max_jolts(input, 2)
}

pub fn part_2(input: &str) -> i64 {
    sum_max_jolts(input, 12)
}

fn main() {
    // let input = include_str!("input//input.example");
    let input = include_str!("input//real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
