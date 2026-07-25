# 004 Release Package And Compatibility Boundary

Status: archived
Owner: Tom
Updated: 2026-07-24

Archived: 2026-07-24
Disposition: approved and promoted into release architecture and Contract 036

## Purpose

Resolve the product-policy choices needed to promote Swallowtail's first
release contract without publishing, tagging, changing a registry, or implying
API 1.0.

## Scope

This spec covers:

- public package set and registry target
- coordinated versus independent crate versions
- pre-1.0 compatibility and internal dependency requirements
- MSRV and resolver policy
- package, API, documentation, changelog, target, and consumer evidence
- human authority for release mutations

Provider and harness interface ranges remain governed separately by Contract
029. This spec does not change a provider route or consumer repository.

## Evidence

Research 033 found:

- 23 consumer-usable library crates and no internal tool or umbrella crate
- one exact three-stage publication DAG
- path-only internal dependencies that cannot package for a registry yet
- no declared MSRV, publish restriction, tag, GitHub release, or release task
- four path-pinned crates used by both current consumers
- no consumer toolchain or Rust-version constraint
- all 23 exact crates.io names currently absent but unreserved
- current stable Rust `1.97.1`
- resolved dependency floors at or below `1.88`, except Bedrock at `1.94.1`

## Recommended Decisions

1. Mark all 23 current crates as public, separately consumable packages.
2. Use crates.io as the initial registry.
3. Keep one coordinated workspace version through pre-1.0.
4. Use `0.1.0` for the first candidate.
5. Give internal dependencies both a local path and an ordinary compatible
   `0.1.0` registry requirement.
6. Permit patch releases only for compatible Rust API changes. Use the next
   workspace minor for breaking changes.
7. Declare a rolling N-4 Rust support window at each new minor line: `1.93`
   initially, with Bedrock at its required `1.94.1`.
8. Use resolver 3 and verify each declared floor plus current stable.
9. Treat Apple Silicon macOS as initially verified. Permit other targets as
   unverified rather than denying them.
10. Prepare and verify packages through credential-free Effigy selectors.
11. Require separate human approval for registry upload, owner changes, tags,
    pushes, GitHub releases, and consumer updates.

These recommendations do not grant publication authority.

## Required Contract Rules

Card 002 must promote:

- the exact 23-package role table and three-stage dependency order
- package metadata and content requirements
- compatible path-plus-version internal dependency rules
- coordinated pre-1.0 version and change classification
- MSRV window, exception, resolver, and test rules
- API baseline, documentation, changelog, package archive, and clean-source
  evidence
- current-stable and declared-floor validation
- verified versus unverified target language
- non-published consumer upgrade and rollback evidence
- immutable separation between crate releases and provider-interface ranges
- explicit human release authority

## Operator Decision

The operator approved the recommended bundle on 2026-07-24.

Registry credentials, owner identity, and actual publication remain unresolved
and unauthorized. Contract 036 now governs the package boundary.

## Acceptance Criteria

- [x] the package set is explicit
- [x] registry and ownership authority are separate
- [x] version coordination and dependency requirements are explicit
- [x] compatible and breaking pre-1.0 changes are distinguishable
- [x] the MSRV promise is bounded and testable
- [x] unsupported does not mean hard-denied where execution can proceed
- [x] provider-interface versions remain separate
- [x] release preparation grants no release mutation authority
- [x] operator approval is recorded before contract promotion

## Promotion Targets

- repository architecture package topology
- a new durable release and compatibility contract
- roadmap g02.001 cards 003-004
- archive after promotion
