// https://adventofcode.com/2025/day/9

type Answer = usize;
type Point = (usize, usize);

fn area(a: &Point, b: &Point) -> usize {
    let width = a.0.abs_diff(b.0) + 1;
    let height = a.1.abs_diff(b.1) + 1;
    width * height
}

fn biggest_area<'a>(points: &'a [Point]) -> (&'a Point, &'a Point) {
    let all_pairs = points
        .iter()
        .enumerate()
        .flat_map(|(i, j1)| points[i + 1..].iter().map(move |j2| (j1, j2)));

    all_pairs
        .max_by(|a, b| area(a.0, a.1).cmp(&area(b.0, b.1)))
        .expect("")
}

pub fn part_1(input: &str) -> Answer {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let parts = line.split_once(',').unwrap();
            (parts.0.parse().unwrap(), parts.1.parse().unwrap())
        })
        .collect();

    let p = biggest_area(&points);
    area(p.0, p.1)
}

pub fn part_2(input: &str) -> Answer {
    todo!("day 9 part 2")
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
