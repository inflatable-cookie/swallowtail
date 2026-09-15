# Gemini CLI 0.59.0 currentness corpus

This secret-free identity corpus freezes official npm
`@google/gemini-cli@0.56.0`, `0.57.0`, `0.58.0`, and `0.59.0`, their GitHub
tags `v0.56.0` through `v0.59.0`, and every darwin-arm64 unsigned release
asset before Swallowtail widens the separate ACP and headless claims.

The installed host is `0.53.0`; its npm bin link is unsigned, links to
`bundle/gemini.js`, and was not changed. The published stable points after the
previous ceilings are exactly `0.57.0`, `0.58.0`, and `0.59.0`. The first
unpublished later stable is `0.59.1`; `0.60.0-preview.0` is a preview and is
ignored.

Every npm tarball reproduces its registry `integrity` and `shasum`, every
GitHub tagged source archive and darwin-arm64 asset reproduces the published
release digest, and the `0.56.0` values reproduce the historical
`gemini-cli-0.56.0` corpus byte-for-byte. No downloaded artifact was
executed, installed, or unpacked beyond static extraction.

`surface-ledger.json` freezes one deterministic tagged-source file inventory
per compared point and the exact changed-path set for each hop: 89 paths
`0.56.0..0.57.0`, 36 paths `0.57.0..0.58.0`, and 24 paths `0.58.0..0.59.0`.
No path under `packages/cli/src/acp/**` and none of the selected stream-json,
terminal, or retention sources changes anywhere in the window.

Both axes are compatible extensions:

- ACP keeps `gemini-cli.acp.v0.51.0`, baseline `0.51.0`, and raises its
  qualified ceiling to `0.59.0`. Every selected ACP source is byte-identical
  and the ACP SDK pin stays `@agentclientprotocol/sdk@0.16.1`.
- Headless keeps `gemini-cli.headless.stream-json.v1`, baseline `0.51.0`,
  and raises its qualified ceiling to `0.59.0`. Only `geminiChat.ts`
  (provider-request retry nudge placement, empty-part filtering, abort
  rollback) and the `resolveWorktreeBaseSha` git environment helper in
  `config.ts` change; selected invocation flags, stream-json event names and
  fields, terminal result shape, native exit codes, and retention sources are
  unchanged.

Workspace-trust fail-closing, the policy safety-checker declaration, the
macOS Seatbelt sandbox profiles, and the MCP OAuth metadata repair stay
unmapped: the selected headless command already passes `--skip-trust`, the
selected route does not set `GEMINI_RESTRICTED_MODE` or
`GEMINI_CLI_TRUST_WORKSPACE=false`, and the sandbox and ambient MCP surfaces
are explicitly separate.

The selected access boundary is an enterprise-owned Gemini Developer API key.
Browser login, individual-account service, prompts, authenticated sessions,
live catalogues, host replacement, and account state are outside this corpus.

No fixture contains a credential, host path, account identity, provider
payload, real prompt, or real session id.
