# 101 Pi RPC Scheduling Conformance And Closeout

Status: completed
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

- [x] production fixtures pass the shared RPC contract under both topologies
- [x] scheduling order and command acknowledgement remain exact
- [x] callback timeout and late-response behavior are deterministic
- [x] no implicit provider, model, auth, retry, sandbox, or fallback appears
- [x] cleanup evidence remains visible when provider work succeeds
- [x] full QA passes or failures are recorded honestly
- [x] roadmap 035 closes with one sole next task

## Completion Evidence

- the production driver passes the unchanged long-lived RPC profile and the
  separate Contract 028 assertion pack
- local and remote-authoritative fixtures preserve exact host, target,
  configured-instance, provider, model, resource, access, and `AmbientHost`
  identity
- deterministic scheduling keeps prompt, steering, and follow-up distinct and
  separates command acknowledgement from model settlement
- callback deadlines send one cancelled response, expiry stays callback-local,
  late responses fail with an exact safe diagnostic, and timer work joins
- provider failure, retry drift, disconnect, malformed frames, response
  mismatch, bounded concurrency, cancellation, deadline, redaction, and
  process-cleanup failure remain distinct
- focused core/runtime/testkit/Pi validation passes 108 tests; full repository
  inventory and QA are recorded in the closeout log
- roadmap 036 and cards 102-104 compile the bounded DeepSeek V4 continuation
  lane; card 102 is the sole ready task and does not authorize implementation

## Validation

- focused Pi, runtime, and testkit validation passes 108 tests
- `effigy qa --json` passes on the complete rerun
- exact inventory is 469 tests: 466 pass and 3 gated probes remain ignored
- `effigy doctor --json` retains the inherited 19 findings: 7 errors and 12
  warnings; no Pi file enters the report
- `git diff --check` passes
- the first full test attempt hit a Gemini Live mock-server broken-pipe race;
  the exact case and the complete QA rerun passed without code changes

## Stop Conditions

- local and remote-authoritative behavior diverges
- conformance requires weakening another profile
- driver completion hides callback or process cleanup failure
- direct continuation decisions remain ambiguous enough to set consumer policy

## Auto-Continuation

No. Close the harness proof before starting direct-continuation research.
