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
        let mut removed = false;
        let mut nums = max_bank
            .chars()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .peekable();

        while let Some((idx, cur)) = nums.next() {
            if let Some((_, next)) = nums.peek()
                && cur < *next
            {
                max_bank.remove(idx);
                removed = true;
                break;
            }
        }

        if !removed {
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
    // let input = std::fs::read_to_string("input/03.example").unwrap();
    let input = std::fs::read_to_string("input/03.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
