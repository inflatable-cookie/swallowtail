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

Implement g05.081 Goose 1.50.0 failure-binding reopen. Qoder g05.080 is
complete at exact `1.1.54`; Goose remains the one currentness reopen lane. Do
not infer release, tag, publication, live-provider, or consumer authority.

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
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:61ecf9f66a5bc15445d556f7fb84ae76024531ca3177758355bcab71d6b5d63e -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g05 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g05.056 | complete | none | 8 | sha256:04575df830c1975942c0f05f7f53c8b5510df7908f4cd18acea8303a2c5f8166 |
| g05.057 | complete | none | 8 | sha256:cc6b735ae740daa8ff0488767cf368edb3861101a7c8d3b77fe53c8139f319ac |
| g05.058 | complete | none | 8 | sha256:f1fa7407e9be7b62ed97d5cc0f1279b54889d95431a7a0cea01d86248c844cd8 |
| g05.059 | complete | none | 8 | sha256:6faa98046ef88d5d1a7ccbcc2a60785743e8a8eb34342697a22e9eee0d906514 |
| g05.060 | complete | none | 8 | sha256:a824e3781686e49ed9e80170d350ee528bf3af6e4f87b637e780884c8e2ba906 |
| g05.061 | complete | none | 8 | sha256:9247a437714e4625d1f6b49cd55668776cb4144ca413420ac350d0b8937624fa |
| g05.062 | complete | none | 8 | sha256:17ac7e6f3ab7f6ed5e924b46f2d22574ba179fcce64fdc18344d1aa1eeb4accb |
| g05.063 | complete | none | 8 | sha256:c2b7b2f48ef4e8089a881aa3b9eee7192d736c58beb3f30955d658c0280b3a3d |
| g05.064 | complete | none | 8 | sha256:2f85f11601971f4288748d461c2992b2723579d8f0337a4ecc71f8f9f7a43863 |
| g05.065 | complete | none | 8 | sha256:d05090070bc3b985089c32299fa5ad306731815e01046a72319abc04734329e1 |
| g05.067 | complete | none | 8 | sha256:ba28e7454fff7eb5a8cd81214e02dc54e7ecc83ee8dc2dda5ea3aa1a14993ecd |
| g05.068 | complete | none | 8 | sha256:d1d741fb17688f7934629d5de8d98bfdd9c24092ce67b222891b6eef16b217b4 |
| g05.070 | complete | none | 8 | sha256:5bdd5c79c117f6439c7f63373850d2d20f25f5d991c5c0989c9e98ec700c1946 |
| g05.071 | complete | none | 8 | sha256:01ad03c7d5299ce0b32c58317f45ce5b7b80405d7d34bacd6cfe19cd7f071e08 |
| g05.072 | complete | none | 8 | sha256:f4bca8804534aad5ca5180f1f02fe4cfc55f503197d72b0f1c98b0d2c13bbf8f |
| g05.073 | complete | none | 8 | sha256:1897696db18dad0865b08214a2cd1fef6b3c9c7aff7456c4114c44d3d0d7c39d |
| g05.074 | complete | none | 8 | sha256:b4320ee873482331483ba376a936a91737dccad7d03370108181ba37b4a8d185 |
| g05.075 | complete | none | 8 | sha256:b09624442301f7bee77ce0ee7054de621398cd18f0f079a61a9161b64017ada8 |
| g05.076 | complete | none | 8 | sha256:187ec9d3870cbdb9c9b86030732c8ae37dc814f81eca8d3c774df7cee717b2f9 |
| g05.077 | complete | none | 8 | sha256:5a8821d0ed913ccf1b0a57060f3c790cdc92880c6e9b3406f5a8f947567170a0 |
| g05.078 | complete | none | 8 | sha256:0290231269ee25ff6c9d39c4a54c0fece5f8aecbfa4a2316e09e420db2f57ac1 |
| g05.079 | complete | none | 8 | sha256:1d44609db92ba01f2312a46873dbc6b57faddc4b276537085a922645c0d04090 |
| g05.081 | complete | none | 8 | sha256:32dcdfcb3b98b7898541b18da2436164af08dc27b29adf5e11439042a7b3d382 |
<!-- northstar:lifecycle:end -->
