// https://adventofcode.com/2025/day/5

use std::cmp::{max, min};

fn parse_ranges(lines: &str) -> Vec<std::ops::RangeInclusive<i64>> {
    let mut ranges: Vec<std::ops::RangeInclusive<i64>> = lines
        .lines()
        .map(|r| {
            let (start, end) = r.split_once('-').unwrap();
            let (start, end): (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());
            start..=end
        })
        .collect();
    ranges.sort_by_key(|r| *r.start());

    while let Some((existing_idx, [w0, w1])) = ranges
        .windows(2)
        .enumerate()
        .find(|(_, w)| w[0].start() <= w[1].end() && w[1].start() <= w[0].end())
    {
        ranges[existing_idx] = min(*w0.start(), *w1.start())..=max(*w0.end(), *w1.end());
        ranges.remove(existing_idx + 1);
    }

    ranges
}

pub fn part_1(input: &str) -> i64 {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(ranges);
    let mut fresh = 0;

    for id in ids.lines() {
        let id = id.parse::<i64>().unwrap();
        if ranges.iter().any(|r| r.contains(&id)) {
            fresh += 1
        }
    }

    fresh
}

pub fn part_2(input: &str) -> i64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(ranges);

    let mut fresh = 0;
    for r in ranges {
        fresh += r.count() as i64
    }

    fresh
}

fn main() {
    // let input = std::fs::read_to_string("input/05.example").unwrap();
    let input = std::fs::read_to_string("input/05.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
