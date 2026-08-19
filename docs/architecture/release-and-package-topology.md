# Release And Package Topology

Status: active
Owner: Tom
Updated: 2026-08-19
Realization: roadmap g02.001; g03.043; g03.059

## Boundary

Swallowtail's immutable `v0.3.2` source tag is a coordinated 30-package Rust
workspace, two packages ahead of the `v0.3.1` tag's 28 packages. Current source
adds the reviewed but unreleased `swallowtail-adapter-deepseek-harness`
package and its two routes, `deepseek-harness.jsonrpc` and
`deepseek-harness.local-server`, plus the reviewed but unreleased
`swallowtail-adapter-zcode`, `swallowtail-adapter-cline`,
`swallowtail-adapter-goose`, `swallowtail-adapter-copilot-cli`,
`swallowtail-adapter-mistral-vibe`, `swallowtail-adapter-qoder`,
`swallowtail-adapter-openhands`, `swallowtail-adapter-kiro`, and
`swallowtail-adapter-deepagents`. Current
source is 40 packages and 47 production routes in the candidate tree.
OpenHands Agent Server is deferred: that package exists without a production
route.
The immutable `v0.1.x` source tags contain 27 packages; `v0.2.0` and later tags
contain 28 after adding `swallowtail-adapter-muse`. `swallowtail-adapter-command-code`
and `swallowtail-idioms` first appear in `v0.3.2`. No crate is published to
crates.io in this release line.

Each package remains independently selectable from the tagged Git source.
There is no umbrella crate or private implementation package.

## Package Roles

Foundations:

- `swallowtail-core`
- `swallowtail-runtime`

Support:

- `swallowtail-host-local`
- `swallowtail-idioms`
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
- `swallowtail-adapter-cline`
- `swallowtail-adapter-codex`
- `swallowtail-adapter-command-code`
- `swallowtail-adapter-copilot-cli`
- `swallowtail-adapter-cursor`
- `swallowtail-adapter-deepagents`
- `swallowtail-adapter-deepseek`
- `swallowtail-adapter-deepseek-harness`
- `swallowtail-adapter-gemini`
- `swallowtail-adapter-goose`
- `swallowtail-adapter-grok`
- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`
- `swallowtail-adapter-kiro`
- `swallowtail-adapter-llama-cpp`
- `swallowtail-adapter-mistral-vibe`
- `swallowtail-adapter-muse`
- `swallowtail-adapter-opencode`
- `swallowtail-adapter-ollama`
- `swallowtail-adapter-oh-my-pi`
- `swallowtail-adapter-openai`
- `swallowtail-adapter-openhands`
- `swallowtail-adapter-pi`
- `swallowtail-adapter-qoder`
- `swallowtail-adapter-qwen`
- `swallowtail-adapter-xai`
- `swallowtail-adapter-zcode`

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

Release metadata, dependency topology, and semantic API checks distinguish the
40-package current candidate from the immutable 30-package `v0.3.2` source.
Immutable `v0.1.x` inventories retain their 27
packages and 33 routes; `v0.2.0` and `v0.3.1` retain their 28-package,
34-route inventory. Later candidates do not rewrite historical release notes,
tag contents, or evidence.

## Version And Toolchains

All packages in the current source tag share version `0.3.2`.

- unified MSRV: Rust `1.95.0`
- verified target: Apple Silicon macOS

The immutable `v0.1.x` line used Rust `1.90.0` generally and Rust `1.94.1` for
Bedrock. `v0.2.0` raised and unified the floor at Rust `1.95.0`. The `v0.3.0`
tag keeps that floor with the fail-closed optional return from the public
Codex and Ollama version-binding helpers. `v0.3.1` and `v0.3.2` are compatible
patches on that baseline. The workspace lock and Cargo resolver 3 retain
reproducible, floor-aware selection.

## Source-Tag Consumption

A consumer selects only the packages it needs:

```toml
[dependencies]
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.3.2" }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.3.2" }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.3.2" }
```

All selected packages must use the same tag. Consumers do not combine moving
branches, local paths, registry placeholders, or different Swallowtail source
identities.

Installed harnesses and application-owned sidecars remain external runtime
dependencies. In particular, the Claude Agent ACP npm sidecar is pinned by the
consuming application and resolved from its local `.bin`; it is not embedded
in a Rust source tag.

## v0.3.2 Release Shape

The release version is `0.3.2`. Its package order places
core and protocols first, idioms before runtime, then host support, testkit,
transport, and adapters. This keeps the
runtime-to-idioms dependency resolvable from independently packaged source.

The release is one clean canonical commit plus deterministic evidence:

- exact commit and parent
- clean worktree
- 30-package immutable release metadata and topology plus the 40-package
  unreleased candidate, immutable 28-package
  `v0.2.0` / `v0.3.1` and 27-package `v0.1.x` evidence
- frozen 30-package `v0.3.2` semantic API inventory plus separate reviewed
  current-source overrides for post-tag API surfaces, including the ten
  additive packages and compatible Claude Agent, Cursor, and Grok additions,
  with removals from the
  immutable 28-package `v0.3.0` baseline forbidden
- documented public API
- dependency and security policy
- MSRV and current-stable checks
- QA, examples, and guide coverage
- isolated external source-consumer proof
- current changelog, release notes, license, and security policy

Historical `.crate` candidates remain evidence for earlier registry work.
They do not constrain the current source release.

## Release Authority

Candidate preparation creates no tag or remote mutation. The operator approves
the exact commit and tag action after all evidence passes.

The initial action excludes crates.io upload and GitHub Release creation. A
future registry lane requires a new accepted boundary and fresh package
evidence.
