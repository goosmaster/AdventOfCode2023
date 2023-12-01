work day:
    cargo watch --clear -w src/{{day}} -x check -x test -x clippy
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}} {{part}} >> {{day}}.bench.txt