use super::InputType;
use std::fmt::Write;

pub trait Solution {
    const YEAR: u16;
    const PROBLEM_NO: u16;
    const NAME: &str;

    fn part_1(input_type: &InputType) -> Option<Box<dyn super::Answer>>;
    fn part_2(input_type: &InputType) -> Option<Box<dyn super::Answer>>;

    fn read_input(input_type: &InputType) -> String {
        let filename = format!(
            "input/{}/day{:02}.{}",
            Self::YEAR,
            Self::PROBLEM_NO,
            match input_type {
                InputType::Example => "example",
                InputType::Real => "in",
            }
        );
        std::fs::read_to_string(filename).unwrap()
    }

    fn pretty_print_result(input_type: &InputType) -> Result<(), std::fmt::Error> {
        let mut output = String::new();
        let p1 = if let Some(ans) = Self::part_1(input_type) {
            format!("{ans}")
        } else {
            String::from("xxx")
        };
        let p2 = if let Some(ans) = Self::part_2(input_type) {
            format!("{ans}")
        } else {
            String::from("xxx")
        };
        let header = format!("----- {:02}. {} -----", Self::PROBLEM_NO, Self::NAME);

        writeln!(&mut output, "{header}")?;
        writeln!(&mut output, "      Part 1: {}", p1)?;
        writeln!(&mut output, "      Part 2: {}", p2)?;
        writeln!(
            &mut output,
            "{}",
            std::iter::repeat_n('-', header.len()).collect::<String>()
        )?;

        println!("{output}");

        Ok(())
    }
}
