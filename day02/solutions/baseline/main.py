#!/usr/bin/env python3
import argparse
import re
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def sum_invalid_range(left: int, right: int) -> int:
    total = 0
    pow10 = 1
    for _ in range(1, 20):
        pow10 *= 10
        factor = pow10 + 1
        lower_k = pow10 // 10
        min_n = lower_k * factor
        if min_n > right:
            break
        k_min = max(lower_k, (left + factor - 1) // factor if left else 0)
        k_max = min(pow10 - 1, right // factor)
        if k_min <= k_max:
            count = k_max - k_min + 1
            sum_k = (k_min + k_max) * count // 2
            total += factor * sum_k
    return total


def solve(text: str) -> int:
    nums = [int(x) for x in re.findall(r"\d+", text)]
    if len(nums) % 2 != 0:
        raise ValueError("input has odd number of bounds")
    total = 0
    for i in range(0, len(nums), 2):
        total += sum_invalid_range(nums[i], nums[i + 1])
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
