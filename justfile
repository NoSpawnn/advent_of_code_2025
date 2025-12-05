_default:
    @just --list

new_day DAY:
    #!/usr/bin/env bash
    set -m

    DAY="$(printf '%d' '{{ DAY }}')"
    DAY_PRETTY="$(printf '%02d' '{{ DAY }}')"
    DIR="src/${DAY_PRETTY}"
    CARGO_TOML="Cargo.toml"

    mkdir -p ${DIR}/{bin,input}
    cat <<EOF > ${DIR}/bin/main.rs
    // https://adventofcode.com/2025/day/${DAY}

    pub fn part_1(input: &str) -> i32 {
        0
    }

    pub fn part_2(input: &str) -> i32 {
        0
    }

    fn main() {
        let input = include_str!("../input/input.example");
        // let input = include_str!("../input/real");
        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
    EOF

    curl -s https://adventofcode.com/2025/day/${DAY}/input > ${DIR}/input/real

    if ! grep -q "name = \"${DAY}\"" $CARGO_TOML; then
    cat <<EOF >> "$CARGO_TOML"

    [[bin]]
    name = "${DAY}"
    path = "${DIR}/main.rs"
    EOF
    fi

bench DAY:
    cargo b --bin {{ DAY }} --release && hyperfine ./target/release/{{ DAY }} --warmup 5

run DAY:
    cargo r --release --bin {{ DAY }}
