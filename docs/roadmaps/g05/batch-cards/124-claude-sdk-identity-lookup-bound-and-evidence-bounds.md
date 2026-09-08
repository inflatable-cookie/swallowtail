# 124 Claude SDK Identity Lookup Bound And Evidence Bounds

Status: complete; PR 271 merged at `02003ecb`
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: card 120 merged (`4a27676d`); card 121; the card 120 review's two precision findings

## Goal

Two bounded corrections from the card 120 review, both in the sidecar and
its Rust mirror, before they ship in `v0.4.4`.

1. **Lookup bound.** The loaded SDK manifest lookup walks ancestor
   directories to the filesystem root. If the module's own `package.json`
   is missing, an unrelated ancestor manifest (a consumer's own
   `package.json` above `node_modules`) is read and the result is
   classified `sdk_version_mismatch`, bypassing card 120's
   `sdk_identity_unverifiable` stop semantics. Bound the lookup to the
   module directory and at most the nearest `node_modules/<scope>/<name>`
   package boundary; anything else is `sdk_identity_unverifiable`.
2. **Evidence bounds.** The control-character bound applied to model ids in
   the card 119 evidence differs between the sidecar and the Rust decoder.
   Make one bound authoritative (the Rust decoder's, since it is the last
   gate before the observer), have the sidecar apply the same predicate, and
   prove agreement with a shared fixture table.

## Scope

Sidecar manifest resolution and its Rust failure mapping; the model-evidence
sanitiser on both sides; fixtures: unrelated-ancestor manifest classifies
unverifiable; module-local manifest still verifies; the control-character
table agrees byte-for-byte across sidecar and Rust.

## Out Of Scope

Any change to the declared version, the qualification claim, or the
observer vocabulary; any live turn.

## Acceptance Criteria

- [ ] an unrelated ancestor manifest yields `sdk_identity_unverifiable`, never `sdk_version_mismatch`
- [ ] one control-character predicate, applied identically on both sides, proved by a shared table
- [ ] existing card 119 and 120 fixtures unchanged and green
- [ ] changelog `[Unreleased]`; additive baseline if any; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: identity is verified only from the module's own manifest, and a
model id is admitted or rejected by exactly one predicate. Smallest
counterexample: a manifest found above `node_modules`.

## Auto-Continuation

No. Stop for exact-head review. No live tier.

## Result

Implemented the two promoted Card120 review precision fixes. The sidecar now
resolves identity only from the imported module directory through the nearest
`node_modules/<scope>/<name>` package boundary; unrelated ancestors,
malformed, unreadable, missing, and ambiguous identity stay typed
`sdk_identity_unverifiable` before `sdk.query` or provider work. A nested
package-root fixture proves the matching loaded version still reaches open
evidence, while the existing mismatch evidence and failure code remain
unchanged.

The sidecar model catalogue/evidence paths now use the Rust decoder's exact
C0/C1 `char::is_control` equivalent. One shared byte/control boundary table
drives both the provider-free sidecar requested/effective evidence checks and
Rust wire decoder checks; existing Card119/120 evidence, digest, key, count,
observer, rejection, loaded-version, and terminal-retry semantics remain
unchanged. No public API baseline changed.
