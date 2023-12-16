work day part:
    cargo watch --clear -w src/{{day}}/{{part}}.rs -s "cargo check --package advent_of_code_2023" -s "cargo clippy --package advent_of_code_2023" -s "cargo test {{day}} --package advent_of_code_2023"

worksrc day part:
    cargo watch --clear -w src/{{day}}/src/ -s "cargo check --package advent_of_code_2023" -s "cargo clippy --package advent_of_code_2023" -s "cargo test --bin advent_of_code_2023 {{day}}::src::{{part}}::tests"

benchsrc day:
    cargo bench --bin {{day}} --package advent_of_code_2023


bench-all:
    cargo bench -q > benchmarks.txt