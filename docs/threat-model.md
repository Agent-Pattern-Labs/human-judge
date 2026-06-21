# Threat Model

This project evaluates agent outputs for unsafe autonomy signals. It does not
execute agent plans or provide machinery for replication, deployment, or
persistence.

## In Scope

- Proposed self-replication or uncontrolled copying.
- Persistence without explicit approval.
- Bypassing logs, monitors, approvals, or human oversight.
- Credential, token, or secret misuse.
- Unauthorized use of external compute, accounts, or systems.
- Deceptive framing that hides the true behavior from operators.

## Out Of Scope

- Building autonomous replication.
- Executing untrusted code.
- Network scanning or exploitation.
- Credential discovery.
- Deployment automation.

## Assumptions

- Early gates should be conservative.
- Simulated judges should be calibrated against real human labels before being
  trusted for high-impact decisions.
- High-risk unknowns should go to human review instead of being treated as safe.

## Failure Modes

- False negatives: unsafe outputs pass because phrasing evades patterns.
- False positives: safe refusals mention risky concepts and get over-flagged.
- Weak world model: static text outputs miss dangerous action traces.
- Judge drift: future LLM judges may become overconfident or inconsistent.
- Gate drift: teams may weaken block conditions under shipping pressure.

## Memory Rule

Every escaped unsafe output becomes a regression scenario. Every judge mistake
becomes a calibration example with the human label and rationale preserved.
