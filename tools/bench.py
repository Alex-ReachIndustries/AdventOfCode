#!/usr/bin/env python3
import argparse
import json
import os
import shutil
import statistics
import subprocess
import sys
import tempfile
import time


def _run_command(cmd: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        cmd,
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )


def _simple_bench(cmd: str, runs: int, warmup: int) -> dict:
    for _ in range(warmup):
        result = _run_command(cmd)
        if result.returncode != 0:
            _raise_command_error(cmd, result)

    times_ms = []
    for _ in range(runs):
        start = time.perf_counter()
        result = _run_command(cmd)
        end = time.perf_counter()
        if result.returncode != 0:
            _raise_command_error(cmd, result)
        times_ms.append((end - start) * 1000.0)

    mean_ms = statistics.mean(times_ms)
    stdev_ms = statistics.pstdev(times_ms) if len(times_ms) > 1 else 0.0
    best_ms = min(times_ms) if times_ms else 0.0
    return {
        "mean_ms": mean_ms,
        "stdev_ms": stdev_ms,
        "best_ms": best_ms,
        "runs": runs,
        "source": "internal",
    }


def _hyperfine_bench(cmd: str, runs: int, warmup: int) -> dict:
    with tempfile.NamedTemporaryFile(delete=False) as handle:
        json_path = handle.name

    args = ["hyperfine", "--export-json", json_path, "--runs", str(runs)]
    if warmup > 0:
        args.extend(["--warmup", str(warmup)])
    args.append(cmd)

    result = subprocess.run(
        args,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        _raise_command_error(" ".join(args), result)

    with open(json_path, "r", encoding="utf-8") as handle:
        data = json.load(handle)
    os.remove(json_path)

    entry = data["results"][0]
    mean_ms = entry["mean"] * 1000.0
    stdev_ms = (entry["stddev"] or 0.0) * 1000.0
    best_ms = entry["min"] * 1000.0
    return {
        "mean_ms": mean_ms,
        "stdev_ms": stdev_ms,
        "best_ms": best_ms,
        "runs": runs,
        "source": "hyperfine",
    }


def _raise_command_error(cmd: str, result: subprocess.CompletedProcess) -> None:
    sys.stderr.write(f"Command failed: {cmd}\n")
    if result.stdout:
        sys.stderr.write(result.stdout)
    if result.stderr:
        sys.stderr.write(result.stderr)
    raise SystemExit(result.returncode)


def _print_stats(label: str, part: str, stats: dict, as_json: bool) -> None:
    if as_json:
        payload = {
            "label": label,
            "part": part,
            **stats,
        }
        print(json.dumps(payload))
        return

    prefix = f"[{label}] " if label else ""
    print(
        f"{prefix}{part} mean_ms={stats['mean_ms']:.3f} "
        f"stdev_ms={stats['stdev_ms']:.3f} best_ms={stats['best_ms']:.3f} "
        f"runs={stats['runs']} source={stats['source']}"
    )


def main() -> None:
    parser = argparse.ArgumentParser(description="Benchmark runner for AoC days.")
    parser.add_argument("--part1", required=True, help="Command for part 1")
    parser.add_argument("--part2", help="Command for part 2 (optional)")
    parser.add_argument("--runs", type=int, default=10)
    parser.add_argument("--warmup", type=int, default=2)
    parser.add_argument("--no-hyperfine", action="store_true")
    parser.add_argument("--label", default="")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    use_hyperfine = (not args.no_hyperfine) and shutil.which("hyperfine")
    bench = _hyperfine_bench if use_hyperfine else _simple_bench

    stats1 = bench(args.part1, args.runs, args.warmup)
    _print_stats(args.label, "part1", stats1, args.json)

    if args.part2:
        stats2 = bench(args.part2, args.runs, args.warmup)
        _print_stats(args.label, "part2", stats2, args.json)


if __name__ == "__main__":
    main()
