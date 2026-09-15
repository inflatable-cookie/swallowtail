# 2026-09-15 Gemini CLI 0.59.0 Identity

Research 324 froze official npm `@google/gemini-cli` `0.56.0`, `0.57.0`,
`0.58.0`, and `0.59.0`, their GitHub tags `v0.56.0` through `v0.59.0` with
commit and tree identities, and every darwin-arm64 unsigned release asset.
Every npm tarball reproduces its registry `integrity` and `shasum`, every
GitHub asset reproduces the published digest, and every `0.56.0` value
reproduces the historical `gemini-cli-0.56.0` corpus byte-for-byte. The
installed host stays `gemini 0.53.0` at the Research 182 digest.

One deterministic tagged-source inventory per point produced the changed-path
sets directly: 89 paths `0.56.0..0.57.0`, 36 paths `0.57.0..0.58.0`, and 24
paths `0.58.0..0.59.0`. No changed path lies under `packages/cli/src/acp/**`,
and every selected stream-json, terminal, option, and retention source is
byte-identical except the internal `geminiChat.ts` and the worktree
`getSafeGitEnv()` helper in `config.ts`.

ACP is a compatible extension: byte-identical selected sources, unchanged
`@agentclientprotocol/sdk@0.16.1` pin, unchanged wire version, and unchanged
Plan and Auto Edit profiles. Headless is a compatible extension: the provider
request-loop retry nudge, empty-part filtering, and abort rollback change
internals only, with no stream-json event, terminal record, exit code, or
process-level cancellation change.

Release notes named the candidate changes. `0.57.0` moved the retry nudge and
changed abort rollback, `0.58.0` simplified the empty-text key check and
declared write-tool safety checkers, and `0.59.0` made workspace trust
fail-closed and repaired MCP OAuth metadata discovery. Workspace trust, policy
checkers, macOS Seatbelt sandboxing, and ambient MCP stay unmapped: the
selected route sets neither `GEMINI_RESTRICTED_MODE` nor
`GEMINI_CLI_TRUST_WORKSPACE=false`, and the selected headless command already
passes `--skip-trust`.

The corpus is frozen in
`crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.59.0/` with a
mutation-sensitive ledger test. No downloaded artifact was executed,
installed, or unpacked beyond static extraction, and no prompt, login,
credential, catalogue, live session, host update, release, or consumer
mutation occurred.
