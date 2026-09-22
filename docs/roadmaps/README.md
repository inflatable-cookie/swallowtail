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

Map the carried-forward g06 frontier before dispatching that work. The five carried-forward tasks
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
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:12552e682d1f1baeae6fcd1e68a5965a78a2a3451671e4d9353bd4ed1a55adbe -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g06 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g06.006 | complete | none | 8 | sha256:635e26aa157bc647fa329f09a2f574f8ddd2e891da474ef878fcf2b420cf326e |
| g06.007 | complete | none | 8 | sha256:f1b2d5d544acd7d632fac663acd5dbc1e376192c239bb257f99f1ba5e8c06b23 |
| g06.008 | complete | none | 8 | sha256:68d63185abb4a3122810c5d4deee794a81ec22184b10d9fc8515bae2a1253d5e |
| g06.009 | complete | none | 8 | sha256:e7e6c3f850646b4678e08c9ad0522ecd245caaf8eeb142f06c585a0a53afd2e9 |
| g06.010 | complete | none | 8 | sha256:866915187aeaaa4a453448a80f3e35b4cf0c4624110c39344bf6254faa3a89ca |
| g06.011 | complete | none | 8 | sha256:d79a43593d0ac5f65eefb50bc51bca22a6dce1eedab68e2f21360679741922da |
| g06.012 | complete | none | 8 | sha256:ae092bb7d4ee6f8be088206020d644d515a86755011c513341f4c44eb46ac00c |
| g06.013 | complete | none | 8 | sha256:626f7ba39265aa64063a1ca5aa5be3a4c5f7c8c076689f73878a1854b0c7f5b0 |
| g06.014 | complete | none | 8 | sha256:d7c2dccee1d02541a17ba6e30f05d089fe53dce1642cbf200ae4c66d09085dc1 |
| g06.015 | complete | none | 8 | sha256:80f5e0d909ed63c33c3ac11045abc79f9c8f8e996f1992ca26f0d1cfab35018f |
| g06.016 | complete | none | 8 | sha256:f142de6a246403d77e824a3515b07e194b4719ad6b6f496e5cc4661d38b5d441 |
<!-- northstar:lifecycle:end -->
