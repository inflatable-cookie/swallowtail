# 036 Crate Release And Compatibility Boundary

Status: active
Owner: Tom
Updated: 2026-07-24

## Purpose

Define Swallowtail's public package set, pre-1.0 compatibility promise, MSRV,
package evidence, registry boundary, and release authority.

This contract governs crate releases. Contract 029 separately governs
provider, harness, SDK, protocol, service, and runtime-interface versions.

## Public Package Set

All 23 current workspace libraries are public packages.

Foundations:

- `swallowtail-core`
- `swallowtail-runtime`

Support:

- `swallowtail-host-local`
- `swallowtail-testkit`

Protocols and transport:

- `swallowtail-protocol-acp`
- `swallowtail-protocol-openai-chat`
- `swallowtail-transport-acp-remote`

Opt-in adapters:

- `swallowtail-adapter-alibaba-model-studio`
- `swallowtail-adapter-anthropic`
- `swallowtail-adapter-bedrock`
- `swallowtail-adapter-claude-agent`
- `swallowtail-adapter-codex`
- `swallowtail-adapter-deepseek`
- `swallowtail-adapter-gemini`
- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`
- `swallowtail-adapter-llama-cpp`
- `swallowtail-adapter-opencode`
- `swallowtail-adapter-ollama`
- `swallowtail-adapter-openai`
- `swallowtail-adapter-pi`
- `swallowtail-adapter-qwen`
- `swallowtail-adapter-xai`

There is no umbrella crate or intentionally unpublished implementation crate
in this package set. A future package addition, removal, merge, or private role
requires architecture and contract review before manifest work.

Every package remains separately consumable. Publishing one grants no
authority to use another, install a harness, acquire a model, start a server,
authenticate, select a billing route, or claim provider support.

## Registry Boundary

crates.io is the initial package registry.

Registry name visibility is observation, not reservation or ownership. The
first publish is permanent and establishes external package state. Account,
owner, team, token, and final name availability must be checked separately at
the release gate.

No package may silently fall back to another registry, a Git dependency, or a
local path when registry resolution was selected.

## Dependency Topology

Publication follows three stages.

Stage 1:

- `swallowtail-core`
- `swallowtail-protocol-acp`
- `swallowtail-protocol-openai-chat`

Stage 2:

- `swallowtail-runtime`

Stage 3:

- `swallowtail-host-local`
- `swallowtail-testkit`
- `swallowtail-transport-acp-remote`
- all adapter packages

Stage 2 waits until core is visible in the registry index. Stage 3 waits until
every normal internal dependency is visible. Crates within one stage may be
ordered freely because they have no normal internal edge.

Every normal or build internal dependency must declare:

- the exact local workspace path
- an ordinary Cargo-compatible requirement for the current workspace version

The first requirement is `0.1.0`, meaning Cargo's compatible
`>=0.1.0, <0.2.0` range. Exact `=0.1.0` requirements are prohibited without a
new contract decision.

Path-only development dependencies are permitted. Generated published
manifests must resolve all normal and build dependencies through the selected
registry with no local path or unpublished package requirement.

## Coordinated Pre-1.0 Version

All packages use one coordinated workspace version. The first release
candidate is `0.1.0`.

The workspace minor is the breaking compatibility boundary before 1.0:

- a patch may contain only compatible public API and guaranteed-behavior
  changes
- a breaking public API or guaranteed-behavior change advances the workspace
  minor
- a major version remains zero until a separate API 1.0 decision

A coordinated version does not require uploading an unchanged package for
every patch. Any uploaded package must match the workspace version of the
candidate commit.

## Compatible And Breaking Changes

Patch-compatible changes may include:

- additive public items with non-exhaustive or otherwise compatible shape
- bug and safety fixes that preserve documented behavior
- internal refactoring
- newly qualified provider-interface versions
- additive diagnostics or evidence that do not expose secrets or change
  stable meanings

Breaking changes include:

- removing, renaming, or incompatibly changing a public item
- adding a required trait item or making a public type newly non-exhaustive in
  an incompatible direction
- removing or changing a default feature or target guarantee
- shrinking a guaranteed provider-interface range
- removing a capability or weakening lifecycle, cleanup, access, isolation, or
  evidence truth
- raising a package's declared Rust-version floor
- changing registry identity or making an optional route implicit

Cargo's current SemVer guidance governs unlisted Rust API cases. A deterministic
API comparison must supplement, not replace, maintainer classification.

Deprecation should precede planned removal when practical. Deprecation does not
authorize a compatibility shim, silent alias, fallback, or duplicate API.
Removal still requires the next workspace minor.

An urgent dependency or security constraint may require an exception. It must:

- be operator-approved
- identify affected packages and consumers
- state why a normal minor release is unsafe or impractical
- record the compatibility loss, rollback, and upgrade path
- never be described as an ordinary compatible patch

## Provider-Interface Separation

Crate versions and provider-interface versions are independent axes.

Contract 029 remains the authority for qualified baselines, milestones,
deprecations, exclusions, and visible unverified-newer execution. A crate
version does not imply support for every provider release. A provider release
does not force a Swallowtail release when the current adapter can run it under
the contract's unverified-newer posture.

Release notes must state material provider-range changes separately from Rust
API changes.

## Rust-Version Policy

Each new workspace minor selects a general floor from the stable Rust release
four minor versions behind current stable at that line's first candidate.

The initial floors are:

- `1.93` for every package except Bedrock
- `1.94.1` for `swallowtail-adapter-bedrock`

The Bedrock exception reflects the pinned AWS dependency floor. A package may
carry a higher floor only when current normal or build dependencies require
it and the exception is explicit.

The workspace uses resolver 3. Package manifests declare their exact
`rust-version`.

Before a release candidate:

- every package must compile and pass its contract-selected checks at its
  declared floor
- the full workspace must pass on current stable Rust
- dependency resolution at the floor must be reproducible from the candidate
  lock and manifests

The declared floor is a support promise, not a hard execution ceiling.
Unsupported older or newer toolchains may work, but are unverified. The floor
stays fixed through compatible patches unless the explicit exception process
is used.

## Supported Targets

Apple Silicon macOS is the initial verified target because that is the current
repository and consumer evidence.

Other Cargo targets are unverified, not prohibited. Swallowtail must not add a
runtime denial solely because a target is outside the verified set when the
package can otherwise compile and operate.

Adding a verified target is compatible. Removing one is breaking. Target-
specific provider, SDK, native library, harness, or runtime constraints remain
package- and route-specific.

## Package Metadata

Every public package must declare or inherit:

- version
- edition
- license
- repository
- description
- readme
- registry publication policy
- Rust-version

Documentation, homepage, keywords, and categories may be shared or
package-specific. Required metadata must describe the crate's actual role and
must not imply unavailable provider support or access.

No package metadata may contain:

- developer-local absolute paths
- credentials or credential locations
- private endpoint or account identity
- mutable provider payloads presented as stable authority
- consumer-owned product policy

## Package Contents

Package contents must be listed and audited before assembly. Final archives
must be built from a clean exact commit without `--allow-dirty`.

A publishable candidate must use the exact non-root commit already present in
the canonical branch's local history. Its source bundle preserves that commit
and ancestry. A deterministic root snapshot assembled from tracked and
untracked working-tree files remains valid package-check evidence, but it
cannot be the source of an active publication candidate.

Allowed contents include:

- package source
- Cargo-generated normalized manifest, lock, and VCS records
- public documentation and license material
- bounded deterministic tests and fixtures needed to verify the package

Forbidden contents include:

- secrets, tokens, private keys, or live authentication state
- developer-local paths or machine state
- mutable caches, build output, logs, or temporary files
- live provider captures that were not frozen, bounded, redacted, and reviewed
- unrelated consumer or repository artifacts

Every final `.crate` archive must:

- remain below the selected registry's current size limit
- have a recorded cryptographic checksum
- match the audited file list
- build and test through contract-selected checks after extraction
- resolve only allowed registry dependencies

## API, Documentation, And Changelog Evidence

The first candidate creates the public API baseline for every package.
Subsequent candidates compare against the latest released compatible baseline.

A candidate requires:

- deterministic public API change evidence
- explicit compatible or breaking classification
- successful package documentation generation
- a manually curated changelog entry
- release notes naming package, MSRV, target, provider-range, and known
  compatibility changes
- exact source commit, package list, version, archive checksum, and dependency
  order

Tool output is evidence, not release authority. A tool's failure cannot be
waived silently.

## Consumer Evidence

The release candidate must prove the selected packages without editing
consumer repositories by default.

Where the normal public path uses Contract 037 preparation, compile-only
consumer evidence is insufficient. The candidate must also run deterministic,
credential-free preparation through the packaged public API and prove its
expanded plan, access provenance, compatibility assessment, and runtime request
agreement. Live installed-binary and authentication checks remain separately
gated.

Nucleus and Soundcheck handoffs must name:

- exact package versions and source
- selected Swallowtail packages
- minimum Rust version
- validation commands and expected evidence
- provider-interface guarantees relevant to the consumer
- rollback through the prior dependency source or candidate
- known unverified targets and toolchains

A consumer edit, branch, commit, or release remains owned by that consumer and
requires separate authorization.

## Deterministic Preparation

Credential-free package, API, documentation, MSRV, content, dependency-order,
and checksum checks belong behind explicit Effigy selectors.

Preparation may use:

- `cargo metadata`
- `cargo package`
- `cargo publish --dry-run`
- deterministic API and documentation tools
- local archive extraction and build verification
- read-only registry and release-state checks

Local package verification may assemble a deterministic working-tree snapshot.
Final candidate preparation instead must:

- reject tracked or untracked source changes, excluding only generated
  candidate evidence outside the package source scope
- retain the current clean `HEAD` and its ancestry rather than creating a new
  synthetic commit
- record the exact source commit, parent, scope, and source bundle
- reproduce the same commit and archive checksums from that bundle

Preparation must not require a registry token. It must not upload, tag, push,
change an owner, create a GitHub release, or mutate a consumer.

An unpublished candidate may be marked superseded when later deterministic
evidence exposes an incomplete normal integration path. Its frozen source,
packages, hashes, and handoffs remain retained. Supersession authorizes neither
replacement freeze nor external release mutation.

## Release Authority

No automation, manifest version, changelog, clean package, passing dry run, or
release plan grants authority to mutate external release state.

After all candidate evidence passes, the operator must explicitly authorize
the exact external action against:

- source commit
- registry
- package set and publication order
- package version
- archive checksums
- owner identity
- tag and GitHub release plan

Before the first upload, the exact candidate commit must be reachable from the
canonical branch on the approved remote. The release tag must target that same
commit. Publishing from an orphan commit or from a working-tree snapshot is
prohibited.

Registry upload, owner changes, tag creation, push, GitHub release creation,
workflow edits, and consumer changes are separate mutations. One approval may
cover several only when it names the exact bounded action set.

Release credentials stay outside stable diagnostics and repository files.
Failure or registry timeout must not trigger a repeated upload without
checking external state first.

## Acceptance

- all 23 packages remain public and separately consumable
- dependency direction and publication order are exact
- pre-1.0 patch compatibility and breaking minor changes are distinct
- MSRV is explicit, bounded, tested, and separate from unverified execution
- provider-interface ranges remain independent
- package contents and metadata are reproducible and redacted
- consumer upgrade and rollback evidence is exact
- release preparation is credential-free
- active publication candidates retain canonical source history
- every external release mutation remains human-approved
