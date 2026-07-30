# 147 Codex 0.146 Range Extension

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../044-installed-harness-range-maintenance.md`

## Goal

Move both Codex guaranteed upper bounds from `0.145.0` to exact `0.146.0`
without changing their operation, access, lifecycle, or disclosure contracts.

## Scope

1. Promote `0.146.0` from unverified newer into the final maintained segment.
2. Update exec, app-server, lifecycle, continuity, activity, discovery, and
   prepared fixtures.
3. Preserve additive command-action and deferred-search behavior.
4. Add a later stable classification point and keep prereleases rejected.
5. Refresh exact route and compatibility guidance.

## Acceptance Criteria

- [x] exec and app-server claims end at `0.146.0`
- [x] lifecycle archive, restore, and hard-delete truth is unchanged
- [x] activity fidelity maps the exact additive `0.146.0` fields
- [x] discovery and prepared operations report `0.146.0` as maintained
- [x] a later stable point remains visible unverified newer
- [x] prereleases and existing gaps remain incompatible
- [x] focused Codex tests and warnings-denied clippy pass

## Validation

- focused Codex compatibility, discovery, prepared, lifecycle, and activity
  tests
- `cargo clippy -p swallowtail-adapter-codex --all-targets -- -D warnings`
- docs QA after public-truth edits
- no broad workspace suite

## Stop Conditions

- Stop if `0.146.0` changes selected auth, sandbox, workspace, lifecycle, or
  cleanup behavior.
- Stop if qualification needs a new public operation or compatibility shim.
- Do not run a live provider prompt.

## Auto-Continuation

Yes. Continue to card 148 only after focused validation passes.

## Evidence

- exact `0.146.0` npm, source-tag, exec-help, exec-source, stable-schema,
  experimental-schema, model-list, and lifecycle-schema digests
- 128 focused Codex adapter tests
- warnings-denied all-target Codex clippy
- provider route, lifecycle, feature, and activity matrix checks
- docs QA and `git diff --check`
