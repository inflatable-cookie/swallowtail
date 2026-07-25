# 001 Release Boundary And Package Readiness

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: completed g01
Vision tags: reusable substrate, consumer upgrade support, release discipline
Contract refs: 001, 002, 004, 005, 029
Planning state: completed

## Problem

Swallowtail has 23 pre-release crates and broad production-driver evidence, but
it has no durable rule for which crates are public, how their versions move,
which Rust toolchains they support, how internal dependencies are published,
or what consumer compatibility evidence a release requires.

The workspace is not yet an honest release unit. All crates inherit `0.1.0`,
none declares `rust-version`, internal path dependencies have no registry
version requirement, one crate lacks a description, and Effigy has no release
configuration.

## Goals

- [x] Inventory the exact public package and dependency topology.
- [x] Define pre-1.0 version, compatibility, deprecation, MSRV, and release
      authority policy from current evidence.
- [x] Promote a durable release and package contract before manifest changes.
- [x] Make the selected package set reproducibly packageable without publishing.
- [x] Establish API-change and consumer-upgrade evidence for a first candidate.

## Non-Goals

- [ ] Do not publish a crate, create a tag, push a branch, or mutate a registry.
- [ ] Do not declare API 1.0 or promise compatibility not backed by tests.
- [ ] Do not create an umbrella crate without evidence that consumers need one.
- [ ] Do not edit GitHub workflows without explicit operator approval.
- [ ] Do not edit Nucleus or Soundcheck during planning or package work.
- [ ] Do not resume the Grok backlog or select another provider.

## Contract Coverage

- [x] Contract 001 governs planning, validation, and closeout.
- [x] Contract 002 keeps repository and consumer authority separate.
- [x] Contracts 004-005 preserve runtime and crate dependency direction.
- [x] Contract 029 governs provider-interface compatibility ranges, not
      Swallowtail crate releases.
- [x] Contract 036 defines release units, public packages, internal
      dependency requirements, pre-1.0 compatibility, MSRV, release authority,
      package evidence, and consumer upgrade truth.

## Execution Plan

### Batch 1.1 — Release And Package Evidence

- [x] Execute card 001.
- [x] Freeze local workspace, dependency, metadata, public-API, consumer-pin,
      changelog, and release-automation evidence.
- [x] Revalidate official Cargo packaging, SemVer, workspace, dependency, and
      Rust-version rules.
- [x] Record unresolved product choices in a provisional spec.

### Batch 1.2 — Contract And Publication Topology

- [x] Execute card 002 after card 001 closes the evidence gap.
- [x] Promote the release boundary into architecture and a new contract.
- [x] Fix the selected public package set, version-coupling model, MSRV policy,
      dependency order, and human release gate.

### Batch 1.3 — Deterministic Package And Compatibility Gates

- [x] Execute card 003 only after contract promotion.
- [x] Apply package metadata and versioned internal dependencies.
- [x] Add deterministic package, API-change, MSRV, and documentation checks
      through Effigy where the contract requires them.
- [x] Prove the selected package graph from clean local artifacts without
      uploading.

### Batch 1.4 — Release Candidate And Consumer Upgrade Handoffs

- [x] Execute card 004 after package gates pass.
- [x] Build a non-published first release candidate and exact release plan.
- [x] Produce Nucleus and Soundcheck upgrade, rollback, and compatibility
      handoffs without editing either consumer.
- [x] Return actual publication or tag creation to an explicit operator gate.

## Acceptance Criteria

- [x] public and private crate identities are explicit
- [x] every publishable internal dependency has an exact policy-backed version
      requirement and deterministic publication order
- [x] pre-1.0 SemVer, MSRV, supported-target, deprecation, and changelog rules
      are durable and testable
- [x] package contents and generated metadata contain no secrets, local paths,
      live fixtures, or unintended artifacts
- [x] consumer upgrade evidence uses exact artifacts and preserves rollback
- [x] release mutation remains human-gated
- [x] provider compatibility ranges remain distinct from Swallowtail crate
      versioning

## Risks And Mitigations

- Risk: publishing all adapters creates unnecessary coordinated-release cost.
  Mitigation: card 001 must compare package sets and dependency ownership before
  contract selection.
- Risk: one workspace version implies stronger lockstep compatibility than
  intended. Mitigation: distinguish release coordination from API compatibility
  before choosing unified or independent versions.
- Risk: choosing an MSRV from the development toolchain breaks consumers.
  Mitigation: inventory consumer toolchains and dependency floors before
  setting `rust-version`.
- Risk: release automation outruns repository authority. Mitigation: keep all
  upload, tag, credential, and workflow mutations behind explicit approval.

## Remaining Work

No milestone work remains. Registry name, account, credential, owner, team,
publication, tag, push, and release actions remain behind one explicit
operator decision.

## Evidence Requirements

- `cargo metadata` package and dependency graph
- package manifest and crate-root public-surface inventory
- official Cargo packaging, workspace, dependency, SemVer, and `rust-version`
  rules
- current Nucleus and Soundcheck pin and toolchain evidence
- exact local package-content and dry-run results after contract promotion
- API compatibility, MSRV, docs, changelog, and consumer-upgrade results

## Decision Gate

The operator approved Spec 004. Release architecture and Contract 036 now
govern all 23 public packages, crates.io, coordinated `0.1.0`, compatible
internal requirements, the `1.93` general and `1.94.1` Bedrock floors, and
human-gated release authority.

Card 004 froze one reproducible non-published candidate and proved the exact
four-package consumer set in isolated Nucleus and Soundcheck source snapshots.
Release notes and consumer handoffs are complete. Release mutation remains
unauthorized.
