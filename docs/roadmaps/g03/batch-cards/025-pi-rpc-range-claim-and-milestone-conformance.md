# 025 Pi RPC Range Claim And Milestone Conformance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../010-pi-rpc-installed-range-closure.md`
Depends on: card 024

## Goal

Extend Pi RPC's maintained claim through `0.83.0` using the frozen exact corpus
and behavior milestones.

## Scope

1. Extend the Pi package claim without moving baseline `0.80.10`.
2. Dispatch exact behavior revisions without adding operations or authority.
3. Bind discovery, preflight, prepared evidence, stream handling, activity, and
   usage to the exact planned version.
4. Cover milestone boundaries, interiors, prereleases, exclusions, malformed
   observations, and one later stable unverified point.

## Acceptance Criteria

- [x] `0.80.10..=0.83.0` membership matches the corpus exactly
- [x] every segment reports its exact behavior revision
- [x] later stable versions remain permitted but unverified
- [x] unsupported and malformed versions fail before provider work
- [x] selected retry and activity drift remains fail-closed
- [x] focused Pi and extracted-package validation pass
- [x] card 026 becomes sole ready and next

## Validation

- `effigy validate:focused swallowtail-adapter-pi`
- `effigy package:verify-affected swallowtail-adapter-pi`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 026 after focused and extracted-package proof.

## Evidence

- five maintained behavior segments cover six exact stable points
- unpublished gaps and prereleases remain incompatible
- summarization-retry events fail with stable retry-policy drift diagnostics
- exact latest prepared evidence binds the `0.83.0` behavior revision
- 41 focused tests and the extracted 85-file package passed
