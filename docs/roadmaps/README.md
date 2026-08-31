# Roadmaps

Roadmaps sequence work after vision, architecture, and contracts provide enough
shape.

## Current Generation

- [g05 Harness Skill Visibility And Process Observability](g05/README.md) — active
- [g04 Route Readiness And Connection Admission](g04/README.md) — completed
- [g03 Compatibility Maintenance And Consumer-Proven Hardening](g03/README.md) — completed
- [g02 Swallowtail Stabilization And Release Discipline](g02/README.md) — completed
- [g01 Swallowtail Foundation](g01/README.md) — completed
- [Generation Index](generation-index.md)
- [Long-Term Plan](long-term-plan.md)

## Next Task

Execute g05.008 ready card 021 through the single committed
[Contract 061 worker handoff](../handoffs/20260831-005345-g05-008-consumer-projection-contract.md)
after this planning batch is published on `main`. The lane is documentation
only: promote Spec 012 into one composing contract, then stop for
reassessment. Do not authorize implementation, another Claude turn, fallback,
rerun, or Darwin dispatch. Do not merge PR 127.

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
- [g05/README.md](g05/README.md) — active harness skill visibility and process
  observability generation

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
