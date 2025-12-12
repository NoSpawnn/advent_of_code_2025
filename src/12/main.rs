// https://adventofcode.com/2025/day/12

type Answer = i32;
type Shape = Vec<Vec<bool>>;

fn parse_shape(block: &str) -> Shape {
    block
        .lines()
        .skip(1)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn parse_region(line: &str) -> (String, Vec<usize>) {
    let (area, shapes) = line.split_once(':').unwrap();
    let shapes = shapes
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    (area.into(), shapes)
}

fn parse(lines: &str) -> (impl Iterator<Item = (String, Vec<usize>)>, Vec<Shape>) {
    let mut p = lines.split("\n\n").peekable();
    let mut shapes = Vec::new();

    while let Some(block) = p.next() {
        if p.peek().is_none() {
            return (block.lines().map(parse_region), shapes);
        } else {
            shapes.push(parse_shape(block));
        }
    }

    // we'll always reach the end of the iterator
    unreachable!()
}

fn region_area(width: usize, height: usize) -> usize {
    width * height
}

fn shape_area(s: &Shape) -> usize {
    s.len() * s[0].len()
}

pub fn part_1(input: &str) -> Answer {
    let (regions, shapes) = parse(input);

    let mut ans = 0;

    for (dimensions, presents) in regions {
        let (width, height) = dimensions.split_once('x').unwrap();
        let width = width.parse::<usize>().unwrap();
        let height = height.parse::<usize>().unwrap();
        let region_area = region_area(width, height);
        let total_shape_area: usize = presents
            .iter()
            .enumerate()
            .map(|(shape_idx, &count)| {
                (1..count)
                    .map(|_| shape_area(&shapes[shape_idx]))
                    .sum::<usize>()
            })
            .sum();
        if total_shape_area < region_area {
            ans += 1;
        }
    }

    ans
}

pub fn part_2(input: &str) -> Answer {
    todo!("day 12 part 2")
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
