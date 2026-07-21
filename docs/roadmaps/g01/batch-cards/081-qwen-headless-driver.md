# 081 Qwen Headless Driver

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../026-qwen-headless-structured-harness-proof.md`

## Objective

Implement the pinned Qwen headless structured-run driver against card 080's
frozen corpus.

## Governing References

- Research 017
- Contracts 005, 006, 008-011, 014, and 023
- roadmap 026
- card 080 corpus and completion evidence

## Scope

- separate Qwen integration, adapter, transport, instance, access, and model
  identities
- exact preflight binding for executable, environment, host, working resource,
  access, route, model, isolation, durable retention, deadlines, and capabilities
- stdin content and bounded stream-JSON normalization
- explicit safe mode, approval posture, tool exclusions, native bounds, and no
  persistent retry or sandbox
- cancellation, deadline, process stop, terminal mapping, redaction, and joined
  cleanup
- separately gated installed/authenticated probe only after offline proof

## Acceptance Criteria

- [x] production argv matches the frozen corpus exactly
- [x] no prompt, secret, path, or raw provider payload enters diagnostics
- [x] process start occurs only after exact preflight and host binding
- [x] native budget, cancellation, timeout, provider failure, and protocol
      failure remain distinct
- [x] process and reader work join before terminal cleanup completes
- [x] no resume, deletion, sandbox, provider fallback, or direct-inference claim

## Validation

- focused Qwen driver tests
- focused warnings-denied clippy
- all-target workspace compile
- `git diff --check`

## Stop Conditions

- card 080 corpus drifts
- process behavior requires an uncontracted host service
- safe-mode or tool-exclusion behavior differs from the frozen release
- a credential or configured provider must be inspected outside host authority

## Auto-Continuation

No. Promote card 082 only after deterministic production mapping passes.

## Completion Evidence

- the separately registered `swallowtail.qwen.headless` driver binds one exact
  executable, host-approved environment, working resource, provider, model,
  delegated harness access profile, deadline, and `AmbientHost` policy
- production argv matches card 080's frozen `v0.19.11` corpus; content travels
  only through stdin and the process receives no sandbox, resume, background,
  write, shell, network, or provider-fallback authority
- bounded stream JSON projects progress, final output, and typed usage while
  unknown events remain safe observations and malformed or raw provider data
  never enters diagnostics
- seven deterministic driver tests distinguish success, native turn and run
  budgets, provider failure, protocol failure, cancellation, and timeout; all
  paths wait for the process and close joins the reader task
- 13 Qwen tests, focused warnings-denied clippy, and all-target workspace
  compilation pass; full provider-neutral conformance remains card 082
- doctor remains at the inherited 19 findings: 12 warnings and 7 errors
