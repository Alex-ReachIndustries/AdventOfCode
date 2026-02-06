#!/usr/bin/env python3
import argparse
import re
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def digits(value: int) -> int:
    if value == 0:
        return 1
    count = 0
    while value:
        count += 1
        value //= 10
    return count


def sum_invalid_range_repeat(
    left: int, right: int, pow10: list[int], max_digits: int, max_k: int | None
) -> int:
    total = 0
    max_d = max_digits // 2
    if max_d == 0:
        return 0
    for d in range(1, max_d + 1):
        pow10_d = pow10[d]
        lower_k = pow10[d - 1]
        upper_k = pow10_d - 1
        rep = 1
        k_limit = max_k if max_k is not None else max_digits // d
        for _ in range(2, k_limit + 1):
            rep = rep * pow10_d + 1
            min_n = lower_k * rep
            if min_n > right:
                break
            k_min = max(lower_k, (left + rep - 1) // rep if left else 0)
            k_max = min(upper_k, right // rep)
            if k_min <= k_max:
                count = k_max - k_min + 1
                sum_k = (k_min + k_max) * count // 2
                total += rep * sum_k
    return total


def sum_invalid_range(left: int, right: int, pow10: list[int], max_digits: int) -> int:
    return sum_invalid_range_repeat(left, right, pow10, max_digits, 2)


def solve(text: str, part: int) -> int:
    nums = [int(x) for x in re.findall(r"\d+", text)]
    if len(nums) % 2 != 0:
        raise ValueError("input has odd number of bounds")
    max_r = 0
    for i in range(1, len(nums), 2):
        if nums[i] > max_r:
            max_r = nums[i]
    max_digits = digits(max_r)
    max_d = max_digits // 2
    pow10 = [1]
    for _ in range(max_d):
        pow10.append(pow10[-1] * 10)
    total = 0
    for i in range(0, len(nums), 2):
        if part == 1:
            total += sum_invalid_range(nums[i], nums[i + 1], pow10, max_digits)
        else:
            total += sum_invalid_range_repeat(nums[i], nums[i + 1], pow10, max_digits, None)
    return total


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--part", type=int, default=1)
    parser.add_argument("--input", dest="input_path")
    args = parser.parse_args()

    if args.part not in {1, 2}:
        print(f"unsupported part: {args.part}", file=sys.stderr)
        raise SystemExit(2)

    text = read_input(args.input_path)
    answer = solve(text, args.part)
    print(answer)


if __name__ == "__main__":
    main()
