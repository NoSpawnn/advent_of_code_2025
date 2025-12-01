// https://adventofcode.com/2025/day/1

use std::{ops::Rem, thread::current};

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct DialMove {
    dir: Direction,
    count: i32,
}

impl From<&str> for DialMove {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        let dir = match chars.next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid input: {value}"),
        };
        let count = match chars.collect::<String>().parse::<i32>() {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse integer from '{value}': {e}"),
        };

        Self { dir, count }
    }
}

pub fn part_1(input: &str) -> i32 {
    const DIAL_SIZE: i32 = 100;
    let mut pos = 50; // Dial starts at 50
    let mut seq = vec![pos];

    for line in input.lines() {
        let m = DialMove::from(line);
        pos = (pos
            + if m.dir == Direction::Left {
                -m.count
            } else {
                m.count
            })
        .rem_euclid(DIAL_SIZE);
        seq.push(pos);
    }

    seq.into_iter().filter(|x| *x == 0).count() as i32
}

pub fn part_2(input: &str) -> i32 {
    const DIAL_SIZE: i32 = 100; // Total numbers on the dial
    let mut pos = 50; // Dial starts at 50
    let mut seq = vec![pos];

    for line in input.lines() {
        let m = DialMove::from(line);
        let step: i32 = if m.dir == Direction::Left { -1 } else { 1 };
        for _ in 0..m.count {
            pos = (pos + step).rem_euclid(DIAL_SIZE);
            seq.push(pos);
        }
    }

    seq.into_iter().filter(|x| *x == 0).count() as i32
}
