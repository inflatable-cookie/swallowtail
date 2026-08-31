# g05.010 Bounded Skill Inventory And Effective Visibility

Status: complete; Contract 062 active; implementation unplanned
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Depends on: Contracts 058 and 062; archived Spec 013; operator four-track reframe
Vision tags: skill discovery, explicit authority, working-resource truth

## Purpose

Split two truths that the earlier runway treated as one. Swallowtail needs a
bounded inventory of skills distributed through approved global, project, and
harness-specific locations. Contract 058 remains the stronger claim about what
one exact selected harness context exposes to its model.

## Result

Card 025 promoted Spec 013 into Contract 062. The contract fixes root and
decoder authority, positive limits, deterministic traversal and containment,
descriptor-only disclosure, immutable completeness-aware snapshots, and
fail-closed composition with Contract 058. Provider-free runtime, testkit,
host-local, and adapter realization remain unplanned pending review.

## Boundary

No ambient home or project scan, provider contact, skill mutation, install,
execution, arbitrary body disclosure, or inferred effective winner belongs to
this milestone. The contract card does not implement Rust.

## Batch Cards

- [025 Bounded Skill Inventory Contract Promotion](batch-cards/025-bounded-skill-inventory-contract-promotion.md) — complete; Contract 062 active; implementation unplanned

## Acceptance

- [x] one dedicated contract owns inventory without weakening Contract 058
- [x] every root and decoder has exact ownership and positive bounds
- [x] discovery, partial, stale, conflict, and model-effective states stay distinct
- [x] implementation remains unplanned until the contract promotion is reviewed
