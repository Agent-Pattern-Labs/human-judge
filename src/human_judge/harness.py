from __future__ import annotations

import hashlib
import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from .gate import decide_gate
from .judges import run_default_judges
from .models import AgentOutput, Scenario


def load_json(path: str | Path) -> dict[str, Any]:
    with Path(path).open("r", encoding="utf-8") as handle:
        return json.load(handle)


def load_scenario(path: str | Path) -> Scenario:
    return Scenario.from_dict(load_json(path))


def load_agent_output(path: str | Path) -> AgentOutput:
    path = Path(path)
    if path.suffix.lower() == ".json":
        return AgentOutput.from_dict(load_json(path))
    return AgentOutput(agent_id=path.stem, content=path.read_text(encoding="utf-8"))


def write_json(path: str | Path, data: dict[str, Any]) -> None:
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=2, sort_keys=True)
        handle.write("\n")


def hash_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()[:16]


def safe_trace_part(value: str) -> str:
    return re.sub(r"[^A-Za-z0-9_.-]+", "_", value).strip("_") or "unknown"


def run_evaluation(
    scenario_path: str | Path,
    agent_output_path: str | Path,
    trace_dir: str | Path,
) -> dict[str, Any]:
    scenario = load_scenario(scenario_path)
    output = load_agent_output(agent_output_path)
    started_at = datetime.now(timezone.utc)
    judge_results = run_default_judges(scenario, output)
    gate = decide_gate(scenario, judge_results)
    ended_at = datetime.now(timezone.utc)
    trace_id = (
        f"tr_{safe_trace_part(scenario.scenario_id)}_"
        f"{safe_trace_part(output.agent_id)}_"
        f"{ended_at.strftime('%Y%m%dT%H%M%S%fZ')}"
    )
    trace_path = Path(trace_dir) / f"{trace_id}.json"

    trace = {
        "trace_id": trace_id,
        "scenario_id": scenario.scenario_id,
        "objective_id": scenario.objective_id,
        "environment": "local",
        "started_at": started_at.isoformat(),
        "ended_at": ended_at.isoformat(),
        "actor": {
            "type": "agent_output_file",
            "agent_id": output.agent_id,
        },
        "world": {
            "risk_level": scenario.risk_level,
            "scenario_file": str(scenario_path),
            "agent_output_file": str(agent_output_path),
        },
        "inputs": {
            "prompt_hash": hash_text(scenario.user_prompt),
            "output_hash": hash_text(output.content),
            "proposed_actions": output.actions,
        },
        "outcome": {
            "task_completed": gate.decision in {"ship", "warn"},
            "errors": 0,
            "support_needed": gate.decision == "human_review",
        },
        "judge_scores": [result.to_dict() for result in judge_results],
        "gate": gate.to_dict(),
    }

    write_json(trace_path, trace)
    trace["trace_path"] = str(trace_path)
    return trace
