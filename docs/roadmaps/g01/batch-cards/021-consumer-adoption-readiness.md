# Consumer Adoption Readiness

Status: completed
Owner: Tom
Roadmap: 006 Codex Proof Drivers
Updated: 2026-07-19

## Goal

Compile bounded Soundcheck and Nucleus adoption plans from the proven drivers
without changing consumer repositories.

## Scope

- public dependency and host-service integration points
- incremental replacement boundaries
- product-policy exclusions
- rollback and validation plans
- next-roadmap readiness for Soundcheck first, then Nucleus

## Out Of Scope

- changes in the Soundcheck or Nucleus repositories
- compatibility shims around either consumer's current connector
- runtime API stabilization or release preparation
- non-Codex adapter implementation

## Acceptance Criteria

- each consumer plan names exact owned seams
- Swallowtail remains consumer-independent
- no wholesale source move is required
- runtime stability remains gated on later non-Codex profiles

## Governing References

- `docs/architecture/repository-authority-map.md`
- `docs/architecture/consumer-runtime-evidence.md`
- Contracts 002, 004, and 008-011
- `docs/roadmaps/long-term-plan.md`

## Validation

- each proposed seam maps to an existing public Swallowtail boundary
- each consumer-owned behavior remains explicitly downstream
- roadmap and documentation QA

## Stop Condition

Stop before writing adoption roadmaps if either consumer requires a new shared
runtime behavior not covered by active contracts. Promote that gap first.

## Closeout

- current Soundcheck and Nucleus code was inspected read-only and promoted into
  exact adoption seams in the consumer runtime evidence.
- roadmap 007 sequences structured-input contracts, local materialization and
  deadlines, Codex exec expansion, and a Soundcheck-owned handoff.
- roadmap 008 sequences session/callback contracts, Codex tool-call transport,
  topology/lifecycle proof, and a Nucleus-owned handoff.
- Soundcheck goes first because its bounded one-shot path does not require
  callback exchange or durable multi-turn continuity.
- neither plan edits a consumer, moves product policy, or claims runtime
  stability. The current shared gaps are already covered semantically by
  Contracts 004 and 008-011; card 022 makes their concrete records testable.
