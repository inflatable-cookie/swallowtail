# 111 v0.1.0 Source Release Readiness Audit

Status: promoted
Owner: Tom
Updated: 2026-08-05

## Question

Is the current workspace ready to become Swallowtail's first public source
release as Git tag `v0.1.0`, without crates.io publication?

## Baseline

- clean `main` at `8bd29856cbb449e1268747f6105b3bbbc3e8cca5`
- 27 workspace packages at `0.1.0`
- 33 production routes
- 41 examples
- 1,459 deterministic tests across 136 binaries
- 279,612 Rust source lines

The complete deterministic QA selector passes. The test phase completes in
10 seconds after compilation. Rust `1.90.0` passes all non-Bedrock packages;
Rust `1.94.1` passes Bedrock. Guide coverage passes for all routes and portable
feature families.

## Release Blockers

### Source-release authority drift

The accepted release contract still selected crates.io. The operator now
selects a GitHub tag only.

Current release evidence disagrees with the workspace:

- Contract 036 says 26 packages; the workspace has 27
- release architecture says 24 packages
- the release package script contains 24 packages and omits Antigravity,
  Cursor, and Oh My Pi
- the API baseline contains 26 packages and omits Oh My Pi
- dependency topology omits Oh My Pi's two internal edges
- metadata and MSRV scripts require Rust 1.93; manifests and direct floor proof
  use Rust 1.90
- the retained release note describes a historical 23-package candidate
- Effigy's generic release planner cannot parse this virtual workspace

The existing crates.io candidate machinery is stale and irrelevant to the
selected first release. It must leave the active gate instead of being
relabelled as source-tag proof.

### Bedrock legacy TLS graph

`swallowtail-adapter-bedrock` enables both the AWS SDK's modern
`default-https-client` and its legacy `rustls` feature. The legacy feature
pulls `rustls 0.21.12` and `rustls-webpki 0.101.7`.

The selected graph carries:

- `RUSTSEC-2026-0098`
- `RUSTSEC-2026-0099`
- `RUSTSEC-2026-0104`

The modern AWS client path already uses `rustls 0.23.42` and
`rustls-webpki 0.103.13`. Removing the redundant legacy feature is the narrow
candidate fix. Bedrock must then pass focused tests, its Rust floor, and an
advisory recheck.

No dependency policy file or automated advisory/license/source gate exists.
All 339 selected external packages report license metadata, but that inventory
is not enforced.

### Public API is not release-readable

All 27 crates have crate-level documentation and route guides. The public Rust
surface does not have item-level documentation.

`RUSTDOCFLAGS="-W missing_docs" cargo doc --workspace --no-deps --locked`
reports 5,897 missing-documentation warnings. The largest package totals are:

- runtime: 1,788
- core: 1,127
- testkit: 353
- ACP protocol: 263

All 27 manifests disable doctests. Compiling standalone examples partly
compensates but does not define the supported item-level contract.

The current API baseline hashes source declarations and file paths. It detects
movement, misses semantic distinctions, and is not a reliable compatibility
baseline. Before the first tag, public items must be reviewed, narrowed where
they are implementation detail, documented where supported, and captured by a
semantic API tool.

### Consumer front door is missing

The route and feature guides are deep. The root README is 877 lines and mostly
historical roadmap chronology. It does not give a short Git-tag dependency
example, package selection rule, supported Rust/target posture, or release
status.

No dedicated source-install guide exists. Release notes and the changelog are
stale. There is no `SECURITY.md`, contribution guidance, or GitHub Actions
workflow.

The Claude Agent ACP sidecar boundary is already honest: Swallowtail pins it
for repository development, while downstream applications pin their own local
npm dependency. A Rust source tag cannot embed `node_modules`. No global npm
installation is required.

## Maintainability Findings

Effigy reports 238 oversized-file findings: 22 errors, including one critical
744-code-line Kimi reconciliation module. Thirteen error-level files are
production runtime or adapter modules; the remainder are tests or test support.

The duplicate-block scan reports 679 findings, including 43 critical blocks.
The largest are exact Cursor/Grok ACP activity, turn, and connection copies,
plus Pi/Oh My Pi protocol and catalogue copies. This creates drift risk for
the same compatibility fixes that g03 repeatedly applies.

The production tree contains 264 poisoned-lock `expect` sites. They are not
ordinary input-triggered panics in current proof, but the runtime kernel does
not yet state or test a public panic posture.

These findings do not justify a pre-release rewrite. The release lane should:

1. review public surface before it freezes
2. fix any reachable panic or invariant defect found by that review
3. consolidate exact shared code only where behavior and ownership are truly
   identical
4. record remaining internal size and duplication debt explicitly

Mass decomposition without a consumer or correctness benefit would add more
release risk than it removes.

## Positive Evidence

- full deterministic QA passes
- both declared Rust floors pass
- no production unsafe blocks
- no TODO, FIXME, HACK, placeholder, or release-removal markers
- no oversized frozen protocol fixture
- all package licenses are MIT
- all selected external dependencies expose license metadata
- route and feature guide coverage is complete
- examples and prepared facades cover normal consumer paths
- source tree and remote are clean and aligned

The suite is large but not the bottleneck. Preserve behavioral coverage.
Consolidate implementations and reusable conformance helpers instead of
deleting regression tests.

## Promotion

- Contract 036 now selects the `v0.1.0` GitHub source tag, 27 public packages,
  Rust 1.90/1.94.1 floors, documented semantic API evidence, dependency policy,
  and explicit tag authority.
- release architecture now records the 27-package source topology.
- roadmap g03.043 sequences the blocking fixes, consumer surface, candidate
  gate, and explicit tag handoff.

No tag, push, GitHub Release, crate publication, consumer edit, credential use,
or provider call is authorized by this audit.
