# 048 Late Activity Correlation Adoption

Status: completed
Owner: Tom
Created: 2026-08-07
Generation: g03
Depends on: card 047's failure-context diagnostics

## Defect

Nucleus hit `swallowtail.event_buffer_rejected` ("Activity identity changed
within one operation") on every Codex 0.147.0 turn where a dynamic tool call
ran. Codex 0.147 emits `item/started` for a dynamic tool call *before* the
`item/tool/call` request; the qualified order had been request-first. The
started observation establishes the activity identity with no correlation;
the completion then carries the callback correlation, and the runtime event
buffer's identity check rejected the turn.

## Change

- `ActivityLifecycleTracker` treats correlation as late-acquired linkage:
  adopting `None → Some(correlation)` is accepted and fixes the correlation
  for later observations; changing an established correlation or any fixed
  identity dimension (kind, disclosure, phase, owner, provider ref) still
  rejects.
- The identity-conflict diagnostic now names the activity and both
  identities (activity ids and kinds are protocol metadata, never user
  content), which is what isolated this defect in one live retry.

## Evidence

- New runtime test `correlation_is_adopted_once_then_fixed` covers adopt-once
  and change-rejection.
- `cargo test -p swallowtail-runtime` (142), focused validation for
  swallowtail-runtime and swallowtail-adapter-codex pass.
- Live evidence: nucleus banner after the enriched diagnostic —
  `codex-app-server:item:exec-… (established ConsumerOwnedTool/ProviderDisplayContent,
  incoming ConsumerOwnedTool/ProviderDisplayContent)` — kind and disclosure
  identical, isolating correlation as the conflicting dimension.
- Contract 044's Tool And Request Correlation section describes correlation
  as carried data, not an immutable-at-first-sight dimension; no contract
  amendment needed.
