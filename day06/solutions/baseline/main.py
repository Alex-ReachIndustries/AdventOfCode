#!/usr/bin/env python3
import argparse
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def trim_empty_edges(lines: list[str]) -> list[str]:
    while lines and not lines[0].strip():
        lines.pop(0)
    while lines and not lines[-1].strip():
        lines.pop()
    return lines


def solve(text: str) -> int:
    lines = trim_empty_edges(text.splitlines())
    if not lines:
        return 0
    width = max(len(line) for line in lines)
    grid = [line.ljust(width) for line in lines]

    segments: list[tuple[int, int]] = []
    in_segment = False
    start = 0
    for col in range(width):
        all_space = True
        for row in grid:
            if row[col] != " ":
                all_space = False
                break
        if not all_space:
            if not in_segment:
                start = col
                in_segment = True
        elif in_segment:
            segments.append((start, col - 1))
            in_segment = False
    if in_segment:
        segments.append((start, width - 1))

    total = 0
    op_row = grid[-1]
    for start_col, end_col in segments:
        op = next((op_row[c] for c in range(start_col, end_col + 1) if op_row[c] != " "), " ")
        if op not in {"+", "*"}:
            continue
        values = []
        for row in grid[:-1]:
            digits = [ch for ch in row[start_col : end_col + 1] if ch.isdigit()]
            if digits:
                values.append(int("".join(digits)))
        if not values:
            continue
        if op == "+":
            result = sum(values)
        else:
            result = 1
            for value in values:
                result *= value
        total += result
    return total


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--part", type=int, default=1)
    parser.add_argument("--input", dest="input_path")
    args = parser.parse_args()

    if args.part != 1:
        print("part 2 not implemented yet", file=sys.stderr)
        raise SystemExit(2)

    text = read_input(args.input_path)
    answer = solve(text)
    print(answer)


if __name__ == "__main__":
    main()
