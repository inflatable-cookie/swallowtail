# 006 Qoder Skill Visibility Acceptance

Status: planned
Owner: Tom
Created: 2026-08-28
Milestone: `../002-effective-harness-skill-visibility-proof.md`
Depends on: card 005

## Goal

Close the first Contract 058 proof with route fixtures, guide coverage, and
consumer-facing truth.

## Scope

Prove positive global and project rows when admitted, complete-empty behavior,
staleness, bounds, cancellation, cleanup, redaction, and absence behavior.
Update shared route and feature guidance only after the proof passes.

## Acceptance Criteria

- [ ] one exact effective roster is visible to a consumer
- [ ] provenance does not exclude operator-installed skills
- [ ] no ambient scan or model prompt occurs
- [ ] route matrices and guide claims match fixtures exactly

## Validation

- `effigy validate:focused swallowtail-adapter-qoder`
- `effigy package:verify-affected swallowtail-adapter-qoder`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. Remains planned until card 005 lands.
