from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Sequence

from .harness import run_evaluation

EXIT_CODES = {
    "ship": 0,
    "warn": 0,
    "human_review": 3,
    "block": 2,
}


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="human-judge",
        description="Run safety judge scenarios against agent outputs.",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    run_parser = subparsers.add_parser("run", help="Run one scenario evaluation.")
    run_parser.add_argument("--scenario", required=True, help="Path to scenario JSON.")
    run_parser.add_argument(
        "--agent-output",
        required=True,
        help="Path to an agent output JSON or text file.",
    )
    run_parser.add_argument(
        "--trace-dir",
        default="qa-loop/traces/regression",
        help="Directory where trace JSON should be written.",
    )
    run_parser.add_argument(
        "--json",
        action="store_true",
        help="Print the full trace JSON instead of a concise summary.",
    )

    list_parser = subparsers.add_parser("list", help="List scenario files.")
    list_parser.add_argument(
        "--scenario-dir",
        default="scenarios",
        help="Directory containing scenario JSON files.",
    )

    return parser


def summarize(trace: dict) -> str:
    lines = [
        f"Gate: {trace['gate']['decision']}",
        f"Reason: {trace['gate']['reason']}",
        f"Trace: {trace['trace_path']}",
        "Scores:",
    ]
    for result in trace["judge_scores"]:
        reason = "; ".join(result["reasons"])
        lines.append(
            f"- {result['judge_id']}: {result['score']} "
            f"(confidence={result['confidence']:.2f}) - {reason}"
        )
    return "\n".join(lines)


def list_scenarios(scenario_dir: str) -> str:
    paths = sorted(Path(scenario_dir).glob("*.json"))
    if not paths:
        return f"No scenarios found in {scenario_dir}"
    return "\n".join(str(path) for path in paths)


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "list":
        print(list_scenarios(args.scenario_dir))
        return 0

    trace = run_evaluation(args.scenario, args.agent_output, args.trace_dir)
    if args.json:
        print(json.dumps(trace, indent=2, sort_keys=True))
    else:
        print(summarize(trace))
    return EXIT_CODES[trace["gate"]["decision"]]
