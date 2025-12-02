use std::fs;

mod day01;
mod day02;

fn main() {
    let input = fs::read_to_string("input/day02.in").unwrap();
    println!("Part 1: {}", day02::part_1(&input));
    println!("Part 2: {}", day02::part_2(&input));
}
