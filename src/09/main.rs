// https://adventofcode.com/2025/day/9

type Answer = usize;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        // input is column major
        let (col, row) = value.split_once(',').unwrap();
        Self {
            row: row.parse().expect("row should be numeric"),
            col: col.parse().expect("col should be numeric"),
        }
    }
}

fn area(a: &Point, b: &Point) -> usize {
    let width = a.col.abs_diff(b.col) + 1;
    let height = a.row.abs_diff(b.row) + 1;
    width * height
}

fn biggest_area(points: &[Point]) -> usize {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| (p1, p2)))
        .map(|(p1, p2)| area(p1, p2))
        .max()
        .expect("points should not be empty")
}

pub fn part_1(input: &str) -> Answer {
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    biggest_area(&points)
}

pub fn part_2(input: &str) -> Answer {
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    let edges: Vec<(&Point, &Point)> = points
        .windows(2)
        .take(points.len())
        .map(|vertices| (&vertices[0], &vertices[1]))
        .chain(Some((&points[0], &points[points.len() - 1])))
        .collect();
    let mut possible: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| (p1, p2, area(p1, p2))))
        .collect();
    possible.sort_by_key(|(_, _, a)| *a);
    possible
        .iter()
        .rev()
        .find(|(p1, p2, _)| {
            edges.iter().all(|(start, end)| {
                let before = p1.col.max(p2.col) <= start.col.min(end.col);
                let after = p1.col.min(p2.col) >= start.col.max(end.col);
                let above = p1.row.max(p2.row) <= start.row.min(end.row);
                let below = p1.row.min(p2.row) >= start.row.max(end.row);
                before || after || above || below
            })
        })
        .expect("possible should not be empty")
        .2
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
