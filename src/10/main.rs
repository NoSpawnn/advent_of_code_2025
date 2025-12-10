// https://adventofcode.com/2025/day/10

use std::collections::HashSet;

type Answer = i32;

#[derive(Debug, Clone)]
struct Button {
    toggles: Vec<usize>,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let value = String::from(value);
        let value = value.replace('(', "");
        let value = value.replace(')', "");
        let toggles: Vec<_> = value.split(',').map(|s| s.parse().unwrap()).collect();
        Self { toggles }
    }
}

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Button>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let buttons_start = value
            .find('(')
            .expect("input line should be of the form `[] ()...() {}`");
        let joltages_start = value
            .find('{')
            .expect("input line should be of the form `[] ()...() {}`");

        let chars: Vec<_> = value.chars().collect();
        let lights: Vec<_> = chars[..buttons_start - 1]
            .iter()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect();
        let buttons: Vec<_> = chars[buttons_start..joltages_start - 1]
            .split(|c| c.is_whitespace())
            .map(|s| s.iter().collect::<String>())
            .map(|s| Button::from(s.as_str()))
            .collect();

        Self {
            target: lights,
            buttons,
        }
    }
}

fn push_button(mut state: Vec<bool>, button: &Button) -> Vec<bool> {
    for b in &button.toggles {
        state[*b] = !state[*b];
    }
    state
}

pub fn part_1(input: &str) -> Answer {
    let machines: Vec<Machine> = input.lines().map(Machine::from).collect();
    machines
        .iter()
        .map(|machine| {
            // initial statess is a single state with all lights off
            let mut states = HashSet::from([vec![false; machine.target.len()]]);
            // BFS: find the length of the first sequence of button presses
            // that results in the desired state (minimium 1)
            (1..)
                .find(|_| {
                    states = states
                        .iter()
                        .flat_map(|state| {
                            machine
                                .buttons
                                .iter()
                                .map(move |button| push_button(state.clone(), button))
                        })
                        .collect();
                    states.contains(&machine.target)
                })
                .expect("we should never not find a valid sequence of presses")
        })
        .sum()
}

pub fn part_2(input: &str) -> Answer {
    todo!("day 10 part 2")
}

fn main() {
    let input = include_str!("input/example");
    let input = include_str!("input/real");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
