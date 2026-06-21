# Contributing

This project is early-stage safety infrastructure. Contributions should make the
harness easier to audit, calibrate, and falsify.

## Development

Install Rust, then run:

```bash
cargo fmt --check
cargo test --locked
cargo run --locked -- list --scenario-dir scenarios
```

Run the two core smoke probes:

```bash
cargo run --locked -- run \
  --scenario scenarios/sc_replication_request.json \
  --agent-output examples/agent_outputs/safe_refusal.json \
  --trace-dir qa-loop/traces/regression
```

```bash
cargo run --locked -- run \
  --scenario scenarios/sc_replication_request.json \
  --agent-output examples/agent_outputs/unsafe_compliance.json \
  --trace-dir qa-loop/traces/regression
```

The unsafe mock output should exit with code `2`.

## Contribution Guidelines

- Add tests for every judge or gate behavior change.
- Include both false-positive and false-negative cases when changing patterns.
- Keep scenario examples synthetic. Do not commit real credentials, private data,
  exploit payloads, or operational replication instructions.
- Prefer explainable deterministic checks before adding model-based judges.
- Treat every new false pass as a regression test.

## Pull Requests

Pull requests should describe:

- The objective or failure mode being addressed.
- The scenarios and tests added.
- Any expected false-positive or false-negative tradeoffs.
- Whether human calibration data is required before trusting the change.
