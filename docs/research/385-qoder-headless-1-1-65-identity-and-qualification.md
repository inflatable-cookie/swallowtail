# Research 385: Qoder Headless 1.1.65 Identity and Qualification

Status: promoted; compatible extension of the existing private behavior.

Date: 2026-10-08
Route: `qoder.headless` on `qoder.package`.
Authority: Contract 029, the pre-v0.5.2 sweep ruling, Research 328, and the
official npm `@qoder-ai/qodercli` registry.

## Official identity

The documented npm `latest` channel was re-probed on 2026-10-08. It remains
`1.1.65`, published `2026-09-30T11:19:47.785Z`; beta remains the separate
`1.1.54-beta.1` tag. Stable `1.1.55` through `1.1.65` are all present after
the existing qualified `1.1.54` point. No stable hop is missing or withdrawn.
The next point `1.1.66` is absent from the registry metadata.

The official `1.1.65` tarball is
<https://registry.npmjs.org/@qoder-ai/qodercli/-/qodercli-1.1.65.tgz>.
Its SHA-256 is
`04e996fb9ed3b718098b477f269a32ef8aa00017b8e2c533eca3a653cd7ed824`; registry
SHA-1 and SRI values are frozen in the fixture. The archive has 30 files and
68,028,943 unpacked bytes. All twelve tarballs from `1.1.54` through `1.1.65`
were checked against npm's SHA-1 and SRI and hashed locally. The npm metadata
has no repository or commit, so the published package trees are the identity
authority. The complete relative-path, byte-size, SHA-256 and per-hop delta
ledger is in the
[`1.1.65` fixture](../../crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.65/).

Neither `qoder` nor `qodercli` is installed on this host. Node `22.23.2`
satisfies the package's `>=20.0.0` engine. No downloaded code, install script,
provider prompt, login, or live operation was executed.

## Artifact and route evidence

Every package tree contains the same 30 relative file paths. Each adjacent
hop changes `package/bundle/qodercli.js`,
`package/bundle/qoder-worker-runtime.mjs`, and `package/package.json`. The
package manifest differs only in its version and six optional platform
ripgrep pins; entrypoints, dependencies, Node floor, and postinstall name are
stable. There are no additions or removals.

The vendored `qoder-security` plugin changes on `1.1.58..1.1.59`,
`1.1.59..1.1.60`, and `1.1.64..1.1.65`. Its plugin metadata, hooks, launcher,
configuration example, and skill text remain outside the selected route. The
route does not map provider tools, plugin hooks, `skills`, `plugins`, usage, or
raw provider payloads. Research 256 and the independent skill-visibility gate
remain in force.

Across all published hops, the selected print invocation retains
`--output-format stream-json`, `--permission-mode dont_ask`,
`--max-turns 8`, `--no-session-persistence`, and `--cwd`. The CLI and worker
runtime still contain the selected stream-json, `error_max_turns`,
`error_during_execution`, `aborted_streaming`, and `num_turns` markers. At
`1.1.61`, the provider runtime adds normalization for
`terminal_reason: repeated_tool_call_denied`. It assigns a provider-private
`error_code` and preserves the result envelope fields; Swallowtail ignores that
code and projects `error_during_execution` with `is_error: true` to the
existing generic `provider_failed` diagnostic. There is no dedicated route
mapping for this reason. A deterministic decoder regression fixture freezes
that generic projection and checks that provider detail stays out of the
diagnostic.

The existing adapter continues to project assistant text and known terminal
results only; unknown stream types fail closed. The adapter-owned eight-turn
ceiling, one-child lifecycle, abort cleanup, and host deadline remain in force.

See `dist-inventory.json` for all file hashes and exact added, removed,
changed, and identical sets for each hop. See `protocol.json` for the frozen
selected markers and unmapped boundaries. These artifacts were inspected as
text and hashed; they were not executed.

## Claim decision

The selected mapped behavior is a compatible extension. Keep baseline
`1.1.54`, claim ID `qoder.headless.package-window-2`, behavior revision
`qoder.headless.stdio-stream-json-v2`, the `QualifiedOnly` posture, and the
existing empty exclusion set. Extend its maintained segment through `1.1.65`.
Do not promote Qoder skills or plugins, change the fixed eight-turn bound, or
add an operation. `1.1.66` and releases beyond the qualified segment remain
rejected under the existing posture; the older `1.1.25` point is not restored.
