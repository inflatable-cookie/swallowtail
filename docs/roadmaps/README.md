# Roadmaps

Roadmaps sequence work after vision, architecture, and contracts provide enough
shape.

## Current Generation

- [g05 Agent Runtime Surfaces And Route Truth](g05/README.md) — active; the sole expanded generation
- [Generation Index](generation-index.md)
- [Long-Term Plan](long-term-plan.md)

Closed generations live only as roll-ups under
[archive/](archive/): [g04](archive/g04.md), [g03](archive/g03.md),
[g02](archive/g02.md), [g01](archive/g01.md).

## Next Task

Reassess the g05 runway through a Chatterbox planning ruling. g05.055 completed
the operator-authorized `v0.5.1` annotated source tag at exact candidate
`e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27` (tree `d375b322…`, tag object
`97a6933a`); the tag-triggered CI run 34743711387 is green 11/11 and
documentation closeout PR #318 merged at `b0c800e0` after independent
exact-head review `5651817371`. Canonical `main` is synchronized at that
merge. No task is ready: planned tasks 035 and
039–042 await operator promotion, disposition, or requirements. Desktop
g02.089 Phase B may resume against the ordinary `v0.5.1` pin; no registry,
GitHub Release, artifact, provider call, or consumer mutation follows
automatically.

## Standing Lanes

Generation-independent work lives in
[standing-lanes.md](standing-lanes.md). Contract 029 currentness is the
first standing lane. It does not keep a generation open.

## Index

- [generation-index.md](./generation-index.md) — generation status
- [status-grammar.md](./status-grammar.md) — Status buckets and census phrases
  for `qa:docs:roadmaps:status`
- [long-term-plan.md](./long-term-plan.md) — staged multi-consumer adoption
- [archive/per-route-feature-completion.md](archive/per-route-feature-completion.md) —
  route-local feature delivery programme (retained with the g04 roll-up)
- [archive/per-route-feature-inventory.md](archive/per-route-feature-inventory.md) —
  past disposition counts and parallel qualification queue (retained with the g04 roll-up)
- [standing-lanes.md](standing-lanes.md) — generation-independent lanes
- [archive/g01.md](archive/g01.md) — completed foundation generation roll-up
- [archive/g02.md](archive/g02.md) — completed stabilization, provider-wide
  facade, activity, compatibility, and lifecycle generation roll-up
- [archive/g03.md](archive/g03.md) — completed compatibility-maintenance and
  consumer-proven hardening generation roll-up
- [archive/g04.md](archive/g04.md) — completed route-readiness and
  connection-admission generation roll-up
- [g05/README.md](g05/README.md) — active watchers, bounded skill inventory,
  feature projection, and route-currentness generation

## Status And Census

Task `Status:` lines, plus the active generation census in
`generation-index.md`, must match
[status-grammar.md](./status-grammar.md). That note names the live regexes in
`scripts/check-roadmap-status-drift.py`. `gated` is detail after an accepted
bucket, not a status by itself.

## Generation Shape

Generations normally collect 30-50 numbered tasks (`gNN.NNN` files directly
under `gNN/`). There is no nested card level. A phase boundary
does not imply a generation rollover.
