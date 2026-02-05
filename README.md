# Advent of Code 2025 (Days 1-12) - Performance Benchmarks

This repo contains fast, correctness-first solutions for Advent of Code 2025
days 1-12. Each day includes a fastest practical implementation (primary) and
at least one baseline implementation for comparison, with per-day benchmarks.

Notes:
- Puzzle statements and personal inputs are not included. Provide your own
  input via --input <path> or stdin.
- Each day is solved without hardcoding or special-casing the example.
- Current scope is part 1 only; part 2 will be added later.

## Layout

- dayXX/
  - README.md (algorithm notes + benchmark table)
  - example.txt (from the puzzle statement)
  - solutions/
    - primary/ (fastest practical implementation)
    - baseline/ (comparison implementation)
- tools/bench.py (benchmark runner)
- TODO.md (living task list)

## CLI Contract

All implementations must support:

- --part 1|2
- --input <path> (optional; reads stdin if omitted)

They print only the answer for the requested part. If part 2 is requested
before it is implemented, the program exits with a non-zero status and prints
an error to stderr.

## Benchmarking

Use the provided benchmark runner:

  python tools/bench.py --part1 "<cmd>" --part2 "<cmd>" --runs 10 --warmup 2

If hyperfine is installed, the runner uses it by default. Otherwise it falls
back to a simple repeated timer. Each day README records the measured results
and the exact command used. For part 1-only days, the part 2 column is marked
as not implemented.

## Days Overview

| Day | Primary (fastest) | Baseline | Status | Link |
| --- | --- | --- | --- | --- |
| 01 | Rust | Python 3 | part1 complete | day01/README.md |
| 02 | Rust | Python 3 | part1 complete | day02/README.md |
| 03 | Rust | Python 3 | part1 complete | day03/README.md |
| 04 | Rust | Python 3 | part1 complete | day04/README.md |
| 05 | Rust | Python 3 | part1 complete | day05/README.md |
| 06 | Rust | Python 3 | part1 complete | day06/README.md |
| 07 | Rust | Python 3 | part1 complete | day07/README.md |
| 08 | Rust | Python 3 | part1 complete | day08/README.md |
| 09 | Rust | Python 3 | part1 complete | day09/README.md |
| 10 | Rust | Python 3 | part1 complete | day10/README.md |
| 11 | Rust | Python 3 | part1 complete | day11/README.md |
| 12 | TBD | TBD | pending | day12/README.md |