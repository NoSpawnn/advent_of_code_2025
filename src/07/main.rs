// https://adventofcode.com/2025/day/7

type Answer = usize;

fn init_beams(line: &str) -> Vec<usize> {
    line.chars().map(|c| if c == 'S' { 1 } else { 0 }).collect()
}

fn do_line_splits(line: &str, beams: &mut Vec<usize>) -> usize {
    let mut splits = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '^' && beams[i] >= 1 {
            beams[i - 1] += beams[i];
            beams[i + 1] += beams[i];
            beams[i] = 0;
            splits += 1;
        }
    }
    splits
}

pub fn part_1(input: &str) -> Answer {
    let mut lines = input.lines();
    let mut splits = 0;
    let mut beams = init_beams(&lines.next().unwrap());

    while let Some(line) = lines.next() {
        splits += do_line_splits(line, &mut beams);
    }

    splits
}

pub fn part_2(input: &str) -> Answer {
    let mut lines = input.lines();
    let mut beams = init_beams(&lines.next().unwrap());

    while let Some(line) = lines.next() {
        let _ = do_line_splits(line, &mut beams);
    }

    beams.iter().sum()
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
