# Responsible Use

`human-judge` is a research prototype for evaluating candidate agent outputs.
It is not a complete containment system, not a human-level simulator, and not a
substitute for security review.

## Appropriate Uses

- Testing whether mock agent outputs trigger conservative safety gates.
- Creating regression scenarios for unsafe autonomy signals.
- Comparing deterministic judges against human-labeled examples.
- Building audit trails for scenario-based evaluation.

## Inappropriate Uses

- Treating a passing gate as proof that an agent is safe.
- Running untrusted agent code through this harness.
- Creating, deploying, or improving self-replicating agents.
- Bypassing monitoring, approval, logging, or human review.
- Testing with real credentials, private data, or unauthorized systems.

## Operating Principle

High-risk uncertainty should escalate to human review. False passes are safety
failures. False blocks are calibration failures. Both should become regression
tests with clear trace evidence.
