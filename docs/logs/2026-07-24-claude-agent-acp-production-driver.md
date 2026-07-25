# Claude Agent ACP Production Driver

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/144-claude-agent-acp-production-driver.md`

## Outcome

The separate `swallowtail-adapter-claude-agent` crate now implements installed
discovery and the frozen read-only Claude Agent ACP session. It does not widen
Anthropic direct inference or Managed Agents.

The driver identity is `swallowtail.claude-agent.acp`, family
`claude-agent`, transport `acp-v1-stdio`, execution layer
`HarnessInteraction`, and operation shape `InteractiveSession`.

## Version Dispatch

Discovery invokes one exact host-approved wrapper with only `--version` and
accepts only a bare semantic version. Preflight and runtime bind that observed
point to four private revisions:

1. `0.53.0` — baseline v1
2. `0.54.0..=0.59.0`, excluding `0.58.0` — session-config v2
3. `0.60.0` — provider-capability v3
4. `0.61.0` — steering-metadata v4

Stable newer versions execute through the latest-qualified private behavior
only as visible unverified points. They do not extend guaranteed support.
Excluded `0.58.0`, incompatible `0.52.0`, prereleases, malformed output, and
wrong version axes remain unavailable.

## Access And Session

The first route binds one host-approved Anthropic public-API-key lease with
audience `api.anthropic.com`. The adapter also receives one explicit approved
environment reference for the child process. It never places credential bytes
in arguments, stable diagnostics, events, or outputs.

The wrapper starts with no arguments. Initialization confirms ACP v1, exact
wrapper version, no authentication methods, and the additive capability shape
for the selected private revision. Session creation binds one exact model and
provider-native `Read`, `Glob`, and `Grep` tools against one read-only working
resource.

This is an ambient harness relay. `Ambient` configuration and `AmbientHost`
isolation are explicit. Read-tool restriction is not a sandbox claim.
Terminal auth, Claude subscription login, provider switching, persistent
sessions, writes, shells, web tools, subagents, MCP, installation, and updater
authority remain absent.

## Lifecycle

The driver maps bounded reasoning, tool, usage, progress, and output updates.
Read callbacks use the host working-resource I/O lease. Provider permission
requests are rejected with the offered one-shot rejection before ACP
cancellation and surface only as redacted provider observations.

Active cancellation and monotonic deadlines send native session cancellation.
Disconnect, malformed protocol, exact-model drift, and access mismatch stay
distinct. Prompt, deadline, protocol-pump, callback, and process work join
before the read-only resource and API-key leases release.

## Validation

- `cargo test -p swallowtail-adapter-claude-agent` — 9 passed
- `cargo test -p swallowtail-protocol-acp` — 52 passed
- `cargo clippy -p swallowtail-adapter-claude-agent --all-targets -- -D warnings`
  — passed
- `effigy check:rust`, `effigy qa:docs`, `effigy qa:northstar`, formatting,
  and `git diff --check` — passed
- `effigy doctor` — unchanged inherited 19 findings: 12 warnings, 7 errors
- no live account, external inference request, package installation, or
  container used

## Continuation

Card 145 is ready. It applies unchanged provider-neutral conformance under
local and remote-authoritative host identities, audits exclusions and
redaction, runs full repository QA, and closes roadmap 048 at a deliberate
generation checkpoint.
