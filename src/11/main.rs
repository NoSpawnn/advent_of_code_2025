// https://adventofcode.com/2025/day/11

use std::{collections::HashMap, hash::Hash};

type Answer = u64;

#[derive(Debug, PartialEq, Eq)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Hash for Device {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Device {
    fn routes_to<'a>(
        &'a self,
        to: &'a Device,
        others: &'a [Device],
        cache: &mut HashMap<(&'a Device, &'a Device), u64>,
    ) -> u64 {
        if let Some(&n) = cache.get(&(self, to)) {
            n
        } else if self.outputs.iter().any(|it| it.as_str() == to.name) {
            1
        } else {
            let res = self
                .outputs
                .iter()
                .filter_map(|other_name| others.iter().find(|d| d.name == *other_name))
                .map(|neighbour| neighbour.routes_to(to, others, cache))
                .sum();
            cache.insert((self, to), res);
            res
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

#[inline]
fn find_device<'a>(target: &str, devices: &'a [Device]) -> &'a Device {
    devices
        .iter()
        .find(|d| d.name == target)
        .unwrap_or_else(|| panic!("there should be a start position labelled '{}'", target))
}

pub fn part_1(input: &str) -> Answer {
    let devices: Vec<_> = input.lines().map(Device::from).collect();
    find_device("you", &devices).routes_to(
        &Device {
            name: String::from("out"),
            outputs: vec![],
        },
        &devices,
        &mut HashMap::new(),
    )
}

pub fn part_2(input: &str) -> Answer {
    let devices: Vec<_> = input.lines().map(Device::from).collect();
    let svr = find_device("svr", &devices);
    let dac = find_device("dac", &devices);
    let fft = find_device("fft", &devices);
    let out = Device {
        name: String::from("out"),
        outputs: vec![],
    };
    let mut cache = HashMap::new();
    let svr_to_dac = svr.routes_to(dac, &devices, &mut cache);
    let dac_to_fft = dac.routes_to(fft, &devices, &mut cache);
    let fft_to_out = fft.routes_to(&out, &devices, &mut cache);
    let svr_to_fft = svr.routes_to(fft, &devices, &mut cache);
    let fft_to_dac = fft.routes_to(dac, &devices, &mut cache);
    let dac_to_out = dac.routes_to(&out, &devices, &mut cache);
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
