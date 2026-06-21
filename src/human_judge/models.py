from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any, Literal

Score = Literal["pass", "warn", "fail", "review"]
GateStatus = Literal["ship", "warn", "human_review", "block"]


@dataclass(frozen=True)
class Scenario:
    scenario_id: str
    objective_id: str
    name: str
    risk_level: str
    user_prompt: str
    allowed_outcomes: list[str]
    disallowed_behaviors: list[str]
    expected_outcome: dict[str, Any]
    judges: list[str]
    metadata: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "Scenario":
        required = {
            "scenario_id",
            "objective_id",
            "name",
            "risk_level",
            "user_prompt",
            "allowed_outcomes",
            "disallowed_behaviors",
            "expected_outcome",
            "judges",
        }
        missing = sorted(required - data.keys())
        if missing:
            raise ValueError(f"Scenario is missing required fields: {missing}")

        known = required
        metadata = {key: value for key, value in data.items() if key not in known}
        return cls(
            scenario_id=str(data["scenario_id"]),
            objective_id=str(data["objective_id"]),
            name=str(data["name"]),
            risk_level=str(data["risk_level"]),
            user_prompt=str(data["user_prompt"]),
            allowed_outcomes=list(data["allowed_outcomes"]),
            disallowed_behaviors=list(data["disallowed_behaviors"]),
            expected_outcome=dict(data["expected_outcome"]),
            judges=list(data["judges"]),
            metadata=metadata,
        )


@dataclass(frozen=True)
class AgentOutput:
    agent_id: str
    content: str
    actions: list[str] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "AgentOutput":
        if "content" not in data:
            raise ValueError("Agent output is missing required field: content")
        known = {"agent_id", "content", "actions"}
        metadata = {key: value for key, value in data.items() if key not in known}
        return cls(
            agent_id=str(data.get("agent_id", "unknown_agent")),
            content=str(data["content"]),
            actions=[str(action) for action in data.get("actions", [])],
            metadata=metadata,
        )


@dataclass(frozen=True)
class JudgeResult:
    judge_id: str
    score: Score
    confidence: float
    reasons: list[str]
    evidence: list[str] = field(default_factory=list)
    suggested_fix: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True)
class GateDecision:
    decision: GateStatus
    reason: str
    blocking_judges: list[str] = field(default_factory=list)
    review_judges: list[str] = field(default_factory=list)
    warning_judges: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)
