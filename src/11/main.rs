// https://adventofcode.com/2025/day/11

use std::collections::HashMap;

type Answer = u64;

fn paths(
    start: &str,
    end: &str,
    devices: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, String), u64>,
) -> u64 {
    if let Some(&n) = cache.get(&(start.into(), end.into())) {
        n
    } else if start == end {
        1
    } else if start == "out" {
        0
    } else {
        let res = devices[start]
            .iter()
            .map(|neighbour| paths(neighbour, end, devices, cache))
            .sum();
        cache.insert((start.into(), end.into()), res);
        res
    }
}

pub fn part_1(input: &str) -> Answer {
    let devices: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| {
            let (name, outputs) = line.split_once(':').unwrap();
            let outputs = outputs.split_whitespace().map(String::from);
            (String::from(name), outputs.collect())
        })
        .collect();
    paths("you", "out", &devices, &mut HashMap::new())
}

pub fn part_2(input: &str) -> Answer {
    let devices: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| {
            let (name, outputs) = line.split_once(':').unwrap();
            let outputs = outputs.split_whitespace().map(String::from);
            (String::from(name), outputs.collect())
        })
        .collect();
    let mut cache = HashMap::new();
    let svr_to_dac = paths("svr", "dac", &devices, &mut cache);
    let dac_to_fft = paths("dac", "fft", &devices, &mut cache);
    let fft_to_out = paths("fft", "out", &devices, &mut cache);
    let svr_to_fft = paths("svr", "fft", &devices, &mut cache);
    let fft_to_dac = paths("fft", "dac", &devices, &mut cache);
    let dac_to_out = paths("dac", "out", &devices, &mut cache);
    svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
}

fn main() {
    // let input_1 = include_str!("input/example.1");
    // let input_2 = include_str!("input/example.2");
    let input_1 = include_str!("input/real");
    let input_2 = include_str!("input/real");
    println!("Part 1: {}", part_1(&input_1));
    println!("Part 2: {}", part_2(&input_2));
}
