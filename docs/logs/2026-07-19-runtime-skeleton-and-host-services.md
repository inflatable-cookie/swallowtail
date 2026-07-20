# Runtime Skeleton and Host Services

Date: 2026-07-19
Status: recorded

## Result

g01 cards 012-014 are complete.

- `swallowtail-runtime` is the third workspace crate. Its only dependencies are
  `swallowtail-core` and `futures-core`.
- Four separate object-safe driver roles store and call through dynamic trait
  objects. Registration cannot attach a role absent from the descriptor.
- Run, session, turn, attached-serving, and owned-serving handle traits expose
  distinct lifecycle authority.
- The bounded event channel enforces start order, monotonic sequence,
  coalescing, semantic-overflow failure, terminal close, and late-event
  quarantine.
- Terminal outcomes are first-wins and keep provider state separate from
  cleanup state. Cancellation is scoped and idempotent.
- Typed optional host ports cover task, blocking work, monotonic time, process,
  network policy, credentials, working resources, attachments, and restricted
  diagnostics.
- Portable references redact host-owned values. Inline schemas are bounded;
  secret leases redact and release on drop.
- Testkit recording services expose every attempted host call with scripted
  outcomes and no real I/O.

## Evidence

- 35 tests pass across core, runtime, and testkit.
- Formatting, checking, clippy, docs QA, links, and diff hygiene pass.
- Effigy reports no new god-file warnings after runtime input and event modules
  were split by contract boundary.

## Next Lane

Card 015 composes these public primitives into one-shot CLI, long-lived RPC or
ACP, hosted API, attached self-hosted, and owned self-hosted synthetic profiles.
Card 016 then validates and promotes the complete runtime kernel.
