// https://adventofcode.com/2025/day/5

use std::cmp::{max, min};

type RangeVec = Vec<std::ops::RangeInclusive<i64>>;

fn disjoint_ranges(lines: &str) -> RangeVec {
    let mut ranges: RangeVec = lines
        .lines()
        .map(|r| {
            let (start, end) = r.split_once('-').unwrap();
            let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
            start..=end
        })
        .collect();
    ranges.sort_by_key(|r| *r.start());

    // Thanks https://stackoverflow.com/a/3269471
    // We know pair[0].start() <= pair[1].end() holds since the vec is sorted
    while let Some((existing_idx, [r0, r1])) = ranges
        .windows(2)
        .enumerate()
        .find(|(_, pair)| pair[1].start() <= pair[0].end())
    {
        ranges[existing_idx] = min(*r0.start(), *r1.start())..=max(*r0.end(), *r1.end());
        ranges.remove(existing_idx + 1);
    }

    ranges
}

pub fn part_1(input: &str) -> usize {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = disjoint_ranges(ranges);
    ids.lines()
        .filter_map(|id| id.parse::<i64>().ok())
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

pub fn part_2(input: &str) -> i64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    disjoint_ranges(ranges)
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

fn main() {
    // let input = include_str!("input//input.example");
    let input = include_str!("input//real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
