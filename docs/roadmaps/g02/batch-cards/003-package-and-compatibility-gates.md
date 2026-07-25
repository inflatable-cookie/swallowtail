# 003 Package And Compatibility Gates

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../001-release-boundary-and-package-readiness.md`

## Governing Refs

- Contract 036
- release and package topology architecture
- Research 033
- completed card 002

## Objective

Implement the promoted package topology and deterministic local release gates
without uploading, tagging, or changing consumer repositories.

## Scope

1. Apply the 23-package metadata, crates.io publication policy, compatible
   internal version requirements, resolver 3, `1.93` general MSRV, Bedrock
   `1.94.1` exception, and package-content bounds.
2. Add deterministic package, documentation, API-change, MSRV, and dependency-
   order checks required by the release contract.
3. Route repeatable checks through Effigy.
4. Package and verify the selected graph locally in dependency order.
5. Prove all 23 selected crates are included and forbidden files stay
   excluded.
6. Keep actual upload, owner, tag, push, GitHub release, workflow, and consumer
   mutations absent.

## Acceptance Criteria

- [x] every selected crate packages from a clean local source snapshot
- [x] packaged manifests resolve only allowed registry dependencies
- [x] package contents exclude secrets, local paths, live state, and unintended
      fixtures
- [x] API, MSRV, docs, and dependency gates are deterministic
- [x] no upload, tag, credential, or workflow mutation occurs
- [x] package versions remain separate from provider-interface ranges

## Validation

- contract-selected Cargo package checks
- package-content and generated-manifest audit
- API compatibility and MSRV checks
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- a package requires unpublished or unauthorized dependencies
- deterministic checks require release credentials
- workflow changes are necessary without explicit approval

## Auto-Continuation

No. Card 004 must be revalidated against the completed package evidence.

## Completion Evidence

- all 23 manifests inherit coordinated `0.1.0`, edition 2024, MIT,
  repository, README, crates.io publication, and bounded Rust-version metadata
- all 46 internal normal dependency edges use local workspace paths plus
  compatible `^0.1.0` registry requirements
- resolver 3 and the exact three-stage dependency topology pass the metadata
  gate
- deterministic public-declaration, documentation, and MSRV gates pass at
  Rust `1.93.0`, Bedrock `1.94.1`, and current stable `1.97.1`
- all 23 packages assemble from an isolated clean Git snapshot; archive paths,
  generated manifests, and extracted content pass the forbidden-source and
  secret audit
- the extracted package family passes workspace check and full test
  compilation with the locked dependency graph
- full repository QA passes with 658 tests inventoried: 654 pass and four
  separately gated probes remain ignored
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  seven errors
- package checksums remain temporary; card 004 owns immutable candidate
  evidence
- no registry, credential, owner, tag, push, workflow, release, or consumer
  mutation occurred
