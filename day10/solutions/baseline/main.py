#!/usr/bin/env python3
import argparse
import re
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def parse_line(line: str) -> tuple[int, list[int]]:
    match = re.search(r"\[([.#]+)\]", line)
    if not match:
        raise ValueError("no indicator pattern found")
    pattern = match.group(1)
    if len(pattern) > 128:
        raise ValueError("too many indicator lights")
    target = 0
    for idx, ch in enumerate(pattern):
        if ch == "#":
            target |= 1 << idx

    buttons: list[int] = []
    for group in re.findall(r"\(([^)]*)\)", line):
        mask = 0
        if group.strip():
            for part in group.split(","):
                idx = int(part)
                if idx >= 128:
                    raise ValueError("toggle index too large")
                mask |= 1 << idx
        buttons.append(mask)
    return target, buttons


def min_presses(target: int, buttons: list[int]) -> int:
    n = len(buttons)
    if n == 0:
        return 0 if target == 0 else 1 << 30
    n1 = n // 2
    left = buttons[:n1]
    right = buttons[n1:]

    left_size = 1 << n1
    left_xor = [0] * left_size
    left_cnt = [0] * left_size
    for i in range(1, left_size):
        lsb = i & -i
        bit = lsb.bit_length() - 1
        prev = i ^ lsb
        left_xor[i] = left_xor[prev] ^ left[bit]
        left_cnt[i] = left_cnt[prev] + 1

    left_best: dict[int, int] = {}
    for i in range(left_size):
        val = left_xor[i]
        cnt = left_cnt[i]
        if val not in left_best or cnt < left_best[val]:
            left_best[val] = cnt

    right_size = 1 << len(right)
    right_xor = [0] * right_size
    right_cnt = [0] * right_size
    for i in range(1, right_size):
        lsb = i & -i
        bit = lsb.bit_length() - 1
        prev = i ^ lsb
        right_xor[i] = right_xor[prev] ^ right[bit]
        right_cnt[i] = right_cnt[prev] + 1

    best = 1 << 30
    for i in range(right_size):
        val = right_xor[i]
        cnt = right_cnt[i]
        needed = target ^ val
        if needed in left_best:
            total = cnt + left_best[needed]
            if total < best:
                best = total
    return best


def solve(text: str) -> int:
    total = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        target, buttons = parse_line(line)
        presses = min_presses(target, buttons)
        if presses >= 1 << 29:
            raise ValueError(f"no solution for line: {line}")
        total += presses
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
