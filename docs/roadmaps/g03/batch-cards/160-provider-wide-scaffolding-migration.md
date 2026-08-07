# 160 Provider-Wide Scaffolding Migration

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 159

## Goal

Close the scaffolding milestone with a provider-wide migration pass and
measured acceptance.

## Scope

1. Re-scan the adapter duplication families after cards 156-159 and migrate
   any remaining near-identical modules that fall under the shared helpers.
2. Run the full provider-wide evidence round: focused, affected-package,
   extracted-package, examples, public API baseline, and route/feature
   matrices.
3. Record the measured before/after duplication and the remaining
   intentionally adapter-local differences in the milestone closeout.

## Out Of Scope

- new contracts or provider-neutral vocabulary changes
- public API or behavior changes

## Acceptance

- [ ] every near-identical module under the six shared families is migrated
      or explicitly recorded as adapter-local with a reason
- [ ] the full deterministic round passes with an unchanged public API
      baseline
- [ ] the closeout records measured duplication before and after

## Stop Conditions

- stop if a migration changes public behavior or classification

## Auto-Continuation

Yes, to card 161 after acceptance.

## Validation

- `effigy qa`, `effigy package:check`, `effigy package:api`
