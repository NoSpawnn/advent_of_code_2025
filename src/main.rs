use std::fs;

mod day01;
mod day02;

fn main() {
    let input = fs::read_to_string("input/day02.in").unwrap();
    println!("{}", day02::part_1(&input));
    println!("{}", day02::part_2(&input));
}
