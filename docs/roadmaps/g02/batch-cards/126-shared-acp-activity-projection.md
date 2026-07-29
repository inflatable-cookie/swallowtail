# 126 Shared ACP Activity Projection

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../037-acp-observable-agent-activity.md`
Depends on: card 125

## Goal

Decode bounded portable ACP activity structure once without creating provider
or runtime policy in the protocol crate.

## Scope

1. Add bounded decoded records for ACP message, thought, plan, tool, usage,
   mode, command, and unknown session updates.
2. Preserve session, tool-call, and update identity.
3. Distinguish deltas, replacement snapshots, and terminal tool status.
4. Keep runtime activity emission and route profiles in adapters.
5. Keep endpoint, access, provider, model, mode policy, callbacks, and cleanup
   outside the protocol boundary.
6. Add malformed, oversize, unknown, and additive-field tests.

## Out Of Scope

- one generic ACP runtime driver
- callback execution
- provider-specific access or activity claims
- raw JSON exposure

## Acceptance Criteria

- [ ] shared decoding loses no qualified semantic update
- [ ] the protocol crate imports no runtime or provider adapter
- [ ] unknown semantic updates retain a bounded namespace and no raw payload
- [ ] replacement and delta semantics remain distinct
- [ ] oversize or contradictory input fails safely
- [ ] existing framing and remote transport tests remain green

## Validation

- `cargo test -p swallowtail-protocol-acp`
- focused remote ACP transport tests
- `effigy check:rust`
- `effigy lint:rust`

## Stop Conditions

- Stop if shared decoding would decide provider policy.
- Stop if unknown preservation requires an uninterpreted public JSON value.

## Auto-Continuation

Continue to card 127 after shared protocol conformance passes.

