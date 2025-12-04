const ROLL: char = '@';

fn count_adjacent(grid: &[String], row: usize, col: usize) -> i32 {
    let r_range: Vec<_> = {
        if row == 0 {
            row..=row + 1
        } else if row == grid.len() - 1 {
            row - 1..=row
        } else {
            row - 1..=row + 1
        }
    }
    .collect();
    let c_range: Vec<_> = {
        if col == 0 {
            col..=col + 1
        } else if col == grid[0].len() - 1 {
            col - 1..=col
        } else {
            col - 1..=col + 1
        }
    }
    .collect();

    let mut adjacent = 0;
    for r_check in &r_range {
        for c_check in &c_range {
            if *r_check == row && *c_check == col {
                continue;
            }
            if grid[*r_check].chars().nth(*c_check) == Some(ROLL) {
                adjacent += 1;
            }
        }
    }

    adjacent
}

pub fn part_1(input: &str) -> i32 {
    let grid: Vec<String> = input.lines().map(str::to_owned).collect();
    let mut accessible = 0;

    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, _) in row.char_indices().filter(|(_, chr)| *chr == ROLL) {
            if count_adjacent(&grid, r_idx, c_idx) < 4 {
                accessible += 1
            }
        }
    }

    accessible
}

pub fn part_2(input: &str) -> i32 {
    let mut grid: Vec<String> = input.lines().map(String::from).collect();
    let mut res = 0;

    loop {
        let mut removable = Vec::new();
        for (r_idx, row) in grid.iter().enumerate() {
            for (c_idx, _) in row.char_indices().filter(|(_, chr)| *chr == ROLL) {
                if count_adjacent(&grid, r_idx, c_idx) < 4 {
                    removable.push((r_idx, c_idx));
                }
            }
        }

        if removable.is_empty() {
            break;
        }

        res += removable.len() as i32;
        for (r_remove, c_remove) in removable {
            let mut new: Vec<char> = grid[r_remove].chars().collect();
            new[c_remove] = '.';
            grid[r_remove] = new.iter().collect();
        }
    }

    res
}

fn main() {
    // let input = std::fs::read_to_string("input/04.example").unwrap();
    let input = std::fs::read_to_string("input/04.in").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
