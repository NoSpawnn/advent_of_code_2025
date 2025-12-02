mod answer;
mod solution;
mod solutions_map;

pub use answer::Answer;
pub use solution::Solution;
pub use solutions_map::SOLUTIONS;

pub enum InputType {
    Example,
    Real,
}

impl From<&str> for InputType {
    fn from(value: &str) -> Self {
        match value {
            "r" | "real" => Self::Real,
            _ => Self::Example,
        }
    }
}
