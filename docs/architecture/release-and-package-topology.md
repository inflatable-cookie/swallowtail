# Release And Package Topology

Status: active
Owner: Tom
Updated: 2026-08-06
Realization: roadmap g02.001; g03.043

## Boundary

Swallowtail current source is a coordinated 28-package Rust workspace. The
immutable `v0.1.0` and `v0.1.1` Git source tags contain 27 packages. The
additive `swallowtail-adapter-muse` package is unreleased source after
`v0.1.1`. No crate is published to crates.io in this release lane.

Each package remains independently selectable from the tagged Git source.
There is no umbrella crate or private implementation package.

## Package Roles

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

Adapters:

- `swallowtail-adapter-alibaba-model-studio`
- `swallowtail-adapter-anthropic`
- `swallowtail-adapter-antigravity`
- `swallowtail-adapter-bedrock`
- `swallowtail-adapter-claude-agent`
- `swallowtail-adapter-codex`
- `swallowtail-adapter-cursor`
- `swallowtail-adapter-deepseek`
- `swallowtail-adapter-gemini`
- `swallowtail-adapter-grok`
- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`
- `swallowtail-adapter-llama-cpp`
- `swallowtail-adapter-muse` (unreleased after `v0.1.1`)
- `swallowtail-adapter-opencode`
- `swallowtail-adapter-ollama`
- `swallowtail-adapter-oh-my-pi`
- `swallowtail-adapter-openai`
- `swallowtail-adapter-pi`
- `swallowtail-adapter-qwen`
- `swallowtail-adapter-xai`

Adapters remain opt-in. Selecting one does not install its provider harness,
grant credentials, select a model, or widen route support.

## Dependency Shape

```text
core and protocol codecs
        |
        v
     runtime
        |
        v
host support, testkit, remote ACP transport, adapters
```

Compatible-chat adapters also depend on
`swallowtail-protocol-openai-chat`. ACP adapters depend on
`swallowtail-protocol-acp` where they use the shared codec.

No normal internal edge points upward. Workspace paths keep the source tag
self-contained. Compatible version requirements preserve coordinated package
identity without claiming registry availability.

Current-source metadata, dependency topology, and semantic API checks include
all 28 packages. Immutable tag inventories retain their 27 packages and 33
routes. Adding Muse does not rewrite release notes, tag contents, or historical
candidate evidence.

## Version And Toolchains

All packages share version `0.1.0` before the first tag.

- general MSRV: Rust `1.90.0`
- Bedrock MSRV: Rust `1.94.1`
- verified target: Apple Silicon macOS

The Bedrock exception follows its AWS SDK graph. The workspace lock and Cargo
resolver 3 retain reproducible, floor-aware selection.

## Source-Tag Consumption

A consumer selects only the packages it needs:

```toml
[dependencies]
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.1.0" }
```

All selected packages must use the same tag. Consumers do not combine moving
branches, local paths, registry placeholders, or different Swallowtail source
identities.

Installed harnesses and application-owned sidecars remain external runtime
dependencies. In particular, the Claude Agent ACP npm sidecar is pinned by the
consuming application and resolved from its local `.bin`; it is not embedded
in a Rust source tag.

## Candidate Shape

The release candidate is one clean canonical commit plus deterministic
evidence:

- exact commit and parent
- clean worktree
- 28-package current-source metadata and topology plus immutable 27-package
  tag evidence
- semantic public API baseline
- documented public API
- dependency and security policy
- MSRV and current-stable checks
- QA, examples, and guide coverage
- isolated external source-consumer proof
- current changelog, release notes, license, and security policy

Historical `.crate` candidates remain evidence for earlier registry work.
They are not the current candidate and do not constrain the source-tag gate.

## Release Authority

Candidate preparation creates no tag or remote mutation. The operator approves
the exact commit and tag action after all evidence passes.

The initial action excludes crates.io upload and GitHub Release creation. A
future registry lane requires a new accepted boundary and fresh package
evidence.
