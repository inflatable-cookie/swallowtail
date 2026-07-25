# Release And Package Boundary Evidence

Date: 2026-07-24

## Result

g02 card 001 completed the first exact release inventory without changing a
manifest or release surface.

Research 033 now records:

- all 23 crate roles, public front doors, targets, features, package contents,
  dependency floors, and candidate package status
- the exact three-stage publication DAG
- no local or remote tag, GitHub release, registry configuration, or Effigy
  release task
- all exact crates.io names absent but unreserved at observation time
- read-only Nucleus and Soundcheck evidence for the same four path-pinned
  crates
- current official Cargo packaging, dependency, SemVer, and Rust-version rules
- current stable Rust `1.97.1` versus local development Rust `1.96.0`

No package archive, publish dry run, upload, credential read, owner change,
tag, push, workflow, manifest, or consumer mutation occurred.

## Validation

- `cargo metadata --no-deps --format-version 1` — 23 packages
- full resolved metadata — dependency-floor inventory completed
- `cargo package --list --allow-dirty` — all 23 package contents listed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings: 12
  warnings and seven errors

No full Rust suite was run for this documentation and read-only evidence batch.

## Recommendation

Provisional Spec 004 recommends:

- all 23 crates as public packages
- crates.io as the initial registry
- one coordinated pre-1.0 `0.1.0` release train
- ordinary compatible internal `0.1.x` requirements
- an N-4 MSRV window: `1.93` initially, with Bedrock at `1.94.1`
- resolver 3 and declared-floor plus current-stable validation
- deterministic Effigy preparation with every external release mutation behind
  a separate human gate

This is one policy bundle, not an adopted contract.

## Lane State

Card 001 is completed. Card 002 remains planned because package set, registry,
version model, and MSRV are product policy. The operator can approve or amend
Spec 004 without resolving credentials or authorizing a publication.
