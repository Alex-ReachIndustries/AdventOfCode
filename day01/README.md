# Day 01: Secret Entrance (Part 1)

## Summary

Track the dial position on a 0-99 ring starting at 50. For each rotation,
update the position with modulo arithmetic and count how many times it lands
on 0 after a move.

Time complexity: O(n) for n rotations.
Space complexity: O(1).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day01/solutions/primary/solve \
  day01/solutions/primary/main.rs
```

Run:

```
./day01/solutions/primary/solve --part 1 --input day01/example.txt
```

### Baseline (Python)

Run:

```
python3 day01/solutions/baseline/main.py --part 1 --input day01/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day01/solutions/primary/solve --part 1 --input day01/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day01/solutions/baseline/main.py --part 1 --input day01/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.385 (best 1.057) | n/a | Byte parsing, modulo ring |
| Baseline | Python 3 | cpython | 18.894 (best 18.305) | n/a | Simple line parsing |
