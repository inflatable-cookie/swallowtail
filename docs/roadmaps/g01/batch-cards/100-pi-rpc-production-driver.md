# 100 Pi RPC Production Driver

Status: ready
Owner: Tom
Updated: 2026-07-22
Milestone: `../035-pi-rpc-harness-proof.md`

## Objective

Implement the separately registered Pi `0.80.10` RPC driver against the frozen
corpus and Contract 028 boundary.

## Readiness Gate

Card 099 is complete with stable shared records, exact runtime binding,
maintained compatibility-window classification, pure preflight, unchanged
profile posture, and a frozen corpus.

## Scope

- production driver inside the fixture-only `swallowtail-adapter-pi` scaffold
- exact executable, version, argv, environment, downstream provider, model,
  working directory, delegated-auth, and `AmbientHost` binding
- supervised piped process with strict-LF bounded JSONL
- prompt, steering, follow-up, abort, state, event, and UI request projection
- disabled session persistence, customization, update, telemetry, package,
  retry, write, edit, and bash behavior
- bounded queues, one active operation, two completed prompts
- cancellation, deadline, disconnect, safe error, redaction, and joined cleanup
- deterministic process fixture only; no installed or live provider call

## Acceptance Criteria

- [ ] the descriptor is separate from Codex, ACP, and direct-inference drivers
- [ ] exact downstream provider and model reach argv without fallback
- [ ] no process or credential work occurs before successful preflight
- [ ] command acknowledgement and model terminal events remain separate
- [ ] native abort cannot overclaim provider stop
- [ ] ambient read-intent behavior makes no containment claim
- [ ] excluded sources and retry stay disabled
- [ ] all process, reader, callback, resource, and auth work joins before close

## Validation

- focused Pi adapter and host fixtures
- focused warnings-denied clippy
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- production behavior diverges from the frozen package/protocol version
- exact model or provider cannot be validated
- process startup needs package installation or credential mutation
- callback relay needs raw payload exposure
- cleanup requires detached work

## Auto-Continuation

No. Confirm production behavior before cross-topology conformance.
