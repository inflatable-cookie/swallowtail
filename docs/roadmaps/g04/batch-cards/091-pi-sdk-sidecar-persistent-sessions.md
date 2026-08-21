# 091 Pi SDK Sidecar Persistent Sessions

Status: ready
Owner: Tom
Created: 2026-08-21
Milestone: `../033-pi-sdk-sidecar-route.md`
Depends on: card 090

## Goal

Realize Contract 017 persistent new, load-with-replay, and replay-free resume
for the Pi SDK sidecar under the exact host-leased resource.

## Scope

1. Add a distinct persistent-session prepared facade for the SDK sidecar.
2. On new session, return only the opaque provider-session identity and exact
   restart binding needed by Contract 017.
3. On load and resume, pass the host-leased cwd through public
   `switchSession(sessionPath, { cwdOverride })` and compare `runtime.cwd`
   before declaring ready.
4. On load, project bounded ordered replay from public typed SDK messages and
   finish replay before readiness.
5. On resume, attach to the same exact session without replay.
6. Reject absent, stale, substituted, mismatched, malformed, or over-bound
   session and restart bindings before provider work.
7. Keep fresh replacement and consumer-authored context handoff distinct from
   provider-session attachment.
8. Abort and dispose session work, join the process, then release provider
   state, credential, resource, and execution-host leases in contract order.
9. Prove deterministic new/load/resume, replay bounds, mismatch, cancellation,
   deadline, disconnect, and cleanup fixtures.

## Out Of Scope

- direct Pi JSONL parsing, copying, rewriting, or stored-cwd trust
- hidden replay prompts or consumer-context reconstruction
- interrupted-turn, callback, or cross-process operation recovery
- provider-wide session listing, import, fork, compaction, or tree navigation
- addable connection admission and public matrices (card 092)

## Acceptance Criteria

- new, load, resume, and fresh replacement remain distinct
- load and resume cannot become ready before exact cwd agreement
- load replay is typed, bounded, ordered, and complete before ready
- resume emits no replay
- no provider work begins on an invalid binding
- cleanup preserves durable provider state while joining all owned work

## Validation

- `effigy validate:focused swallowtail-adapter-pi swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-pi swallowtail-runtime swallowtail-testkit`
- `effigy qa:routes`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

Yes, into card 092.

## Stop Conditions

- Stop if the SDK does not expose the effective cwd before readiness.
- Stop if replay requires direct session-file parsing.
- Stop if restart truth needs raw paths or secrets in portable diagnostics.
- Stop if attachment would claim interrupted operation recovery.
