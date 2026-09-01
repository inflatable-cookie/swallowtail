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

Answer two `kimi-code.acp` operator decisions under
[g05.009](g05/009-contract-061-consumer-projection-realization.md). No card is
ready. The
[Kimi gate](../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
stopped: four of the operator's five answers are realizable route-locally, but
the fifth is not.

1. **Provider-operation observation baseline.**
   `KimiPreparedSessionCatalogue::list_sessions` opens no session, yet
   publishing `control.provider-session-catalogue` needs
   `ActiveSessionObservation`, `PostOpenObservationOnly`, and
   `ConsumerRouteActiveSessionState` — all three defined in
   `swallowtail-runtime` as post-open *session* semantics. Either broaden that
   shared vocabulary, which is a runtime public decision with cross-route reach
   that touches Contract 061, or leave the row withheld as unrepresentable and
   accept one permanently unproved census row.
2. **Compound acknowledgement representation.** Fix a shape for
   `feature.active-session-reasoning-and-plan-ack` that preserves each half's
   state generically, without adapter downcasts and without inventing pending
   state, and that honestly represents a Plan half requested but never
   dispatched because reasoning rejected first.

Decision 1 gates candidate F as a whole; decision 2 gates any implementation
card. [Card 034](g05/batch-cards/034-contract-061-kimi-package-completion.md)
is `blocked` and retains the re-derived 89-row evidence: 74 emitted, 14
withheld, 1 undecided pending decision 1.

Coverage stays at 249 proved and 518 remaining. Do not implement Kimi, promote
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
