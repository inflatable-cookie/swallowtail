# 036 Source Release And Compatibility Boundary

Status: active
Owner: Tom
Updated: 2026-08-08

## Purpose

Define Swallowtail's public package set, pre-1.0 compatibility promise, MSRV,
source-release evidence, consumer proof, and release authority.

Contract 029 separately governs provider, harness, SDK, protocol, service, and
runtime-interface versions.

## Initial Distribution

The initial external release is one annotated Git tag, `v0.1.0`, on the
canonical GitHub repository.

It is not:

- a crates.io publication
- a GitHub Release object
- a binary or sidecar bundle
- an installer
- an API 1.0 promise

All workspace packages declare `publish = false` for this release line.
Changing that posture requires a separate registry contract, package-name and
owner checks, archive evidence, and explicit operator authorization.

Consumers select exact packages from the same exact Git tag. They do not use a
moving branch, an untagged revision presented as a release, or an unpublished
registry fallback. Normal internal path dependencies remain inside the tagged
workspace checkout.

## Public Package Set

The immutable `v0.1.0` and `v0.1.1` tags contain 27 public source packages.
`v0.2.0` contains 28, adding the reviewed `swallowtail-adapter-muse` package.
The `v0.3.0` / `v0.3.1` tags keep those 28 packages and 34 production routes.
Current source adds two reviewed additive packages,
`swallowtail-adapter-command-code` and `swallowtail-idioms`, for 30 packages
and 36 production routes. Both are selected for the next source release but
belong to the prepared `v0.3.2` candidate; they remain unreleased until its
exact tag is separately authorized.

Foundations:

- `swallowtail-core`
- `swallowtail-runtime`

Support:

- `swallowtail-host-local`
- `swallowtail-idioms` (`v0.3.2` candidate)
- `swallowtail-testkit`

Protocols and transport:

- `swallowtail-protocol-acp`
- `swallowtail-protocol-openai-chat`
- `swallowtail-transport-acp-remote`

Opt-in adapters:

- `swallowtail-adapter-alibaba-model-studio`
- `swallowtail-adapter-anthropic`
- `swallowtail-adapter-antigravity`
- `swallowtail-adapter-bedrock`
- `swallowtail-adapter-claude-agent`
- `swallowtail-adapter-codex`
- `swallowtail-adapter-command-code` (`v0.3.2` candidate)
- `swallowtail-adapter-cursor`
- `swallowtail-adapter-deepseek`
- `swallowtail-adapter-gemini`
- `swallowtail-adapter-grok`
- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`
- `swallowtail-adapter-llama-cpp`
- `swallowtail-adapter-muse`
- `swallowtail-adapter-opencode`
- `swallowtail-adapter-ollama`
- `swallowtail-adapter-oh-my-pi`
- `swallowtail-adapter-openai`
- `swallowtail-adapter-pi`
- `swallowtail-adapter-qwen`
- `swallowtail-adapter-xai`

There is no umbrella crate or intentionally private implementation crate.
Every package remains separately selectable. Selecting one grants no authority
to use another, install a harness, acquire a model, start a server,
authenticate, select billing, or claim provider support.

A package addition, removal, merge, or private role requires architecture and
contract review before manifest work. Additive candidates receive explicit
source inventory and semantic API evidence before their first tag. Historical
tag package, dependency, API, route, and release-note inventories remain
immutable.

## Dependency Topology

Normal internal dependencies remain acyclic across three layers:

1. core and protocol codecs
2. runtime
3. host support, testkit, remote ACP transport, and adapters

Each normal or build internal dependency declares both the workspace path and
an ordinary compatible requirement for the coordinated version. Path-only
development dependencies are permitted.

The version requirement does not imply registry availability. It preserves
the package relationship and leaves a future registry decision explicit.

## Coordinated Pre-1.0 Version

All packages use one coordinated workspace version. The first release is
`0.1.0`.

Before 1.0:

- compatible public API and guaranteed-behavior changes advance the patch
- breaking public API or guaranteed-behavior changes advance the minor
- provider-interface qualification remains a separate Contract 029 axis

Patch-compatible changes may include additive public items, internal
refactoring, safety fixes preserving documented behavior, additive safe
diagnostics, and newly qualified provider-interface versions.

The next selected source release is `v0.3.2`. Its two additive packages,
additive routes and public items, stricter fail-closed projections, and tooling
repairs preserve the `v0.3.1` public and guaranteed-behavior baseline. No
breaking API, capability removal, range shrink, MSRV raise, or verified-target
removal is selected, so Contract 036 requires a patch rather than `v0.4.0`.

Breaking changes include removing or incompatibly changing public items,
raising MSRV, shrinking a guaranteed provider range, removing a capability or
verified target, changing route identity, or weakening lifecycle, cleanup,
access, isolation, or evidence truth.

An urgent security exception must be operator-approved and record affected
packages, compatibility loss, rollback, and upgrade path.

## Rust And Target Support

The immutable `v0.1.x` verified floors are:

- Rust `1.90.0` for every package except Bedrock
- Rust `1.94.1` for `swallowtail-adapter-bedrock`

Bedrock's historical higher floor follows its pinned AWS SDK graph. `v0.2.0`
deliberately replaces that split with one Rust `1.95.0` floor for all
packages. This is a breaking MSRV raise and therefore uses a new pre-1.0 minor
version. The `v0.3.0` candidate retains the same floor.

The `v0.3.0` candidate must pass:

- all current-source packages at Rust `1.95.0`
- the complete workspace on the selected current stable toolchain

Apple Silicon macOS is the initial verified target. Other targets may work and
remain unverified, not prohibited.

Raising a floor or removing a verified target is breaking.

## Package Metadata

Every package declares or inherits:

- version
- edition
- license
- repository
- description
- readme
- `publish = false`
- Rust version

Metadata must describe the package's real role. It must not contain local
absolute paths, credentials, private endpoints, mutable provider payloads as
authority, or consumer product policy.

## Source Contents

The tag targets one clean non-root commit already present on the canonical
branch and approved remote. The tagged tree is the release artifact.

It may contain source, bounded deterministic fixtures, public documentation,
examples, license material, the dependency lock, and release validation
scripts.

It must not contain secrets, authentication state, mutable caches, build
output, generated local release bundles, developer-local paths, or unreviewed
live provider captures.

A deterministic source bundle may be retained as evidence. It must reproduce
the exact tagged commit and cannot replace canonical Git history.

`.crate` archives, registry publication order, registry size limits, and
registry owner state are outside the initial source-tag acceptance boundary.
Historical candidate evidence remains historical and must not be presented as
the current release candidate.

## Public API And Documentation

The first tag creates the compatibility baseline for its 27 packages. An
additive post-tag package receives separate candidate API evidence until an
operator authorizes a later source release containing it. `v0.2.0` retains
the 27-package `v0.1.0` baseline and adds Muse's first baseline without
rewriting the earlier inventory. The `v0.3.0` baseline sanctions the breaking
`Option<InterfaceVersionBinding>` return from `codex_cli_binding` and
`ollama_runtime_binding`; the package and route inventories remain unchanged.

Before the tag:

- publicly reachable items are reviewed as supported API or made private
- supported API has meaningful Rustdoc
- workspace documentation builds with missing-public-documentation denied
- normal-path examples compile
- an API baseline is generated from semantic Rust API evidence, not source-line
  hashes alone
- the changelog and release notes describe the actual tagged source

Mechanical comments that repeat an identifier do not satisfy documentation.
Examples and route guides supplement Rustdoc; they do not replace it.

Subsequent compatible candidates compare against the tagged semantic baseline
and receive explicit compatible or breaking classification.

The semantic inventory uses `cargo-public-api 0.52.0` with
`nightly-2026-08-05`, all package features enabled, and blanket, auto-trait,
and auto-derived implementations omitted. That nightly exists only to produce
rustdoc JSON. It does not change the stable release compiler or either verified
Rust floor.

## Dependency And Security Evidence

A candidate requires:

- a committed dependency lock
- no known unaccepted vulnerability in the selected normal dependency graph
- an explicit license and source policy
- review of duplicate major protocol or TLS stacks where they change risk or
  maintenance cost
- currentness review for direct dependencies without blind upgrades

Security findings cannot be reclassified by omission. An accepted exception
must name reachability, affected packages, expiry or recheck condition, and
operator approval.

## Consumer Evidence

The release candidate must prove normal public paths without editing consumer
repositories by default.

Required evidence:

- an isolated external Cargo consumer using exact source identity
- deterministic prepared-facade execution for applicable route families
- complete route and feature guide coverage
- at least one operator-selected working application smoke through a normal
  authenticated product path
- exact upgrade and rollback instructions

The external Cargo smoke uses the candidate commit before tagging. After tag
creation, tag identity must resolve to that same commit.

Live credentials, provider calls, workspace writes, and consumer mutations
remain separately gated. Deterministic adapter or consumer scenarios own
repeatable lifecycle claims. A native application smoke proves integration,
not every provider behavior.

## Deterministic Candidate Gate

Credential-free release checks sit behind explicit Effigy selectors and cover:

- clean source and exact commit identity
- 30-package current-source metadata and dependency topology, kept distinct
  from the immutable 28-package `v0.2.0` / `v0.3.1` and 27-package `v0.1.x`
  baselines
- semantic public API baseline, with the 30-package `v0.3.2` candidate frozen
  separately from the immutable 28-package `v0.3.0` compatibility baseline
- denied missing public documentation
- dependency advisory, license, and source policy
- Rust `1.95.0` floor and current stable
- formatting, lint, tests, guide coverage, and examples
- external source-consumer compilation and normal-path preparation
- release-note, changelog, license, and security-policy presence

The repository-owned Effigy release configuration is authoritative for
candidate gates, version preparation, and tag execution. It targets the
virtual workspace version explicitly and carries no registry or GitHub Release
step. Swallowtail-specific scripts supply package, API, floor, security, and
external-source evidence behind that configuration.

The configuration retains Effigy's first-tag/current-version setting as
historical bootstrap authority for `v0.1.0`. Later candidates still require a
strictly greater version and an absent matching tag. The setting does not
permit a lower version, a repeated release, or a bypass around normal release
gates.

The gate performs no authenticated provider work and no external release
mutation.

## Release Authority

No manifest version, passing gate, changelog, clean commit, or generated
candidate grants authority to mutate external state.

After candidate acceptance, the operator must explicitly authorize the exact
tag action against:

- source commit
- canonical branch and remote
- tag name `v0.3.2`
- annotated tag message
- confirmation that no crate publication or GitHub Release is included

This version selection does not authorize candidate preparation, tag creation,
or push. Creating the local tag and pushing it are separate mutations unless one
approval names both. Branch push, workflow edit, crates.io publication,
GitHub Release creation, consumer edits, and provider work remain separate.

## Acceptance

- all 28 tagged packages are separately consumable from one exact source
  identity
- the 29th and 30th Command Code and idioms packages are visibly candidate-only
  and consumable only from an explicitly approved commit until tagging
- the breaking binding-helper migration is explicit and limited to Codex and
  Ollama callers
- `publish = false` prevents accidental registry publication
- internal dependency direction is exact
- package compatibility and provider-interface versions remain separate
- the Rust floor and Apple Silicon support are explicit and tested
- source contents are clean, bounded, redacted, and reproducible
- public API is reviewed, semantically baselined, and documented
- dependency and security policy passes
- deterministic QA and external source-consumer proof pass
- an accepted working-application smoke remains recorded
- release notes and consumer instructions match the candidate
- tag creation and push remain explicitly authorized external mutations
