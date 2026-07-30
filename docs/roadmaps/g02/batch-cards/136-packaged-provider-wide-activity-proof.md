# 136 Packaged Provider-Wide Activity Proof

Status: completed
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

- [x] all package archives assemble
- [x] the extracted workspace compiles
- [x] every selected positive profile executes from packaged artifacts
- [x] thin and not-applicable profiles remain honest
- [x] no raw payload or hidden reasoning appears in public evidence
- [x] release mutation remains blocked

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

## Evidence

- All 23 local archives assemble and compile as one extracted workspace.
- The reproducibility gate passes from an ephemeral clean source snapshot.
  No retained candidate, tag, push, publication, or release mutation occurred.
- Packaged facade evidence covers 35 suites, all 26 production routes, and ten
  representative activity profiles.
- The profiles cover rich Codex lifecycle, shared ACP, non-ACP harness,
  completion-only headless, direct tool, direct reasoning summary, realtime
  separation, catalogue and serving non-applicability, and unverified-newer
  preservation.
- Lifecycle evidence covers 14 suites and five management adapters. The exact
  route split remains five supported, three unsupported, and eighteen not
  applicable.
- Isolated Nucleus and Soundcheck compatibility checks pass without editing
  either consumer repository.
- `effigy package:verify-local`, `effigy package:candidate:facades`,
  `effigy package:metadata`, `effigy package:api`, and
  `effigy package:docs` pass.
- `effigy qa` passed docs, matrices, and the all-target workspace check. The
  operator stopped its redundant full-workspace test phase after excessive
  runtime; no failure had occurred. The extracted-package and selected
  packaged suites above are the acceptance evidence for this card.

## Auto-Continuation

Continue to card 137 only after extracted-package acceptance passes.
