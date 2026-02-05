#!/usr/bin/env python3
import argparse
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def count_zero_positions(text: str) -> int:
    pos = 50
    count = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        direction = line[0]
        value = int(line[1:])
        if direction == "L":
            pos = (pos - value) % 100
        elif direction == "R":
            pos = (pos + value) % 100
        else:
            continue
        if pos == 0:
            count += 1
    return count


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--part", type=int, default=1)
    parser.add_argument("--input", dest="input_path")
    args = parser.parse_args()

    if args.part != 1:
        print("part 2 not implemented yet", file=sys.stderr)
        raise SystemExit(2)

    text = read_input(args.input_path)
    answer = count_zero_positions(text)
    print(answer)


if __name__ == "__main__":
    main()
