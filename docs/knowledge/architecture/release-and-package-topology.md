# Release And Package Topology

Status: active
Owner: Tom
Updated: 2026-11-21
Realization: roadmap g02.001; g03.043; g03.059; g04.003; g05.021

## Boundary

Swallowtail's immutable `v0.3.2` source tag is a coordinated 30-package Rust
workspace. Immutable `v0.3.3` is the 40-package, 47-route tagged source: it
adds `swallowtail-adapter-deepseek-harness` and its two routes,
`deepseek-harness.jsonrpc` and `deepseek-harness.local-server`, plus
`swallowtail-adapter-zcode`, `swallowtail-adapter-cline`,
`swallowtail-adapter-goose`, `swallowtail-adapter-copilot-cli`,
`swallowtail-adapter-mistral-vibe`, `swallowtail-adapter-qoder`,
`swallowtail-adapter-openhands`, `swallowtail-adapter-kiro`, and
`swallowtail-adapter-deepagents`. OpenHands Agent Server carried no production
route; task 257 removed that package from the current source. Immutable
`v0.3.2` inventories stay
30 packages and 36 routes. `v0.4.0` kept those 40 packages and added two
routes, `pi.sdk-sidecar` and `claude-agent.sdk`, for a 49-route inventory.
`v0.5.4` tags 40 packages and 50 routes; the prepared `0.6.0` candidate keeps
the package set and adds the additive `opencode.acp` route for 51. Historical
inventories stay immutable.
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

- `swallowtail-acp-client`
- `swallowtail-protocol-acp`
- `swallowtail-protocol-openai-chat`
- `swallowtail-transport-acp-remote`
- `swallowtail-transport-http`

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
`swallowtail-protocol-acp` where they use the shared codec and on
`swallowtail-acp-client` for the shared session engine; HTTP/SSE adapters
depend on `swallowtail-transport-http` for the one bounded socket.
`swallowtail-acp-client` reaches `swallowtail-transport-acp-remote` only
through its off-by-default `remote` feature, so a stdio-only ACP adapter
compiles no Tokio, `reqwest`, or WebSocket dependency.

No normal internal edge points upward. Workspace paths keep the source tag
self-contained. Compatible version requirements preserve coordinated package
identity without claiming registry availability.

No adapter depends on `swallowtail-host-local`. A route that needs local
facilities binds them through provider-neutral runtime ports —
`RegisteredToolCourierService` for the mediated-stdio courier and
`FrozenGuardedNodeCapture` plus `ExpectedSource` for a frozen guarded capture —
and the host composition, not the adapter, owns the concrete local behavior.
`swallowtail-host-local` stays an adapter dev-dependency for tests that compose
a real local host.

Release metadata, dependency topology, and semantic API checks distinguish the
41-package current source and its 51-route inventory from the immutable
40-package, 51-route `v0.6.0` tag, the 40-package, 50-route `v0.5.4` tag, the
47-route `v0.3.3` tag, and the
30-package `v0.3.2` source.
Immutable `v0.1.x` inventories retain their 27
packages and 33 routes; `v0.2.0` and `v0.3.1` retain their 28-package,
34-route inventory. Later candidates do not rewrite historical release notes,
tag contents, or evidence.

## Version And Toolchains

All packages in the current source candidate share coordinated version `0.6.0`.
Workspace package version remains Effigy-owned until authorized prepare.

- unified MSRV: Rust `1.97.1` for the source after `v0.6.0` (the next
  pre-1.0 minor, `0.7.0`); `v0.2.0` through `v0.6.0` keep Rust `1.95.0`
- active toolchain baseline: `release-baselines/rust-toolchains-0.7.0.env`
  (`SWALLOWTAIL_MSRV=1.97.1`, `SWALLOWTAIL_CURRENT_STABLE=1.100.0`); the
  earlier env files stay immutable
- verified target: Apple Silicon macOS
- development toolchain: the root `rust-toolchain.toml` pins Rust `1.100.0`
  with rustfmt and clippy; it is separate from the MSRV floor above, which it
  does not change

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
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.5.4" }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.5.4" }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", tag = "v0.5.4" }
```

All selected packages must use the same tag. Consumers do not combine moving
branches, local paths, registry placeholders, or different Swallowtail source
identities.

Installed harnesses and application-owned sidecars remain external runtime
dependencies. In particular, the Claude Agent ACP npm sidecar is pinned by the
consuming application and resolved from its local `.bin`; it is not embedded
in a Rust source tag.

## Candidate Shape

The prepared candidate version is `0.6.0`. Its package order places
core and protocols first, idioms before runtime, then host support, testkit,
transport, and adapters. This keeps the
runtime-to-idioms dependency resolvable from independently packaged source.
`swallowtail-transport-http` sits beside `swallowtail-transport-acp-remote` in
that transport stage; every `curl` HTTP/SSE adapter depends on it for its one
bounded socket, SSE framing, timeout, cancellation and secret-header
implementation. `swallowtail-acp-client` sits alongside them in that stage and
owns the shared ACP v1 client engine, activity projection, and HTTP MCP
placement layer across all 11 ACP adapters, abstracted over both stdio
`ProcessHandle` and `RemoteAcpConnection`; the remote arm is available only
under its off-by-default `remote` feature, so the default build stays
stdio-only.

The candidate is one clean canonical commit plus deterministic evidence:

- exact commit and parent
- clean worktree
- 41-package current-source metadata and 115-edge topology, kept distinct from
  the immutable 40-package `v0.6.0`, `v0.5.4` and `v0.3.3`, 30-package
  `v0.3.2`, 28-package `v0.2.0` / `v0.3.1` and 27-package `v0.1.x` evidence
- frozen 41-package current-source semantic API inventory and 51-route
  inventory, including `opencode.acp`; historical `v0.6.0`, `v0.5.4` and
  `v0.3.3` files stay immutable, with removals from the immutable `v0.5.4`
  baseline forbidden
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
