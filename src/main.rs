use std::fs;

mod day01;

fn main() {
    let input = fs::read_to_string("input/day01.in").unwrap();
    let a = day01::part_2(&input);
    println!("Day 01, Part 01: {a}");
}
