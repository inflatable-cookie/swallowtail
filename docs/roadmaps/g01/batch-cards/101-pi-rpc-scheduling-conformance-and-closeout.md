# 101 Pi RPC Scheduling Conformance And Closeout

Status: pending
Owner: Tom
Updated: 2026-07-22
Milestone: `../035-pi-rpc-harness-proof.md`

## Objective

Prove the production Pi driver under both execution-host identities, close
roadmap 035, and leave one direct-inference contract task.

## Readiness Gate

Card 100 must complete with the production driver passing deterministic local
fixtures.

## Scope

- unchanged long-lived RPC profile plus the Contract 028 assertion pack
- local and remote-authoritative host identity
- exact provider/model/access/resource/posture agreement
- prompt, steering, follow-up, callback, abort, failure, cancellation,
  deadline, disconnect, redaction, and cleanup matrix
- full repository QA and exact test inventory
- roadmap, front-door, architecture, log, and sole Next Task currentness
- compile a bounded DeepSeek V4 direct-continuation research card; no provider
  implementation without its contract

## Acceptance Criteria

- [ ] production fixtures pass the shared RPC contract under both topologies
- [ ] scheduling order and command acknowledgement remain exact
- [ ] callback timeout and late-response behavior are deterministic
- [ ] no implicit provider, model, auth, retry, sandbox, or fallback appears
- [ ] cleanup evidence remains visible when provider work succeeds
- [ ] full QA passes or failures are recorded honestly
- [ ] roadmap 035 closes with one sole next task

## Validation

- focused Pi conformance and failure suites
- `effigy qa --json`
- exact test inventory
- `effigy doctor --json` delta review
- `git diff --check`

## Stop Conditions

- local and remote-authoritative behavior diverges
- conformance requires weakening another profile
- driver completion hides callback or process cleanup failure
- direct continuation decisions remain ambiguous enough to set consumer policy

## Auto-Continuation

No. Close the harness proof before starting direct-continuation research.

