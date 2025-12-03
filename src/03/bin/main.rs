// https://adventofcode.com/2025/day/3

fn sum_max_jolts(input: &str, max_batteries: usize) -> i64 {
    input
        .lines()
        .map(|line| find_max_jolt(line, max_batteries))
        .sum()
}

fn find_max_jolt(bank: &str, max_batteries: usize) -> i64 {
    let mut result_bank = String::from(bank);

    while result_bank.len() > max_batteries {
        let mut removed = false;
        let mut nums: Vec<_> = result_bank.chars().filter_map(|c| c.to_digit(10)).collect();
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] {
                nums.remove(i);
                result_bank = nums.iter().map(|c| c.to_string()).collect();
                removed = true;
                break;
            }
        }

        if !removed {
            result_bank.truncate(max_batteries);
            break;
        }
    }

    result_bank.parse::<i64>().unwrap()
}

pub fn part_1(input: &str) -> i64 {
    sum_max_jolts(input, 2)
}

pub fn part_2(input: &str) -> i64 {
    sum_max_jolts(input, 12)
}

fn main() {
    let input = std::fs::read_to_string("input/03.in").unwrap();
    // let input = std::fs::read_to_string("input/03.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
