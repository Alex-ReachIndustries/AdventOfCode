#!/usr/bin/env python3
import argparse
import heapq
import re
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


class DSU:
    def __init__(self, n: int) -> None:
        self.parent = list(range(n))
        self.size = [1] * n

    def find(self, x: int) -> int:
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    def union(self, a: int, b: int) -> None:
        ra = self.find(a)
        rb = self.find(b)
        if ra == rb:
            return
        if self.size[ra] < self.size[rb]:
            ra, rb = rb, ra
        self.parent[rb] = ra
        self.size[ra] += self.size[rb]


def solve(text: str) -> int:
    nums = [int(x) for x in re.findall(r"-?\d+", text)]
    if len(nums) % 3 != 0:
        raise ValueError("input does not contain xyz triples")
    points = [(nums[i], nums[i + 1], nums[i + 2]) for i in range(0, len(nums), 3)]
    n = len(points)
    if n < 2:
        return 0

    total_pairs = n * (n - 1) // 2
    k = n // 2
    if k > 1000:
        k = 1000
    if k > total_pairs:
        k = total_pairs
    heap: list[tuple[int, int, int]] = []

    for i in range(n):
        xi, yi, zi = points[i]
        for j in range(i + 1, n):
            xj, yj, zj = points[j]
            dx = xi - xj
            dy = yi - yj
            dz = zi - zj
            dist = dx * dx + dy * dy + dz * dz
            item = (-dist, -i, -j)
            if len(heap) < k:
                heapq.heappush(heap, item)
            else:
                if item > heap[0]:
                    heapq.heapreplace(heap, item)

    edges = [(-d, -i, -j) for (d, i, j) in heap]
    edges.sort()

    dsu = DSU(n)
    for _, i, j in edges:
        dsu.union(i, j)

    counts = [0] * n
    for i in range(n):
        counts[dsu.find(i)] += 1
    sizes = sorted((c for c in counts if c > 0), reverse=True)
    product = 1
    for size in sizes[:3]:
        product *= size
    return product


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
