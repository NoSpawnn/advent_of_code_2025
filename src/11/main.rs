// https://adventofcode.com/2025/day/11

use std::collections::HashSet;

type Answer = i32;

#[derive(Debug)]
struct Device {
    name: String,
    outputs: HashSet<String>,
}

impl Device {
    fn routes_to_out(&self, others: &[Device]) -> i32 {
        if self.outputs.contains("out") {
            1
        } else {
            self.outputs
                .iter()
                .filter_map(|other_name| others.iter().find(|d| d.name == *other_name))
                .map(|neighbour| neighbour.routes_to_out(others))
                .sum()
        }
    }
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let (name, outputs) = value.split_once(':').unwrap();
        let outputs = outputs.split_whitespace().map(String::from);
        Self {
            name: String::from(name),
            outputs: outputs.collect(),
        }
    }
}

pub fn part_1(input: &str) -> Answer {
    let devices: Vec<_> = input.lines().map(Device::from).collect();
    devices
        .iter()
        .find(|d| d.name == "you")
        .expect("there should be a start position labelled 'you'")
        .routes_to_out(&devices)
}

pub fn part_2(input: &str) -> Answer {
    todo!("day 11 part 2")
}

fn main() {
    // let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
