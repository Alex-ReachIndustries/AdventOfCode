#!/usr/bin/env python3
import argparse
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def max_joltage(line: str) -> int:
    if len(line) < 2:
        return 0
    max_right = int(line[-1])
    best = 0
    for ch in reversed(line[:-1]):
        d = int(ch)
        value = d * 10 + max_right
        if value > best:
            best = value
        if d > max_right:
            max_right = d
    return best


def solve(text: str) -> int:
    total = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        total += max_joltage(line)
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
