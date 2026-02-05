# Day 01: Secret Entrance (Parts 1 & 2)

## Summary

Part 1: track the dial position on a 0-99 ring starting at 50 and count how
many times it lands on 0 after a rotation.

Part 2: count every click that passes through 0 by computing the arithmetic
progression of click numbers congruent to the needed residue modulo 100.

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
./day01/solutions/primary/solve --part 2 --input day01/example.txt
```

### Baseline (Python)

Run:

```
python3 day01/solutions/baseline/main.py --part 1 --input day01/example.txt
python3 day01/solutions/baseline/main.py --part 2 --input day01/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day01/solutions/primary/solve --part 1 --input day01/example.txt" \
  --part2 "./day01/solutions/primary/solve --part 2 --input day01/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day01/solutions/baseline/main.py --part 1 --input day01/example.txt" \
  --part2 "python3 day01/solutions/baseline/main.py --part 2 --input day01/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.385 (best 1.057) | TBD | Byte parsing, modulo ring |
| Baseline | Python 3 | cpython | 18.894 (best 18.305) | TBD | Simple line parsing |
