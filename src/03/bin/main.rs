// https://adventofcode.com/2025/day/3

// each line of digits in your input corresponds to a single bank of batteries.
// Within each bank, you need to turn on exactly two batteries

#[derive(Debug)]
struct MaxValue {
    index: Option<usize>,
    value: Option<u32>,
}

pub fn part_1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut m1: Option<u32> = None;
        let mut m2: Option<u32> = None;

        let chars: Vec<_> = line.chars().collect();
        let mut ptr = 0usize;
        for i in 0..chars.len() {
            let n = chars[i].to_digit(10).unwrap();
            if Some(n) > m1 {
                m1 = Some(n);
                ptr = i;
            }
        }

        // If the largest number is at the end of the string,
        // we wont use it as the x10
        if ptr == chars.len() - 1 {
            m2 = m1;
            m1 = None;
            for i in 0..chars.len() - 1 {
                let n = chars[i].to_digit(10).unwrap();
                if Some(n) > m1 {
                    m1 = Some(n);
                }
            }
        } else {
            for i in ptr + 1..chars.len() {
                let n = chars[i].to_digit(10).unwrap();
                if Some(n) > m2 {
                    m2 = Some(n);
                }
            }
        }

        sum += m1.unwrap() * 10 + m2.unwrap()
    }

    sum
}

pub fn part_2(input: &str) -> i32 {
    0
}

fn main() {
    // let input = std::fs::read_to_string("input/03.example").unwrap();
    let input = std::fs::read_to_string("input/03.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
