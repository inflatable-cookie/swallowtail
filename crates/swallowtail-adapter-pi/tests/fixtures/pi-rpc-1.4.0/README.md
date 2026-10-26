# Pi RPC 1.4.0 Identity

This fixture freezes `@earendil-works/pi-coding-agent` npm `latest` and the
GitHub latest stable release `v1.4.0`, observed on 2026-10-26. Both channels
agree on the stable point. The previous qualified ceiling is `1.1.0`. The
published stable hops are `1.2.0`, `1.3.0`, and `1.4.0`.

`identity.json` records exact npm tarball integrities, archive digests, GitHub
release and tag commits, and sourcemap-to-tag source correlation. Every
compared package has a null npm `gitHead`. Embedded `sourcesContent` for
`rpc-mode.ts`, `agent-session.ts`, `settings-manager.ts`, `resource-loader.ts`,
`args.ts`, and `grep.ts` matches the same path at each tag `v1.1.0` through
`v1.4.0`. `settings-defaults.ts` and `settings-schema.ts` match from `v1.2.0`.

`dist-inventory.json` is the complete recursive npm tree and the per-hop path
ledger. `protocol.json` classifies every changed file under the selected RPC,
lifecycle, configuration, tool, and extension paths. The selected `pi --mode rpc`
path executes the bundled CLI chunk; `dist/modes/rpc` is the unbundled copy and
is byte-identical across the four points, including the 33-command table,
dispositions, and strict-LF JSONL framing. `agent_settled.aborted` stays the
required boolean introduced at `1.1.0`. Exact `1.1.0` through `1.4.0` stay
Maintained on that revision.

The selected invocation still passes `--mode rpc`, `--no-session`, a plain
`--tools read,grep,find,ls` allowlist, and `--no-extensions` with no extension
paths, and sets steering, follow-up, compaction, and retry over RPC before any
prompt. `1.4.0` grep adds `--no-config`. Constructor tool restoration is skipped
because that allowlist makes `usesDefaultTools` false, not because of
`--no-session`. Share destinations, overflow recovery under disabled
auto-compaction, and codemode or MCP edits stay off that path.

No official artifact was executed. No provider prompt, live catalogue or RPC
session, credential use, installation, or host update occurred. Pi was not on
`PATH`. The Node version in `identity.json` is a read-only observation.
