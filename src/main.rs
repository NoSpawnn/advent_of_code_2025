use std::fs;

mod day01;
mod day02;

fn main() {
    let input = fs::read_to_string("input/day02.in").unwrap();
    println!("{}", day02::part_1(&input));
    // println!("Day 01, Part 02: {}", day01::part_2(&input));
}
