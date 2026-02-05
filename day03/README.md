# Day 03: Lobby (Part 1)

## Summary

For each line, choose two digits in order to maximize the two-digit number.
Scan from right to left maintaining the maximum digit to the right; for each
position compute 10 * digit + max_right and keep the maximum.

Time complexity: O(n) per line.
Space complexity: O(1).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day03/solutions/primary/solve \
  day03/solutions/primary/main.rs
```

Run:

```
./day03/solutions/primary/solve --part 1 --input day03/example.txt
```

### Baseline (Python)

Run:

```
python3 day03/solutions/baseline/main.py --part 1 --input day03/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day03/solutions/primary/solve --part 1 --input day03/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day03/solutions/baseline/main.py --part 1 --input day03/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.194 (best 0.897) | n/a | Right-to-left max digit |
| Baseline | Python 3 | cpython | 19.949 (best 18.110) | n/a | Reverse scan per line |
