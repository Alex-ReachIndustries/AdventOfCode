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
    grid = [list(line.ljust(width, ".")) for line in lines]

    start_row = None
    start_col = None
    for r, row in enumerate(grid):
        for c, ch in enumerate(row):
            if ch == "S":
                start_row = r
                start_col = c
    if start_row is None or start_col is None:
        raise ValueError("start position S not found")

    active = [False] * width
    active[start_col] = True
    splits = 0
    for r in range(start_row + 1, len(grid)):
        next_active = [False] * width
        for c, is_active in enumerate(active):
            if not is_active:
                continue
            if grid[r][c] == "^":
                splits += 1
                if c > 0:
                    next_active[c - 1] = True
                if c + 1 < width:
                    next_active[c + 1] = True
            else:
                next_active[c] = True
        active = next_active
    return splits


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
