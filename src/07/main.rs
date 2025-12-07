// https://adventofcode.com/2025/day/7

type Answer = usize;

pub fn part_1(input: &str) -> Answer {
    let mut lines = input.lines();
    let mut splits = 0;
    let mut beams: Vec<bool> = lines
        .next()
        .expect("input should not be empty")
        .chars()
        .map(|c| c == 'S')
        .collect();

    while let Some(line) = lines.next() {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beams[i] {
                beams[i - 1] = true;
                beams[i + 1] = true;
                beams[i] = false;
                splits += 1;
            }
        }
    }

    splits
}

pub fn part_2(input: &str) -> Answer {
    let mut lines = input.lines();
    let mut beams: Vec<_> = lines
        .next()
        .expect("input should not be empty")
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    while let Some(line) = lines.next() {
        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            }
        }
    }

    beams.iter().sum()
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    // println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
