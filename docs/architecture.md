# Architecture

The harness is intentionally small and inspectable.

## Components

- `Scenario`: the probe definition. It contains the user prompt, risk level,
  allowed outcomes, disallowed behavior classes, and expected outcome.
- `AgentOutput`: the candidate response and any proposed actions.
- `JudgeResult`: one judge's score, confidence, reasons, and evidence.
- `GateDecision`: the final decision made from all judge results.
- `Trace`: the durable record tying the scenario, output, scores, and gate.

## Judge Stack

The default panel has three deterministic judges:

- `safety_policy`: catches disallowed behavior classes in content and actions.
- `oversight_intent`: checks whether high-risk prompts receive refusal and safe
  redirection.
- `usefulness`: checks that safe prompts are answered and unsafe prompts still
  receive a useful safe alternative.

The deterministic panel is not meant to be the final judge. Its value is that
failures are explainable, cheap to run in CI, and suitable for calibration
against human labels.

## Gate Policy

The gate is conservative:

- Any `fail` blocks.
- Any `review` escalates to human review.
- Warnings on high-risk scenarios escalate to human review.
- Other warnings pass with a warning.
- Clean traces ship.

## Extension Points

- Add new scenarios under `scenarios/`.
- Add labeled outputs under `examples/agent_outputs/`.
- Add judges by implementing the `Judge` trait in `src/judges.rs`.
- Add real human labels under `qa-loop/memory/`.
