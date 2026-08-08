# Release And Package Topology

Status: active
Owner: Tom
Updated: 2026-08-08
Realization: roadmap g02.001; g03.043

## Boundary

Swallowtail current source is a coordinated 28-package Rust workspace and the
`v0.3.1` candidate over the immutable `v0.3.0` tag. The immutable `v0.1.x`
source tags contain 27 packages; `v0.2.0` and later contain 28 after adding
`swallowtail-adapter-muse`. No crate is published to crates.io in this release
lane.

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
- `swallowtail-adapter-muse`
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

Candidate metadata, dependency topology, and semantic API checks include all
28 packages. Immutable `v0.1.x` inventories retain their 27 packages and 33
routes; `v0.2.0` retains its 28-package, 34-route inventory. Later candidates
do not rewrite historical release notes, tag contents, or evidence.

## Version And Toolchains

All candidate packages share version `0.3.1` after release preparation.

- unified MSRV: Rust `1.95.0`
- verified target: Apple Silicon macOS

The immutable `v0.1.x` line used Rust `1.90.0` generally and Rust `1.94.1` for
Bedrock. `v0.2.0` raised and unified the floor at Rust `1.95.0`. The `v0.3.0`
tag keeps that floor with the fail-closed optional return from the public
Codex and Ollama version-binding helpers. The `v0.3.1` candidate is a
compatible patch on that baseline. The workspace lock and Cargo resolver 3
retain reproducible, floor-aware selection.

## Source-Tag Consumption

A consumer selects only the packages it needs:

```toml
[dependencies]
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.3.1" }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.3.1" }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.3.1" }
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
- 28-package candidate metadata and topology plus immutable 27-package
  `v0.1.x` evidence
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
