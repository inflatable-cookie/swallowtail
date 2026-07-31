# 076 Cursor ACP Installed Interactive Qualification

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Does installed Cursor Agent `2026.07.01-41b2de7` provide enough exact evidence
for Swallowtail to qualify a bounded first-party ACP interactive route without
running a live model prompt?

## Method

This pass combined:

- card 010's live initialize-only ACP capture
- read-only inspection of the exact installed bundled source
- SHA-256 identity for the relevant installed source chunks
- the stable ACP v1 method and update schemas
- current official Cursor CLI documentation
- deterministic local-authoritative and remote-authoritative host fixtures

No provider prompt, session creation, authentication call, account mutation,
workspace mutation, or network login ran. No account identity, credential, raw
tool input, or raw tool output is retained.

## Exact Artifact Evidence

The installed executable remains `2026.07.01-41b2de7`.

| Installed chunk | SHA-256 | Relevant source |
| --- | --- | --- |
| `5721.index.js` | `0332efbd33814b900e00b52753eb2b9d4ab0fa022dc264c162d2b4f535bda48f` | `cursor-acp-agent.ts`, `agent-session.ts` |
| `8096.index.js` | `139cb9c33b3464c2763044ab82b79a599ab197a27c5691dd6fce86d00f6557d5` | ACP SDK connection and schema dispatch |

The installed implementation proves:

- ACP wire version 1 over stdio NDJSON
- `session/new`, `session/prompt`, and `session/cancel`
- `end_turn` and `cancelled` prompt completion
- assistant and thought chunks
- correlated tool-call start and update messages
- plan replacements and available-command updates
- provider-owned durable session storage
- permission requests when a provider action needs approval

The initialize capture independently proves `cursor_login`, load and list
advertisement, image prompt advertisement, and HTTP/SSE MCP advertisement.
Advertisement alone still does not qualify those optional operations.

## Selected Route

The production claim is limited to `cursor-agent.acp`:

- explicit host-approved executable plus `acp`
- ambient provider-owned local login
- new durable provider session preserved on close
- text prompt turns
- assistant, thought, tool, and plan activity
- active-turn cancellation, interruption, optional deadline, and joined cleanup
- bounded filesystem read callbacks through the approved working-resource lease
- permission requests observed and cancelled without ambient approval

Swallowtail sends no `authenticate` request. It passes no consumer MCP servers,
does not select a model, and does not expose a callback exchange on this route.
Raw `rawInput` and `rawOutput` fields are excluded from stable activity and
diagnostics.

## Declined Claims

This qualification does not claim:

- load, list, resume, close, archive, restore, or delete
- image input
- consumer MCP propagation
- model discovery or model selection through ACP
- consumer-mediated question or permission callbacks
- task or child topology beyond ordinary provider-owned tool activity
- compatibility for the separate registry build `2026.07.23-e383d2b`

Later release dates retain the existing unverified-newer posture. They are not
guaranteed support and are not hard-denied solely for being newer.

## Contract Result

No shared contract change is required. Contracts 005-006 preserve route and
access identity; Contract 015 governs exact ACP negotiation; Contracts 029 and
032 govern executable qualification; Contract 033 preserves ambient harness
configuration; Contracts 044-045 bound observable activity and topology.

The source-derived normalized corpus lives beside the initialize capture under
`swallowtail-protocol-acp/tests/fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7`.
It is explicitly not represented as a live prompt transcript.

## Risks

- Cursor can change behavior across calendar builds and same-day opaque build
  hashes.
- Permission requests currently stop as provider-request observations; a later
  callback lane needs separate evidence and policy.
- Cursor advertises more lifecycle, MCP, model, and input capability than this
  driver claims.
- Provider-owned session retention persists even though Swallowtail does not
  expose lifecycle management yet.

## Primary Sources

- [Agent Client Protocol](https://github.com/agentclientprotocol/agent-client-protocol)
- [Cursor CLI overview](https://docs.cursor.com/en/cli/overview)
- [Cursor CLI usage](https://docs.cursor.com/en/cli/using)
- [Cursor CLI January 2026 changes](https://cursor.com/changelog/cli-jan-08-2026)
