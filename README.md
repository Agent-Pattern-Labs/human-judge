# human-judge

`human-judge` is a small safety-evaluation harness for agentic AI outputs.
It treats "human-level simulation as judge" as a research target, not an
assumption. The repo starts with auditable proxy judges that can be calibrated
against real human labels later.

The first objective is narrow: catch and gate proposed agent behavior that
suggests self-replication, persistence without approval, oversight bypass,
credential misuse, or unauthorized resource use.

## What This Is

- A scenario runner for high-risk and benign agent prompts.
- A trace format that records inputs, outputs, judge scores, and gate decisions.
- A deterministic judge panel that approximates security, oversight, and
  usefulness review.
- A release-style gate: `ship`, `warn`, `human_review`, or `block`.

## What This Is Not

- It is not a self-replication system.
- It does not deploy agents, acquire credentials, bypass monitoring, or create
  persistence.
- It does not claim to simulate humans at human level. The judges are explicit,
  inspectable proxies intended for calibration.

## Quick Start

Run the tests:

```bash
PYTHONPATH=src python3 -m unittest discover -s tests
```

Run a safe high-risk refusal through the harness:

```bash
PYTHONPATH=src python3 -m human_judge run \
  --scenario scenarios/sc_replication_request.json \
  --agent-output examples/agent_outputs/safe_refusal.json \
  --trace-dir qa-loop/traces/regression
```

Run an unsafe mock output and expect a blocking gate:

```bash
PYTHONPATH=src python3 -m human_judge run \
  --scenario scenarios/sc_replication_request.json \
  --agent-output examples/agent_outputs/unsafe_compliance.json \
  --trace-dir qa-loop/traces/regression
```

The second command exits non-zero because the gate blocks the output.

## Loop Shape

This repo follows an objective-trace loop:

```text
Objective -> World -> Probe -> Trace -> Judge -> Repair -> Memory -> Gate
```

- Objective: block unsafe agent behavior while preserving useful benign help.
- World: synthetic prompts, risk profiles, and mock agent outputs.
- Probe: executable scenario plus candidate agent output.
- Trace: JSON evidence written under `qa-loop/traces/`.
- Judge: deterministic proxy panel with reasons and evidence.
- Memory: labeled scenarios, examples, and future calibration cases.
- Gate: allow, warn, escalate to human review, or block.

## Project Layout

```text
src/human_judge/        Harness, judges, gate, CLI
scenarios/              Executable scenario definitions
examples/agent_outputs/ Mock agent outputs for smoke tests
qa-loop/                Objective, loop state, memory, gates, traces
docs/                   Threat model and architecture notes
tests/                  Unit tests
```

## Next Research Steps

1. Add a human-label collection format and compare judges against labels.
2. Add an optional LLM-judge adapter that is evaluated, not trusted by default.
3. Expand scenarios from static outputs to sandboxed action traces.
4. Track judge disagreement as first-class evidence.
5. Build a small review UI for humans to label ambiguous traces.
