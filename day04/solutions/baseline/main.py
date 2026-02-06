#!/usr/bin/env python3
import argparse
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def count_accessible(lines: list[str]) -> int:
    if not lines:
        return 0
    h = len(lines)
    w = len(lines[0])
    offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    total = 0
    for y in range(h):
        row = lines[y]
        for x in range(w):
            if row[x] != "@":
                continue
            neighbors = 0
            for dy, dx in offsets:
                ny = y + dy
                nx = x + dx
                if 0 <= ny < h and 0 <= nx < w:
                    if lines[ny][nx] == "@":
                        neighbors += 1
            if neighbors < 4:
                total += 1
    return total


def solve(text: str) -> int:
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    return count_accessible(lines)


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
