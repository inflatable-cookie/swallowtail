# 2026-09-15 Gemini CLI 0.59.0 Claim

g05.076 raised both separate Gemini CLI ceilings through official `0.59.0`:

- ACP: `0.51.0..=0.59.0` on `gemini-cli.acp-agent`
- headless: `0.51.0..=0.59.0` on `gemini-cli.headless-stream-json`

Both retain their claim ids, baselines, behavior revisions, and
`AllowUnverified` posture. Research 324 froze the four compared official
points and one deterministic tagged-source inventory per point before any
claim changed. No changed path lies under `packages/cli/src/acp/**`, every
selected ACP source is byte-identical with the
`@agentclientprotocol/sdk@0.16.1` pin, and both the read-only Plan Mode and
bounded-write Auto Edit profiles are unchanged. Every selected stream-json
event, terminal record, native exit code, CLI option, and retention source is
unchanged; only `geminiChat.ts` provider-request retry, empty-part, and
abort-rollback internals and the `resolveWorktreeBaseSha` `getSafeGitEnv()`
helper move.

Workspace-trust fail-closing, the policy safety-checker declaration, the macOS
Seatbelt sandbox profiles, and the MCP OAuth metadata SSRF repair stay
unmapped: the selected headless command already passes `--skip-trust`, the
selected route sets neither `GEMINI_RESTRICTED_MODE` nor
`GEMINI_CLI_TRUST_WORKSPACE=false`, and sandbox and ambient MCP are explicitly
separate surfaces. Gemini Live, the Gemini Models HTTP catalogue,
browser/individual-account auth, Vertex, and gateway stay untouched.

The historical ACP activity and headless stream-JSON decoder corpora remain in
place, and the frozen `gemini-cli-0.56.0` identity corpus stays as historical
evidence. Transcript management remains unsupported: cleanup makes one exact
delete attempt and performs no stateful list confirmation. Unpublished
`0.59.1` is the visible `UnverifiedNewer` point.

No provider operation, prompt, login, credential, catalogue call, live
session, installation, host update, or downloaded-artifact execution
occurred. PR review and merge remain queue-owned.
