# Roadmaps

Roadmaps sequence work after vision, architecture, and contracts provide enough
shape.

## Current Generation

- [g05 Agent Runtime Surfaces And Route Truth](g05/README.md) — active
- [g04 Route Readiness And Connection Admission](g04/README.md) — completed
- [g03 Compatibility Maintenance And Consumer-Proven Hardening](g03/README.md) — completed
- [g02 Swallowtail Stabilization And Release Discipline](g02/README.md) — completed
- [g01 Swallowtail Foundation](g01/README.md) — completed
- [Generation Index](generation-index.md)
- [Long-Term Plan](long-term-plan.md)

## Next Task

Close the single operator decision compiled by
[g05.017](g05/017-kimi-code-acp-0-39-containment-gate.md) and its
[containment and mediation gate](../triage/2026-09-01-kimi-code-acp-0-39-containment-and-mediation-gate.md).
The gate re-derives the authority failure path from the prepared working
resource through the `terminal: false` advertisement to the upstream
`local.spawn`, and returns exactly three mutually exclusive directions for
`kimi-code.acp` above `0.38.0`: a permanent `QualifiedOnly` cap, an indefinite
`QualifiedOnly` cap with one artifact-level upstream re-open trigger, or
funding `HostEnforced` execution-host containment while the cap holds. All
three move the claim's newer-version posture to `QualifiedOnly`; that is part
of the direction, not a sub-choice. The ACP claim binds `AllowUnverified`
today, so current `main` is safe only for the exact known exclusions and a
newly published point above `0.38.0` would otherwise be admissible before a
checkpoint could react. Adapter or runtime mediation under `terminal: false`
is impossible; requalification from wire-shape stability, process ownership,
capability omission, `AmbientHost`, or a test-only wrapper is rejected; a cap
that keeps `AllowUnverified` and adds exclusions release by release is
internally inconsistent; and negotiated terminal execution is not a governing
choice, because it cannot close `0.39.1` alone and cannot be selected without
containment or an upstream change. The gate states a recommendation as
analysis only; no direction is accepted, and none is recorded in status,
contract language, or this pointer. Exact `0.39.0` and `0.39.1` stay excluded
and `Incompatible` until the operator answers.

Then validate a fresh all-route Contract 029 currentness checkpoint and select
the next single family from re-probed official points rather than from
Research 269's consumed rank. OpenCode, Ollama, Antigravity, Grok, and
`kimi-code.local-server` remain visible ordered-newer candidates;
`kimi-code.local-server` is its own family and needs its own identity run
before any claim. Do not bulk-bump from registry `latest`, do not lift
Gemini's deferral, and do not keep g05 open for currentness. The g05.009
provider-operation observation decision remains queued; card 034 stays
planned, not ready, and candidate F remains unpromoted at 249 proved / 518
remaining rows.

## Standing Lanes

Generation-independent work lives in
[standing-lanes.md](standing-lanes.md). Contract 029 currentness is the
first standing lane. It does not keep a generation open.

## Index

- [generation-index.md](./generation-index.md) — generation status
- [status-grammar.md](./status-grammar.md) — Status buckets and census phrases
  for `qa:docs:roadmaps:status`
- [long-term-plan.md](./long-term-plan.md) — staged multi-consumer adoption
- [per-route-feature-completion.md](g04/per-route-feature-completion.md) —
  route-local feature delivery programme
- [per-route-feature-inventory.md](g04/per-route-feature-inventory.md) — live
  disposition counts and parallel qualification queue
- [standing-lanes.md](standing-lanes.md) — generation-independent lanes
- [backlog/README.md](backlog/README.md) — deferred work and promotion gates
- [g01/README.md](g01/README.md) — completed foundation generation
- [g02/README.md](g02/README.md) — completed stabilization, provider-wide
  facade, activity, compatibility, and lifecycle generation
- [g03/README.md](g03/README.md) — completed compatibility-maintenance and
  consumer-proven hardening generation
- [g04/README.md](g04/README.md) — completed route-readiness and
  connection-admission generation
- [g05/README.md](g05/README.md) — active watchers, bounded skill inventory,
  feature projection, and route-currentness generation

## Status And Census

Batch-card and milestone `Status:` lines, plus the active generation census in
`generation-index.md`, must match
[status-grammar.md](./status-grammar.md). That note names the live regexes in
`scripts/check-roadmap-status-drift.py`. `gated` is detail after an accepted
bucket, not a status by itself.

## Generation Shape

Generations normally collect 30-50 numbered roadmaps. Batch cards sit inside
those roadmaps and do not count toward the generation range. A phase boundary
does not imply a generation rollover.
