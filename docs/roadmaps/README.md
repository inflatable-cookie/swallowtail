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

Reassess product runway with Chatterbox after hook-owned closeout of the
Queue lifecycle lanes. No task is dispatched by this frontier; it does not
change product priority, promote planned tasks, or authorize another release.

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
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:77337a269f4af13cd5c931758542d02f37c151a5a5f24f987acbb950fcbf4fb1 -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g05 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g05.056 | complete | none | 8 | sha256:04575df830c1975942c0f05f7f53c8b5510df7908f4cd18acea8303a2c5f8166 |
<!-- northstar:lifecycle:end -->
