# 001 Release And Package Evidence

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../001-release-boundary-and-package-readiness.md`

## Objective

Freeze the current workspace, consumer, and official Cargo evidence needed to
choose Swallowtail's first release boundary without changing manifests or
publishing anything.

## Governing Refs

- Vision 001
- repository authority map
- g01 generation-disposition log
- Contracts 001, 002, 004, 005, and 029
- roadmap g02.001
- current Cargo package, workspace, dependency, SemVer, and Rust-version
  documentation

## Scope

1. Inventory all 23 crates:
   - package identity and metadata
   - runtime, development, and build dependency direction
   - path and registry requirements
   - public exports, features, targets, examples, fixtures, and generated files
   - plausible public, support, and intentionally unpublished roles
2. Record the internal dependency DAG and candidate publication orders.
3. Inventory changelog, tags, release automation, package ownership,
   documentation, license, repository, and registry configuration.
4. Inspect Nucleus and Soundcheck read-only for their Swallowtail pins, Rust
   toolchains, selected crates, and upgrade constraints.
5. Revalidate current official Cargo rules for:
   - package and publish validation
   - path plus registry dependency requirements
   - workspace metadata inheritance
   - pre-1.0 SemVer compatibility
   - `rust-version` and MSRV expectations
6. Compare:
   - all consumer-usable crates versus a smaller initial publication set
   - unified versus independent crate versions
   - latest-stable versus bounded-window MSRV
   - manual-first versus automated release preparation
7. Create Research 033 and provisional Spec 004 with explicit recommendations,
   unresolved choices, and promotion targets.
8. Rebaseline card 002 only if the evidence leaves no hidden authority gap.

## Acceptance Criteria

- [x] every workspace crate has one explicit candidate publication role
- [x] internal dependency and candidate publication order are exact
- [x] consumer evidence is read-only and repository authority remains intact
- [x] provider-interface versions remain separate from crate versions
- [x] MSRV and SemVer recommendations cite current official Cargo rules
- [x] no package set, version model, or release mutation is silently selected
- [x] Research 033 and Spec 004 state all remaining operator choices
- [x] card 002 is ready only when contract promotion needs no fresh evidence

## Validation

- `cargo metadata --no-deps --format-version 1`
- manifest, public-export, fixture, license, and package-content inventory
- read-only consumer dependency and toolchain inspection
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- a crate's publication would expose secrets, private fixtures, or unsupported
  dependencies
- crate ownership or registry authority is unknown
- consumer toolchain evidence conflicts materially
- the version or package model remains a product-policy tie
- evidence would require a publish, tag, credential, workflow, or consumer
  mutation

## Auto-Continuation

Yes, only into card 002 when the research and provisional spec make contract
promotion mechanical. Otherwise return the exact policy choice to the
operator.

## Evidence

- Research 033 inventories all 23 public library candidates, exact normal and
  development dependency direction, public crate-root markers, package lists,
  fixtures, metadata, and resolved Rust-version floors.
- The candidate publication DAG has three stages: core and protocols; runtime;
  then host, testkit, remote transport, and adapters.
- Nucleus and Soundcheck read-only evidence shows the same four sibling-path
  dependencies and no explicit Rust toolchain floor.
- Current crates.io API checks found all 23 exact names absent but unreserved.
- The repository has no tag, GitHub release, registry policy, or Effigy release
  task.
- Provisional Spec 004 records one recommended package, version, MSRV,
  registry, and human-authority bundle.
- Documentation and Northstar QA pass. `git diff --check` passes. Effigy doctor
  remains at the inherited 19 oversized-file findings: 12 warnings and seven
  errors.

## Closeout

The evidence is complete. Card 002 remains planned because adopting the
recommended bundle is product policy. No fresh technical research is required
if the operator approves or directly amends Spec 004.
