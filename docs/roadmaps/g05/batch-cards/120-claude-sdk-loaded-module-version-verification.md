# 120 Claude SDK Loaded Module Version Verification

Status: complete; PR 269 merged at `4a27676d`
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: card 119 (shared sidecar; serial); Research 280 identity

## Goal

Close H2 from the producer diagnosis: the sidecar's `sdkVersion` is a hard-coded constant while the module path is host-supplied, so a loaded SDK that differs from the declared `0.3.259` is undetectable. Read the loaded module's `package.json` at open, compare to the declared version, and fail typed `sdk_version_mismatch` before the SDK is constructed.

## Scope

1. Sidecar: resolve the loaded module's package manifest from the host-supplied module path, read `name` and `version`, compare to the declared identity; mismatch throws `sdk_version_mismatch` with both strings (bounded) in the failure diagnostic; unreadable manifest is `sdk_identity_unverifiable`.
2. Rust: both codes in the failure vocabulary and surfaced on open; open evidence carries the loaded version string.
3. Fake-SDK fixtures: matching, mismatching, and missing manifest.
4. Guide, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Changing the qualified version; Contract 029 claims; any live turn.

## Acceptance Criteria

- [ ] loaded version verified before construction; mismatch and unverifiable are typed
- [ ] open evidence carries the loaded version
- [ ] three fixtures green; guide, changelog, baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the sidecar never reports a version it did not load. Smallest counterexample: a hard-coded version string in open evidence.

## Stop Conditions

The SDK package layout has no readable manifest at the module path (record; return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review. No live tier; a later live gate needs fixtures first and separate operator authority; never read or replay the Desktop candidate.

## Result

Implemented on the Card120 review branch. The sidecar now resolves and reads
the package manifest from the host-supplied SDK module path before calling
`sdk.query`; matching identity is carried into open evidence, mismatches emit
bounded declared/loaded identity evidence and fail `sdk_version_mismatch`, and
unreadable identity fails `sdk_identity_unverifiable`. Rust decodes both typed
codes and forwards mismatch evidence through the existing interface-version
observer path. Provider-free sidecar fixtures cover matching, mismatching, and
missing identity while Card119's model predicate, evidence, observer
forwarding, response shape, and fail-soft diagnostic write remain unchanged.
