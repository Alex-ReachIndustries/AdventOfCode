#!/usr/bin/env python3
import argparse
import bisect
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def merge_ranges(ranges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    if not ranges:
        return []
    ranges.sort()
    merged = [ranges[0]]
    for start, end in ranges[1:]:
        last_start, last_end = merged[-1]
        if start <= last_end + 1:
            if end > last_end:
                merged[-1] = (last_start, end)
        else:
            merged.append((start, end))
    return merged


def solve(text: str) -> int:
    text = text.replace("\r\n", "\n")
    sections = text.split("\n\n")
    ranges_section = sections[0] if sections else ""
    ids_section = sections[1] if len(sections) > 1 else ""

    ranges: list[tuple[int, int]] = []
    for line in ranges_section.splitlines():
        line = line.strip()
        if not line:
            continue
        start_s, end_s = line.split("-", 1)
        ranges.append((int(start_s), int(end_s)))

    merged = merge_ranges(ranges)
    starts = [start for start, _ in merged]

    count = 0
    for line in ids_section.splitlines():
        line = line.strip()
        if not line:
            continue
        value = int(line)
        idx = bisect.bisect_right(starts, value) - 1
        if idx >= 0 and merged[idx][0] <= value <= merged[idx][1]:
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
    answer = solve(text)
    print(answer)


if __name__ == "__main__":
    main()
