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

Implement
[Card 034](g05/batch-cards/034-contract-061-kimi-package-completion.md) under
[g05.009](g05/009-contract-061-consumer-projection-realization.md). It is the
sole ready card. The operator accepted the narrow `kimi-code.acp`
[active-observation public baseline](../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md):
`EffectiveReasoningSetup` is unchanged, exact reasoning and Plan values stay
adapter-local behind one bounded 128-byte Kimi provider value, one additive
`KimiPreparedSession::open_session_with_projection` covers the interactive
surface, one separate seam projects
`KimiPreparedSessionCatalogue::list_sessions`, and prepared and active source
IDs stay distinct.

Card 034 owns candidate F's complete 89-row remainder across both Kimi
packages: 25 `kimi-code.acp`, 20 `kimi-code.headless`, 31
`kimi-code.local-server`, and 13 `kimi-platform.chat`. Its four independent
ledgers expect 22/3, 10/10, 31/0, and 12/1 emitted/withheld — 75 emitted and 14
withheld. Only the two Kimi adapter semantic API baselines may change.

Do not count the 89 rows as proved before card 034 merges. Do not promote
another candidate, contact a provider, or compile Batch 9.5.

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
