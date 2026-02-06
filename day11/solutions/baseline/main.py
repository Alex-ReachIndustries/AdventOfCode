#!/usr/bin/env python3
import argparse
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def solve(text: str) -> int:
    adj: dict[str, list[str]] = {}
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        name, _, rest = line.partition(":")
        outputs = rest.strip().split() if rest.strip() else []
        adj[name.strip()] = outputs

    if "you" not in adj:
        return 0

    memo: dict[str, int] = {}
    visiting: set[str] = set()

    def dfs(node: str) -> int:
        if node == "out":
            return 1
        if node in memo:
            return memo[node]
        if node in visiting:
            raise ValueError("cycle detected in graph")
        visiting.add(node)
        total = 0
        for child in adj.get(node, []):
            total += dfs(child)
        visiting.remove(node)
        memo[node] = total
        return total

    return dfs("you")


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
