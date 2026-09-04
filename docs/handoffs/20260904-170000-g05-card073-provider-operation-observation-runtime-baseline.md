# Northstar Worker Handoff

handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator

- Role: worker
- Lane: g05.009 Card 073, Contract 061 provider-operation observation runtime baseline
- Dispatch provenance: operator-confirmed direction relayed by Chatterbox
- Promoted planning commit: `c35f9f30`
- Manifest: `docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`, `Card 073 Manifest`
- Card: `docs/roadmaps/g05/batch-cards/073-contract-061-provider-operation-observation-baseline.md`
- Base requirement: fetch `origin` before preflight; work from current `origin/main`

## Assignment

Implement the accepted Card 070 provider-operation observation gate in
`swallowtail-runtime` and `swallowtail-testkit` only. Realize the gate's names,
admission, fourth view, composer pass, fixed maximum of four, five diagnostics,
and six testkit assertions verbatim. Keep the semantic API diff additive and
fill Card 073's `## Result`. This is one reviewable Rust PR; do not merge.

## Boundaries

Owned paths:

- `crates/swallowtail-runtime/**`
- `crates/swallowtail-testkit/**`
- `CHANGELOG.md` `[Unreleased]`
- runtime and testkit public API baseline evidence
- this card's `## Result`
- `PAPERCUTS.md` append only

Forbidden: every adapter crate, `crates/swallowtail-core/**`, contracts,
architecture, the Card 070 triage note, candidate I note, census, matrices,
guides, and other shared closeout surfaces. Do not promote the gate or compile
candidate I/Card 034. No provider credentials.

## Required closeout

Return a self-contained capsule naming exact branch/worktree/head, changed
paths, the six assertions, semantic API baseline result, focused and
package-affected validation, route/contract checks named by the manifest, and
any mechanical blocker. Stop and escalate to Chatterbox for any vocabulary
deviation. Notify on finish; the Coordinator reviews and reconciles shared
closeout surfaces.
