# Claude Agent ACP 0.53.0-0.61.0 Fixtures

Deterministic ACP v1 evidence for the first Claude Agent ACP maintained range.

Sources were accessed 2026-07-24. The corpus uses exact public npm metadata,
tagged repository source, package manifests, changelog entries, and maintained
mock-backed tests. Native Claude Code `--version` probes ran from exact
darwin-arm64 SDK packages with an empty `HOME` and denied network. They created
no state and made no provider request.

The qualified range begins at `0.53.0`, not `0.52.0`. Version `0.52.0`
intercepts `--cli --version` as the wrapper version and predates the tool-call
before permission-request ordering fix. Published `0.58.0` does not exist;
`0.58.1` is the next published point.

The corpus freezes four private behavior revisions:

- `0.53.0`: baseline v1 with safe wrapper and nested-binary observation
- `0.54.0..=0.59.0`: additive session-config v2, excluding `0.58.0`
- `0.60.0`: additive provider-capability v3
- `0.61.0`: additive steering-metadata v4

The portable subset stays the same across those revisions: initialize, new
session, exact model confirmation, text prompt, read-tool updates, usage,
permission rejection, cancellation, failure, disconnect, and process close.

The first access profile is one host-approved Anthropic public-API key.
Fixtures contain no credential. Terminal auth, Claude subscription login,
logout, gateways, provider switching, persistent sessions, writes, Bash, web
tools, subagents, terminals, MCP, elicitation, steering, and sandbox claims are
not supported.

These are normalized independent fixtures, not captured user traffic.
