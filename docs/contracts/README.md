# Contracts

Contracts hold durable, testable rules. When sources disagree, active
contracts govern behavior before architecture, vision, and roadmaps.

## Start Here

- [Contract Index](contract-index.md) — every contract in one line
- [Contract Summaries](contract-summaries.md) — what each contract governs and
  why, in delivery order
- [001 Working Rules](001-working-rules.md) — delivery rules for every change
- [003 Portable Contract Kernel](003-portable-contract-kernel.md) — the shared
  provider-neutral vocabulary

## Reading Order

The contracts form three groups:

1. **Foundation (003-016)** — identity, runtime roles, execution layers,
   access, hosts, and the async operation lifecycle.
2. **State and transport (017-035)** — persistent sessions, serving, hosted
   transports, version qualification, and portable session options.
3. **Consumer surfaces (036-052)** — releases, prepared integration, activity,
   reconciliation, restoration, failure classification, and documentation.

Contracts 004-016 describe realized runtime and proof-driver structure.
Contracts 017-052 add portable features and consumer-facing surfaces. Each
contract file owns the exact rules; the summaries above are only orientation.
