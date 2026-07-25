# g02 Stabilization Generation Open

Date: 2026-07-24

## Decision

The operator accepted the roadmap-049 recommendation and closed g01 at 49
roadmaps. g02 is the sole active generation.

The primary programme is:

- API stabilization
- release discipline
- reproducible packaging
- consumer upgrade and rollback support

Provider breadth remains secondary and evidence-led.

## g01 Closeout

g01 closes with 48 completed roadmaps and roadmap 047 moved to the shared
backlog. Grok cards 138-141 stay with g01 as backlog evidence. Card 137's exact
corpus and provisional Spec 003 remain intact.

The backlog promotion gate still requires independently provisioned exact Grok
subscription state, matching maintained documentation, or a separate operator
access decision. No Grok release is qualified.

## g02 Baseline

The 23 workspace crates share pre-release version `0.1.0`. No release has been
published. Current manifests provide license, repository, and most
descriptions, but:

- no `rust-version` or MSRV policy exists
- no explicit publication set exists
- internal path dependencies have no registry version requirements
- one transport crate lacks a description
- no Effigy release configuration exists
- no durable contract defines crate versioning, package order, release
  authority, API-change evidence, or consumer upgrade proof

Current official Cargo documentation confirms that packaging, dependency
requirements, pre-1.0 SemVer, workspace metadata, and `rust-version` require
deliberate policy. `cargo publish --dry-run` performs checks without upload,
but even dry-run implementation waits for the release contract.

## Primary Sources

- [Cargo publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)
- [Cargo package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo dependency specification](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Cargo SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
- [Cargo Rust-version policy](https://doc.rust-lang.org/cargo/reference/rust-version.html)

## Compiled Runway

Roadmap g02.001 contains four meaningful batches:

1. release and package evidence
2. contract and publication topology
3. deterministic package and compatibility gates
4. non-published release candidate and consumer handoffs

Only card 001 is ready. Cards 002-004 remain planned. Registry uploads, tags,
pushes, workflow edits, and consumer repository changes remain unauthorized.

## Validation

- `cargo metadata --no-deps --format-version 1` — 23 packages inventoried
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `effigy doctor` — unchanged inherited 19 findings: 12 warnings, seven errors
- `git diff --check` — passed

## Next Task

Execute g02 card 001. Produce Research 033 and provisional Spec 004, then stop
for operator input if package set, version model, MSRV, or release authority
remains a genuine policy choice.
