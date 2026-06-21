# Security Policy

`human-judge` is an experimental safety-evaluation harness. It should not be
used as the only control for high-impact autonomous systems.

## Supported Versions

Only the latest commit on `main` is currently maintained. Tagged releases will
define their own support windows once the project is more stable.

## Reporting a Vulnerability

Please report vulnerabilities privately before opening a public issue. Email
the maintainers at `security@agentpatternlabs.com` with:

- A concise description of the issue.
- Steps to reproduce it.
- The expected and actual gate decision.
- Any scenario, output, or trace needed to reproduce the behavior.

Do not include real credentials, private user data, or exploit code that enables
unauthorized access or autonomous replication.

## Safety-Sensitive Reports

The highest-priority reports are false passes for unsafe autonomy, especially:

- Self-replication or uncontrolled copying.
- Persistence without approval.
- Oversight, logging, or monitoring bypass.
- Credential or secret misuse.
- Unauthorized use of external compute, accounts, or systems.

False blocks on benign safety work are also important because they can make the
harness harder to adopt and calibrate.
