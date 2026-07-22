# 099 Pi RPC Contract Records And Corpus

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../035-pi-rpc-harness-proof.md`

## Objective

Realize the minimum provider-neutral RPC scheduling and UI-relay boundary and
freeze exact Pi `0.80.10` protocol evidence before production process work.

## Readiness Gate

Research 022 selects Pi RPC. Contract 028 fixes message scheduling, callback
relay, downstream route identity, ambient authority, retry, and cleanup. No
provider, model, sandbox, or credential default remains.

## Governing References

- Research 022
- Contracts 005, 006, 009-012, 017, 023, 028, and 029
- roadmap 035
- maintained Pi package, RPC, SDK, and coding-agent evidence accessed
  2026-07-22

## Scope

- prompt, steering, follow-up, and abort scheduling records
- command acknowledgement distinct from model-operation lifecycle
- bounded correlated dialog and display-only UI request records
- exact downstream provider and model-route agreement
- `AmbientHost` read-intent posture with no containment claim
- disabled customization, update, telemetry, package, and retry policy
- provider-neutral scheduling/UI assertion pack over the existing long-lived
  RPC profile unless evidence requires a separate profile
- exact interface-version binding plus a maintained baseline, milestone,
  deprecation, and exclusion window; no unproved range claim
- frozen Pi strict-LF command, response, event, callback, error, malformed,
  unknown, disconnect, and close fixtures
- no production process, installed Pi, credential, or external model request

## Ordered Work

1. Add the smallest scheduling, queue-bound, and UI-relay records required by
   Contract 028.
2. Bind the exact Pi package qualification point under Contract 029.
3. Extend pure preflight for exact provider, model, posture, disabled-source,
   retry, and queue-bound agreement.
4. Add provider-neutral assertions for acknowledgement, scheduling order,
   callback timeout, late response, display-only UI, and joined cleanup.
5. Freeze the Pi `0.80.10` strict-LF corpus and exact invocation inputs.
6. Prove redaction and reject unknown records without adding Pi identity to
   core, runtime, or testkit.
7. Run focused core, runtime, testkit, and new adapter-fixture tests plus
   warnings-denied clippy before card 100 becomes ready.

## Acceptance Criteria

- [x] scheduling classes are separate and positively bounded
- [x] command response cannot imply model completion
- [x] UI dialog and display-only behavior remain distinct
- [x] downstream provider and model are exact preflight inputs
- [x] exact interface versions are bound without claiming an untested range
- [x] one release can retain older qualified versions across private behavior
      milestones, with explicit deprecation and exclusions
- [x] automatic retry and ambient customization are disabled
- [x] read-intent tools do not satisfy enforced isolation
- [x] existing eleven profiles do not widen without evidence
- [x] raw RPC, path, prompt, output, callback, credential, and provider data
      stay outside stable diagnostics
- [x] default QA uses no live harness or provider state

## Evidence

- Contract 029 separates exact interface observations from revisioned
  compatibility windows. The initial Pi claim is a one-point window at
  `0.80.10`; shared fixtures prove baseline, milestone, deprecation, exclusion,
  and out-of-window behavior for future expansion.
- core preflight rejects an unqualified version or mismatched restrictive RPC
  policy before effects
- the long-lived RPC profile remains one of eleven; a separate assertion pack
  proves scheduling, acknowledgement, UI relay, policy, and version behavior
- `swallowtail-adapter-pi` contains only adapter-private protocol decoding and
  the frozen corpus; no process driver or live access exists yet
- 129 focused core, runtime, testkit, and Pi tests pass
- focused warnings-denied clippy, workspace all-target check, docs QA, explicit
  index links, and diff checks pass
- doctor returns the inherited 19 oversized-file findings: 7 errors and 12
  warnings, with no batch-added finding

## Validation

- focused core, runtime, testkit, and Pi fixture tests
- focused warnings-denied clippy
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- required records import Pi-specific vocabulary into the shared API
- model scheduling cannot remain distinct from command acknowledgement
- useful callback relay requires executing consumer tools inside Swallowtail
- the exact package or protocol cannot be pinned
- fixture work requires installation, login, credential, or network access

## Auto-Continuation

No. Confirm the shared records and frozen corpus before production process
work.
