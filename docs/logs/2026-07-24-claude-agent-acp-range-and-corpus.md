# Claude Agent ACP Range And Corpus

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/143-claude-agent-acp-range-authority-and-corpus.md`

## Outcome

Card 143 qualifies Claude Agent ACP `0.53.0..=0.61.0`, excluding unpublished
`0.58.0`.

Candidate `0.52.0` is incompatible. Its argument ordering intercepts
`--cli --version` as the wrapper version, preventing nested native-binary
observation. It also predates the upstream tool-call-before-permission-request
correlation fix.

Four private behavior revisions retain additive upstream differences:

1. `0.53.0` — baseline v1
2. `0.54.0..=0.59.0`, excluding `0.58.0` — session-config v2
3. `0.60.0` — provider-capability v3
4. `0.61.0` — steering-metadata v4

Stable versions above `0.61.0` may execute only as visible unverified-newer
points using the latest qualified private behavior. They do not become
qualified automatically.

## Exact Artifact Evidence

The corpus freezes all 11 published candidate adapter releases with:

- publication time and tag commit
- ACP SDK version
- Agent SDK version
- nested native Claude Code version and SHA-256
- milestone source-file SHA-256 values

Exact Agent SDK platform packages map to Claude Code:

- `0.3.191` / `2.1.191` for rejected adapter `0.52.0`
- `0.3.195` / `2.1.195` at the qualified baseline
- `0.3.217` / `2.1.217` at latest qualified

Every intermediate published point is recorded. Native probes used exact
darwin-arm64 package tarballs, an empty `HOME`, and denied network. Each direct
`--version` created zero state files. The current binary is signed by Anthropic
team `Q6L2SF6YDW`.

Wrapper, ACP SDK, Agent SDK, nested native binary, ACP wire, provider API, and
model identities remain separate.

## Frozen Corpus

Independent raw ACP fixtures cover:

- baseline, provider-capability, and steering initialization
- no advertised terminal-auth or gateway capability
- one new session with exact model confirmation
- provider-native `Read`, `Glob`, and `Grep` selection
- reasoning, read-tool, usage, output, and terminal ordering
- tool-call-before-permission ordering
- explicit permission rejection and turn cancellation
- model drift
- safe access failure
- disconnect before terminal response
- redaction and unsupported-capability exclusions

Access stays one host-approved Anthropic public-API key. Claude subscription,
terminal auth, login, logout, gateways, provider switching, persistent
sessions, writes, Bash, web tools, subagents, terminals, MCP, elicitation,
steering, and sandbox claims remain excluded.

## Contract Result

No new shared contract is required.

The local process host clears ambient environment variables before applying
host-approved values. A hidden `CLAUDE_CODE_EXECUTABLE` override therefore
cannot replace the bundled binary. Existing ACP, access, ambient harness,
version, installed-observation, and configuration contracts cover the first
production subset.

## Validation

- `cargo test -p swallowtail-protocol-acp`: 52 passed
- `cargo clippy -p swallowtail-protocol-acp --all-targets -- -D warnings`:
  passed
- `cargo fmt --all -- --check`: passed
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed
- `git diff --check`: passed
- `effigy doctor`: unchanged inherited 19 oversized-file findings, comprising
  12 warnings and 7 errors

## Continuation

Card 144 is ready: implement the separately registered Claude Agent ACP stdio
driver for the exact qualified range and frozen public-API-key subset.
