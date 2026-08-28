# 005 Qoder Effective Skill Visibility Binding

Status: planned
Owner: Tom
Created: 2026-08-28
Milestone: `../002-effective-harness-skill-visibility-proof.md`
Depends on: card 004; positive Research 256 disposition

## Goal

Bind the exact Qoder roster through Contract 058 without widening structured
run authority.

## Scope

Add the bounded observation records, capability, prepared-plan agreement,
decoder mapping, identity, provenance, completeness, freshness, and safe
failure behavior admitted by Research 256. Preserve unchanged omission.

## Acceptance Criteria

- [ ] only admitted Research 256 rows are implemented
- [ ] no prompt, scan, install, mutation, or inferred provenance is added
- [ ] empty and unavailable stay distinct
- [ ] existing Qoder run behavior is unchanged when unrequested

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-qoder`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-qoder`
- `git diff --check`

## Auto-Continuation

No. Remains planned until card 004 closes positively.
