// https://adventofcode.com/2025/day/5

pub fn part_1(input: &str) -> i64 {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges: Vec<std::ops::RangeInclusive<i64>> = ranges
        .lines()
        .map(|r| {
            let (start, end) = r.split_once('-').unwrap();
            let (start, end): (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());
            start..=end
        })
        .collect();
    let mut fresh = 0;

    for id in ids.lines() {
        let id = id.parse::<i64>().unwrap();
        if ranges.iter().any(|r| r.contains(&id)) {
            fresh += 1
        }
    }

    fresh
}

pub fn part_2(input: &str) -> i32 {
    0
}

fn main() {
    // let input = std::fs::read_to_string("input/05.example").unwrap();
    let input = std::fs::read_to_string("input/05.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
