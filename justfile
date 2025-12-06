_default:
    @just --list

new_day DAY:
    #!/usr/bin/env bash
    set -m

    source .env

    DAY="$(printf '%d' '{{ DAY }}')"
    DAY_PRETTY="$(printf '%02d' '{{ DAY }}')"
    DIR="src/${DAY_PRETTY}"
    CARGO_TOML="Cargo.toml"

    cat <<EOF > ${DIR}/main.rs
    // https://adventofcode.com/2025/day/${DAY}

    type Answer = i32;

    pub fn part_1(input: &str) -> Answer {
        todo!("day ${DAY} part 1")
    }

    pub fn part_2(input: &str) -> Answer {
        todo!("day ${DAY} part 2")
    }

    fn main() {
        let input = include_str!("input/input.example");
        // let input = include_str!("input/real");
        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
    EOF

    mkdir -p ${DIR}/input
    curl -s -H "Cookie: session=${AOC_SESSION_COOKIE}" https://adventofcode.com/2025/day/${DAY}/input > ${DIR}/input/real

    if ! grep -q "name = \"${DAY}\"" $CARGO_TOML; then
    cat <<EOF >> "$CARGO_TOML"

    [[bin]]
    name = "${DAY_PRETTY}"
    path = "${DIR}/main.rs"
    EOF
    fi

bench DAY:
    #!/usr/bin/env bash

    DAY="$(printf '%02d' '{{ DAY }}')"

    cargo b --bin ${DAY} --release
    hyperfine ./target/release/${DAY} --warmup 5

run DAY:
    #!/usr/bin/env bash

    DAY="$(printf '%02d' '{{ DAY }}')"

    cargo r --bin ${DAY}
