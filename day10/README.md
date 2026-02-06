# Day 10: Factory (Part 1)

## Summary

Each machine is a subset-xor problem over GF(2): each button toggles a mask of
lights, and the goal is to reach the target pattern with minimum presses.
Meet-in-the-middle computes the minimum Hamming-weight subset.

Time complexity: O(2^(B/2)) per machine with B buttons.
Space complexity: O(2^(B/2)).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day10/solutions/primary/solve \
  day10/solutions/primary/main.rs
```

Run:

```
./day10/solutions/primary/solve --part 1 --input day10/example.txt
```

### Baseline (Python)

Run:

```
python3 day10/solutions/baseline/main.py --part 1 --input day10/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day10/solutions/primary/solve --part 1 --input day10/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day10/solutions/baseline/main.py --part 1 --input day10/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.208 (best 0.891) | n/a | MITM subset xor |
| Baseline | Python 3 | cpython | 18.647 (best 18.345) | n/a | MITM subset xor |
