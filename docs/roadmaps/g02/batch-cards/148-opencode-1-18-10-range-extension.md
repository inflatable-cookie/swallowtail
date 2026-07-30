# 148 OpenCode 1.18.10 Range Extension

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../044-installed-harness-range-maintenance.md`

## Goal

Extend the OpenCode HTTP/SSE guarantee through exact `1.18.10` without
flattening the unrelated `1.18.8` artifact delta into a protocol milestone.

## Scope

1. Add exact release and artifact records for `1.18.5..=1.18.10`.
2. Extend execution, lifecycle, deletion, continuity, callback, usage,
   generation-control, activity, discovery, and prepared fixtures.
3. Preserve the existing `surface-18`, `delete-02`, and `runtime-02`
   revisions where exact selected source is unchanged.
4. Record `1.18.8`'s unrelated OAuth callback field in full-artifact evidence.
5. Retain a later stable unverified-newer classification.

## Acceptance Criteria

- [x] all six exact stable releases are corpus entries
- [x] selected execution and lifecycle behavior remains unchanged
- [x] deletion remains provider-data deletion with provider-defined descendants
- [x] `1.18.8` does not create a false selected-surface milestone
- [x] discovery and prepared operations report `1.18.10` as maintained
- [x] later stable and prerelease classifications remain distinct
- [x] focused OpenCode tests and warnings-denied clippy pass

## Validation

- focused OpenCode compatibility, HTTP, prepared, lifecycle, callback,
  activity, usage, and generation-control tests
- `cargo clippy -p swallowtail-adapter-opencode --all-targets -- -D warnings`
- no live attached-server mutation
- no broad workspace suite

## Stop Conditions

- Stop if exact source changes selected auth, lifecycle, deletion, cancellation,
  or cleanup truth.
- Stop if a downstream provider transform would need a Swallowtail protocol
  claim.
- Do not start, update, or mutate the installed OpenCode server.

## Auto-Continuation

Yes. Continue to card 149 after focused validation.

## Evidence

- exact npm publication, tag-commit, and OpenAPI records cover
  `1.18.5..=1.18.10`
- all selected route schemas remain on `surface-18`, `delete-02`, and
  `runtime-02`
- `1.18.8` records its unrelated OAuth callback artifact delta without a
  dispatch milestone; `1.18.9` records the revert
- 82 adapter tests pass across protocol, compatibility, HTTP/SSE, prepared,
  lifecycle, callback, continuity, activity, usage, and generation-control
  coverage
- warnings-denied adapter clippy, route matrices, docs QA, and
  `git diff --check` pass
- no live server prompt, mutation, installation, or consumer edit ran
