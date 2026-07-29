# 136 Packaged Provider-Wide Activity Proof

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../040-provider-wide-activity-acceptance-and-consumer-handoff.md`
Depends on: card 135

## Goal

Prove the public observable-activity surface from extracted local package
artifacts across representative rich, thin, direct, realtime, and
not-applicable profiles.

## Scope

1. Assemble all local package archives without publishing.
2. Compile the extracted workspace.
3. Exercise:
   - rich Codex lifecycle
   - shared ACP lifecycle
   - non-ACP harness activity
   - completion-only headless activity
   - direct tool and reasoning-summary activity
   - realtime separation
   - catalogue and serving non-applicability
   - exact unverified-newer profile preservation
4. Run public API, metadata, docs, route, and package checks.
5. Record exact artifact and validation evidence.

## Out Of Scope

- crates.io, tags, pushes, releases, or candidate replacement
- live provider credentials
- consumer application testing
- consumer repository edits

## Acceptance Criteria

- [ ] all package archives assemble
- [ ] the extracted workspace compiles
- [ ] every selected positive profile executes from packaged artifacts
- [ ] thin and not-applicable profiles remain honest
- [ ] no raw payload or hidden reasoning appears in public evidence
- [ ] release mutation remains blocked

## Validation

- `effigy package:verify-local`
- `effigy package:candidate:facades`
- `effigy package:metadata`
- `effigy package:api`
- `effigy package:docs`
- `effigy qa`

## Stop Conditions

- Stop on any source-versus-package activity drift.
- Do not replace the held release candidate in this lane.

## Auto-Continuation

Continue to card 137 only after extracted-package acceptance passes.

