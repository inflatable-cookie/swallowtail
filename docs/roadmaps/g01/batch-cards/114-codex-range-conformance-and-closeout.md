# 114 Codex Range Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Prove both Codex ranges across their exact boundaries, milestones, transports,
and host topologies.

## Scope

- unchanged one-shot structured-CLI and long-lived RPC-harness profiles
- local and remote-authoritative installed-executable observation
- exec baseline, interior points, prior pin, latest, and rejection neighbors
- app-server baseline, both sides of the workspace-root milestone, interior
  points, prior pin, latest, and rejection neighbors
- stable and experimental session variants
- model catalogue, read-only and bounded-write access, tools, callbacks,
  provider requests, cancellation, deadlines, resume constraints, redaction,
  and joined cleanup
- full repository QA, doctor delta, roadmap currentness, and closeout log
- no live authentication in default QA

## Acceptance Criteria

- [x] both ranges are no wider than their frozen corpora
- [x] every behavior segment dispatches from the exact immutable binding
- [x] both common profiles gain no Codex-specific branch
- [x] current `0.145.0` and the maintained baselines pass
- [x] rejected and unknown versions fail before harness work
- [x] full validation passes apart from documented structural debt
- [x] the legacy-span continuation remains explicit

## Validation

- focused Codex and conformance tests
- workspace all-target check
- workspace warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 115 after the current Codex windows close.

## Evidence

- local and remote-authoritative discovery use the same exact target binding
- exec runs pass at `0.122.0`, `0.130.0`, `0.140.0`, `0.144.6`, and `0.145.0`
- app-server catalogue runs pass at `0.110.0`, `0.120.0`, `0.131.0`,
  `0.140.0`, `0.144.6`, and `0.145.0`
- `0.130.0` rejects bounded workspace access before resource or process work;
  `0.131.0` accepts it behind explicit experimental negotiation
- missing, below-floor, prerelease, malformed, and unknown-newer versions
  reject before harness work
- full QA passes with a 549-test inventory: 545 pass and four separately gated
  installed/live probes remain ignored
- doctor remains at the inherited 19 findings: seven errors and twelve warnings
- the generated Alibaba test artifact required temporary ad-hoc signing after
  macOS stopped the unchanged binary in `_dyld_start`; no source, installed
  executable, provider access, or repository artifact changed
