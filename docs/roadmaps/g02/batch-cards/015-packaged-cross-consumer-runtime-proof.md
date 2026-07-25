# 015 Packaged Cross-Consumer Runtime Proof

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../006-consumer-runtime-proof-and-candidate-replacement.md`

## Objective

Prove deterministic Nucleus and Soundcheck runtime preparation against the
exact packaged Swallowtail artifacts selected for the replacement candidate.

## Governing Refs

- Contract 036-037
- completed roadmaps g02.004-g02.005
- current package and consumer handoff tooling

## Scope

1. Build the selected Swallowtail package family from one clean source
   snapshot.
2. Patch isolated consumer snapshots to exact packaged artifacts.
3. Run deterministic Nucleus catalogue, read-only session, bounded-workspace,
   and callback preparation.
4. Run deterministic Soundcheck catalogue and structured-run preparation with
   schema, attachment, reasoning, and search variants.
5. Cover missing target, malformed/incompatible version, access evidence,
   policy mismatch, host mismatch, cancellation, deadline, and cleanup.
6. Keep live credentials and provider requests outside the deterministic gate.

## Acceptance Criteria

- [x] packaged artifacts, not sibling source paths, back both proofs
- [x] consumer runtime preparation executes; compilation alone is insufficient
- [x] exact versions and expanded profiles are asserted
- [x] failure occurs before provider work where contracted
- [x] cleanup and redaction evidence pass
- [x] original consumer repositories remain untouched by isolated proof
- [x] card 016 is ready only from passing evidence

## Validation

- exact package assembly and checksum audit
- isolated consumer deterministic runtime selectors
- package dependency and forbidden-path scan
- `effigy package:candidate:consumers`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- exact source and package hashes
- consumer snapshot identities
- deterministic runtime matrix and results
- explicit absence of live credentials/provider calls
- card 016 readiness assessment

## Execution Evidence

`effigy package:candidate:consumers` now creates one transient 23-package
candidate from a clean tracked-plus-untracked source snapshot, extracts every
crate, copies Nucleus, Soundcheck, Soundcheck Library, and Signal into isolated
source snapshots, and patches the consumers to exact `=0.1.0` package
artifacts. No consumer repository is mutated.

Evidence identities:

- candidate source snapshot:
  `1df7bd7c313469c84f8067f06f9b03817638273c`
- package-checksum manifest digest:
  `07dbbfc85772e1821394f0789c12ae3bd2d72beb364a4ed61632c839c6e77c34`
- Nucleus snapshot:
  `b585625f63ceebb4b280478af31b661df68d0a25`
- Soundcheck snapshot:
  `2ced34b19c548d2f87b18d96acd8b84ef153e392`
- Soundcheck Library snapshot:
  `262e8e1a81f8b859682c73b4b6cec94b7d969874`
- Signal snapshot:
  `18be4dc2a45435283826254a7235ce6f42f9bc6a`
- consumer validation digest:
  `c0ce8d84d7e36081462969e5cb0660cd051ee9e69e210a4f7bc4272f078e476a`

Runtime matrix:

- Nucleus packaged Codex selectors: 14 passed, 2 live probes ignored
- Soundcheck packaged Codex selectors: 4 passed, 1 live probe ignored
- extracted packaged Codex adapter: 89 deterministic tests passed
- catalogue, read-only, bounded-workspace, structured exec, tools/callbacks,
  schemas, attachments, reasoning, search, exact version, access provenance,
  policy mismatch, host/target drift, incompatible/malformed/missing version,
  cancellation, deadline, joined cleanup, and redacted failures: pass
- full 23-package assembly, checksum/content audit, extracted check, and test
  compilation: pass
- packaged Codex tests use the candidate source's frozen Cargo lock: pass
- live credentials: absent
- provider calls: none

`effigy doctor` retains the known 19 oversized-file findings: seven errors and
12 warnings. No new doctor category or count appeared. `git diff --check`
passes.

Card 016 is ready.

## Stop Conditions

- a deterministic proof depends on local ambient state
- consumer source must be mutated outside an isolated snapshot
- a package differs from the intended candidate source
- runtime preparation still permits compile-only false confidence

## Auto-Continuation

Yes, only after card 016 is explicitly ready from the complete packaged
runtime evidence.
