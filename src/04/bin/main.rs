const ROLL: char = '@';

pub fn part_1(input: &str) -> i32 {
    let grid: Vec<&str> = input.lines().collect();
    let mut accessible = 0;

    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, _) in row.char_indices().filter(|(_, chr)| *chr == ROLL) {
            let r_range: Vec<_> = {
                if r_idx == 0 {
                    r_idx..=r_idx + 1
                } else if r_idx == grid.len() - 1 {
                    r_idx - 1..=r_idx
                } else {
                    r_idx - 1..=r_idx + 1
                }
            }
            .collect();
            let c_range: Vec<_> = {
                if c_idx == 0 {
                    c_idx..=c_idx + 1
                } else if c_idx == grid[0].len() - 1 {
                    c_idx - 1..=c_idx
                } else {
                    c_idx - 1..=c_idx + 1
                }
            }
            .collect();
            let mut adjacent = 0;

            for r_check in &r_range {
                for c_check in &c_range {
                    if *r_check == r_idx && *c_check == c_idx {
                        continue;
                    }
                    if grid[*r_check].chars().nth(*c_check) == Some(ROLL) {
                        adjacent += 1;
                    }
                }
            }

            if adjacent < 4 {
                accessible += 1
            }
        }
    }

    accessible
}

pub fn part_2(input: &str) -> i32 {
    0
}

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
