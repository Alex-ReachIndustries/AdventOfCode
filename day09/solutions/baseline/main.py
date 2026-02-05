#!/usr/bin/env python3
import argparse
import re
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def solve(text: str) -> int:
    nums = [int(x) for x in re.findall(r"-?\d+", text)]
    if len(nums) % 2 != 0:
        raise ValueError("input does not contain xy pairs")
    points = [(nums[i], nums[i + 1]) for i in range(0, len(nums), 2)]
    best = 0
    for i, (xi, yi) in enumerate(points):
        for xj, yj in points[i + 1 :]:
            dx = abs(xi - xj)
            dy = abs(yi - yj)
            area = (dx + 1) * (dy + 1)
            if area > best:
                best = area
    return best


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
