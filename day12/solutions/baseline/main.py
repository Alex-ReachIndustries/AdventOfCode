#!/usr/bin/env python3
import argparse
import sys


def read_input(path: str | None) -> str:
    if path:
        with open(path, "r", encoding="utf-8") as handle:
            return handle.read()
    return sys.stdin.read()


def normalize(coords: list[tuple[int, int]]) -> list[tuple[int, int]]:
    min_x = min(x for x, _ in coords)
    min_y = min(y for _, y in coords)
    norm = sorted((x - min_x, y - min_y) for x, y in coords)
    return norm


def generate_orientations(coords: list[tuple[int, int]]) -> list[list[tuple[int, int]]]:
    seen = set()
    out = []
    for flip in (False, True):
        for rot in range(4):
            transformed = []
            for x, y in coords:
                nx, ny = x, y
                if flip:
                    nx = -nx
                for _ in range(rot):
                    nx, ny = ny, -nx
                transformed.append((nx, ny))
            norm = tuple(normalize(transformed))
            if norm not in seen:
                seen.add(norm)
                out.append(list(norm))
    return out


def parse_input(text: str) -> tuple[list[dict], list[tuple[int, int, list[int]]]]:
    shapes: dict[int, list[str]] = {}
    regions: list[tuple[int, int, list[int]]] = []
    current_idx: int | None = None
    for raw in text.splitlines():
        line = raw.strip()
        if not line:
            continue
        if ":" in line:
            left, rest = line.split(":", 1)
            if "x" in left:
                w_str, h_str = left.split("x", 1)
                width = int(w_str)
                height = int(h_str)
                counts = [int(x) for x in rest.strip().split()] if rest.strip() else []
                regions.append((width, height, counts))
                current_idx = None
            else:
                idx = int(left)
                current_idx = idx
                shapes.setdefault(idx, [])
        else:
            if current_idx is not None:
                shapes.setdefault(current_idx, []).append(line)

    shape_list: list[dict] = []
    for idx in sorted(shapes):
        rows = shapes[idx]
        coords = []
        for y, row in enumerate(rows):
            for x, ch in enumerate(row):
                if ch == "#":
                    coords.append((x, y))
        orientations = generate_orientations(coords)
        shape_list.append({"area": len(coords), "orientations": orientations})
    return shape_list, regions


def make_mask(cells: list[tuple[int, int]], width: int, ox: int, oy: int) -> int:
    mask = 0
    for x, y in cells:
        idx = (oy + y) * width + (ox + x)
        mask |= 1 << idx
    return mask


def search(shapes: list[dict], board: int) -> bool:
    best_idx = None
    best_count = 10**9
    for i, shape in enumerate(shapes):
        if shape["count"] == 0:
            continue
        if not shape["placements"]:
            return False
        if len(shape["placements"]) < best_count:
            best_count = len(shape["placements"])
            best_idx = i
    if best_idx is None:
        return True

    shape = shapes[best_idx]
    for mask in shape["placements"]:
        if board & mask:
            continue
        shape["count"] -= 1
        if search(shapes, board | mask):
            return True
        shape["count"] += 1
    return False


def solve(text: str) -> int:
    shapes, regions = parse_input(text)
    fit_count = 0
    for width, height, counts in regions:
        total_area = 0
        instances = []
        for idx, shape in enumerate(shapes):
            count = counts[idx] if idx < len(counts) else 0
            total_area += count * shape["area"]
            placements = []
            for orient in shape["orientations"]:
                max_x = max(x for x, _ in orient)
                max_y = max(y for _, y in orient)
                if max_x + 1 > width or max_y + 1 > height:
                    continue
                for oy in range(height - max_y):
                    for ox in range(width - max_x):
                        placements.append(make_mask(orient, width, ox, oy))
            instances.append(
                {
                    "area": shape["area"],
                    "count": count,
                    "placements": placements,
                }
            )
        if total_area > width * height:
            continue
        instances.sort(key=lambda s: (len(s["placements"]), -s["area"]))
        if search(instances, 0):
            fit_count += 1
    return fit_count


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
