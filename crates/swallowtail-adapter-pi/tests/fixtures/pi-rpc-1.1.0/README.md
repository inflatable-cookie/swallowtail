# Pi RPC 1.1.0 Identity

This fixture freezes `@earendil-works/pi-coding-agent` npm `latest` and the
GitHub latest stable release `v1.1.0`, observed on 2026-10-08. Both channels
agree on the stable point. `identity.json` records exact npm tarball
integrities, archive digests, GitHub release and tag identities, runtime
constraints, and source correlation for the latest package. `dist-inventory.json`
contains the complete recursive npm tree and per-hop path ledgers for every
published stable point after the qualified `0.86.1` ceiling. `protocol.json`
classifies selected RPC, lifecycle, configuration, and runtime changes per hop.

The npm `1.1.0` metadata has no `gitHead`; its packaged source maps embed the
exact tagged `rpc-mode.ts` and `agent-session.ts` sources. The fixture records
their SHA-256 matches to GitHub tag `v1.1.0`. Earlier npm `gitHead` values match
their corresponding stable GitHub tag commits.

The qualified `pi.rpc` segments retain the `0.80.10` baseline and claim ID.
New private mappings start at `0.99.0` for prompt, steering, and follow-up
dispositions, and at `1.1.0` for `agent_settled.aborted`. The selected command
set, strict-LF JSONL framing, supported operations, and exclusions remain
unchanged. Published points are separated by explicit gaps; points above
`1.1.0` remain unverified.

The exact selected invocation still uses `--no-extensions` and sends no
extension paths. Although Pi 1.0.4 changed `--tools` so MCP tools are retained
unless explicitly excluded, its `--no-extensions` path omits configured and
built-in extensions, including built-in MCP. The new `--no-mcp` flag is not
needed for this exact invocation.

No official artifact was executed. No provider prompt, live catalogue or RPC
session, credential use, installation, or host update occurred. The host
version and Node runtime hashes in `identity.json` are read-only identity
observations; no host paths or credentials are retained.
