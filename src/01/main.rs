// https://adventofcode.com/2025/day/1

const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct DialTurn {
    dir: Direction,
    count: i32,
}

impl From<&str> for DialTurn {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        let dir = match chars.next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => panic!("Invalid input: {value}"),
        };
        let count = match chars.collect::<String>().parse::<i32>() {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse integer from '{value}': {e}"),
        };

        Self { dir, count }
    }
}

fn dial_seq(
    start: i32,
    dial_size: i32,
    moves: impl Iterator<Item = DialTurn>,
) -> impl Iterator<Item = i32> {
    moves.scan(start, move |state, m| {
        *state = match m.dir {
            Direction::Left => *state - m.count,
            Direction::Right => *state + m.count,
        }
        .rem_euclid(dial_size);
        Some(*state)
    })
}

pub fn part_1(input: &str) -> i32 {
    let moves = input.lines().map(DialTurn::from);
    dial_seq(START_POS, DIAL_SIZE, moves)
        .filter(|x| *x == 0)
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let moves = input.lines().flat_map(|line| {
        let m = DialTurn::from(line);
        std::iter::repeat_with(move || DialTurn {
            dir: m.dir,
            count: 1,
        })
        .take(m.count as usize)
    });

    dial_seq(START_POS, DIAL_SIZE, moves)
        .filter(|x| *x == 0)
        .count() as i32
}

fn main() {
    // let input = include_str!("input/input.example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
