# 052 Session Continuity Contract Fit And Corpora

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Do Contracts 009, 017, and 038 cover Codex app-server load, Claude Agent ACP
load and resume, and OpenCode HTTP load and resume? What exact version and
failure corpora should gate production work?

## Method

Evidence was accessed 2026-07-28.

- checked Contracts 009, 017, and 038 before designing fixtures
- compared exact tagged protocol source at each maintained boundary
- recursively closed the three selected OpenCode OpenAPI operations and local
  references across all 45 qualified releases
- checked Claude Agent ACP handler ordering at the maintained baseline and
  ceiling, then mapped every published qualified package release
- checked Codex `thread/resume` response history and the introduction of
  `excludeTurns`
- froze deterministic bounded positive and negative fixtures
- used no executable, account, credential, provider request, container, or
  model server

## Contract Fit

No shared contract change is required.

Contract 017 already requires load replay to complete in provider order before
the ready handle, requires resume to expose no replay phase, binds the exact
opaque provider session, and denies a usable handle after cancellation,
disconnect, mismatch, malformed history, or overflow. Contract 009 owns
cancellation and joined completion. Contract 038 keeps native close, archive,
restore, deletion, and local teardown independent.

The route differences stay adapter-private:

| Route | Load | Resume | Contract-private difference |
| --- | --- | --- | --- |
| Codex app-server | `thread/resume`, project `thread.turns` | same method without replay | `excludeTurns` starts at `0.129.0`; older returned turns are bounded then ignored |
| Claude Agent ACP | `session/load`, awaited update replay before response | `session/resume`, response before later non-history updates | binding includes exact session id, cwd, and MCP server values |
| OpenCode HTTP | session lookup plus bounded message pages | session lookup plus event subscription | pages arrive newest chunk first while items inside each page are chronological |

## Codex Corpus

The maintained app-server span remains `0.80.0..=0.145.0`, with unpublished
or unsupported gaps `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0`.

Six continuity segments preserve those gaps and the existing legacy/current
claim boundaries:

- `0.80.0..=0.81.0`
- `0.84.0..=0.99.0`
- `0.100.0..=0.107.0`
- `0.110.0..=0.128.0`
- `0.129.0..=0.130.0`
- `0.131.0..=0.145.0`

Default `thread/resume` returns the reconstructed thread, including ordered
turns, across the selected range. `excludeTurns` first appears at `0.129.0`.
Production resume should request it from that milestone under the existing
experimental API gate. Earlier versions may return turns; Swallowtail can
bound and ignore them without exposing a replay phase. Load keeps the default
history-bearing response and projects it before readiness.

Sources:

- [Codex `0.80.0` protocol](https://github.com/openai/codex/blob/rust-v0.80.0/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.129.0` protocol](https://github.com/openai/codex/blob/rust-v0.129.0/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex `0.145.0` protocol](https://github.com/openai/codex/blob/rust-v0.145.0/codex-rs/app-server-protocol/src/protocol/v2.rs)
- [Codex app-server guide](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)

## Claude Agent ACP Corpus

The exact range remains `0.53.0..=0.61.0`, excluding unpublished `0.58.0`.
Ten published versions qualify.

At both exact boundaries, `loadSession` gets the stored messages, converts
them in source order, awaits every `session/update`, and only then returns the
load result. `resumeSession` performs no history replay. Both use the same
session reconstruction and exact cwd/MCP fingerprint boundary. Later
`available_commands_update` notifications are configuration updates, not
history.

The corpus covers ordered load, replay-free resume, foreign session updates,
early load response, replay during resume, frame/transcript bounds,
cancellation, disconnect, native close with provider history retained, joined
process cleanup, and credential-last release.

Sources:

- [Claude Agent ACP `0.53.0`](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.53.0/src/acp-agent.ts)
- [Claude Agent ACP `0.61.0`](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [ACP session setup](https://agentclientprotocol.com/protocol/session-setup)

## OpenCode Corpus

The exact range remains all 45 qualified published releases from
`1.14.48..=1.18.4`. Recursive closure of `session.get`,
`session.messages`, and `session.prompt_async` yields seven wire surfaces and
twelve contiguous published segments. Gaps remain gaps.

Every selected message operation exposes `sessionID`, `directory`,
`workspace`, `limit`, and `before`. Swallowtail must always request a positive
limit. The server returns the newest page first and reverses entries within a
page into chronological order. Portable load therefore collects bounded pages
and emits pages oldest-first while preserving each page's item order. Resume
does not call the message-list route.

The corpus freezes exact-session checks, cursor progression, duplicate cursor,
page/item/byte overflow, missing session, foreign message session, abort,
disconnect, and attached-server cleanup without claiming server stop or
session deletion.

Sources:

- [OpenCode `1.14.48` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode `1.18.4` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)
- [OpenCode server API](https://dev.opencode.ai/docs/server/)

## Promotion

- Confirmed the selected five cells fit existing shared contracts.
- Added exact maintained-range corpora for all three routes.
- Separated history-bearing load from replay-free resume in fixtures.
- Retained Claude native-close history and OpenCode attached-server truth.
- Made card 094 production scope contract-ready without live effects.
