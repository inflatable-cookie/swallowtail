# Roadmaps

Roadmaps sequence work after vision, architecture, and contracts provide enough
shape.

## Current Generation

- [g06 Carried Forward Route Truth And Registered Capability](g06/README.md) — active; the sole expanded generation
- [Generation Index](generation-index.md)
- [Long-Term Plan](long-term-plan.md)

Closed generations live only as roll-ups under
[archive/](archive/): [g05](archive/g05.md), [g04](archive/g04.md),
[g03](archive/g03.md), [g02](archive/g02.md), [g01](archive/g01.md).

## Next Task

Map the g06 frontier before any dispatch. The five carried-forward tasks
(g06.001–g06.005) are planned and none is dispatch-ready: g06.001 awaits the
producer proposal's independent review and canonical contract promotion,
g06.002–g06.003 stay behind the Research 256 disposition gate, and g06.004–
g06.005 lack consumer requirement and operator direction. Chatterbox
reconciles the carried set, compiles the next ready lane, and settles g06's
wider focus with the operator. Do not infer release, tag, publication,
live-provider, or consumer authority.

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
- [archive/g05.md](archive/g05.md) — completed agent-runtime-surfaces and
  route-truth generation roll-up
- [archive/g04.md](archive/g04.md) — completed route-readiness and
  connection-admission generation roll-up
- [g06/README.md](g06/README.md) — active carried-forward route-truth and
  registered-capability generation

## Status And Census

Terminal task state is lifecycle-owned in `.northstar/lifecycle/v1/` and is
projected into the front doors; the generation README's `## Tasks` section is
a flat link registry, and the generation index carries no hand-maintained
status census. Planned, pre-dispatch task files may carry a `Status:` line
whose grammar [status-grammar.md](./status-grammar.md) defines.
`scripts/check-roadmap-status-drift.py` owns the enforcement.

## Generation Shape

Generations normally collect 30-50 numbered tasks (`gNN.NNN` files directly
under `gNN/`). There is no nested card level. A phase boundary
does not imply a generation rollover.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:18346aae9670dc4b061128c98a83c46753f84fe4580bffd19dbd561bbde7ebde -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g06 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g06.006 | complete | none | 8 | sha256:635e26aa157bc647fa329f09a2f574f8ddd2e891da474ef878fcf2b420cf326e |
<!-- northstar:lifecycle:end -->
