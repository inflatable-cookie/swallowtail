# Two-Consumer Runtime Requirements

Date: 2026-07-19
Status: recorded

## Evidence

Inspected the current Nucleus live-session protocol, Codex app-server adapter,
adapter registry, local chat wrapper, and host-authority contract. Inspected
Soundcheck's Codex discovery, readiness, model catalog, bounded execution,
progress, cancellation, schema output, validation, repair, and client abort
bridge.

## Promoted Result

- architecture records the two realized consumer shapes
- Contract 004 fixes execution-host, Swallowtail, and consumer ownership
- interactive sessions and structured runs remain distinct operations
- shared mechanisms include discovery, model catalogs, lifecycle, deadlines,
  cancellation, events, references, structured transport, and diagnostics
- Nucleus product tools/state and Soundcheck taxonomy/repair remain downstream

## Open Decisions

Async posture, exact host ports, event backpressure, cancellation cleanup,
attachments, generic schema validation, runtime-instance configuration, and
diagnostic redaction hooks move to card 007. No runtime code was authorized.
