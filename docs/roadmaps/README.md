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

Implement [g05.009 card 073](g05/batch-cards/073-contract-061-provider-operation-observation-baseline.md):
realize the promoted provider-operation observation vocabulary, fourth view,
and six testkit assertions in runtime and testkit only, from the manifest in
that roadmap. Audit 065 continues. Card 071 is under exact-head review as an
evidence stop at Antigravity `1.1.22`. The all-route currentness checkpoint
lane in `standing-lanes.md` is approved to run concurrently and writes
Research 284 without changing claims.
Cards 068 and 069 are merged. Card 062 stopped: Kimi local server `0.41.0`
identity is frozen and the ceiling stays `0.38.0`; card 063 stays gated.
Card 034 remains planned behind the promoted baseline.
`v0.4.0` is tagged; the feature/currentness freeze is lifted.

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
