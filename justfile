_default:
    @just --list

new_day DAY:
    #!/usr/bin/env bash
    set -m

    DAY="$(printf '%02d' '{{ DAY }}')"
    DIR="src/${DAY}/bin"
    CARGO_TOML="Cargo.toml"

    mkdir -p ${DIR}
    cat <<EOF > ${DIR}/main.rs
    pub fn part_1(input: &str) -> i32 {
        0
    }

    pub fn part_2(input: &str) -> i32 {
        0
    }

    fn main() {
        let input = std::fs::read_to_string("input/${DAY}.example").unwrap();
        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
    EOF

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
