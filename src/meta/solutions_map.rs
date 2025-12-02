use std::collections::HashMap;

use super::{Answer, InputType, Solution};

pub struct SolutionEntry {
    pub part_1: fn(&InputType) -> Option<Box<dyn Answer>>,
    pub part_2: fn(&InputType) -> Option<Box<dyn Answer>>,
    pub pretty_print_result: fn(input_type: &InputType) -> Result<(), std::fmt::Error>,
}

pub static SOLUTIONS: std::sync::LazyLock<HashMap<String, HashMap<String, SolutionEntry>>> =
    std::sync::LazyLock::new(|| {
        HashMap::from([(
            String::from("2025"),
            HashMap::from([
                (
                    String::from("1"),
                    SolutionEntry {
                        part_1: crate::y2025::Day01::part_1,
                        part_2: crate::y2025::Day01::part_2,
                        pretty_print_result: crate::y2025::Day01::pretty_print_result,
                    },
                ),
                (
                    String::from("2"),
                    SolutionEntry {
                        part_1: crate::y2025::Day02::part_1,
                        part_2: crate::y2025::Day02::part_2,
                        pretty_print_result: crate::y2025::Day02::pretty_print_result,
                    },
                ),
            ]),
        )])
    });
