# 094 ACP Stable Session List Schema Delta

Status: promoted
Owner: Tom
Date: 2026-08-02

## Question

Does current stable ACP v1 still match the session-list shape selected by
Research 092, and has any additive wire behavior appeared before the common
codec is frozen?

## Method

The check compared the current official ACP v1 session-list documentation,
stabilization announcement, source schema, and source identity. No provider
process, authentication flow, session, prompt, or filesystem mutation ran.

## Finding

`session/list` remains stable under ACP wire version `1`. Listing is still
independently advertised by `agentCapabilities.sessionCapabilities.list`.
The request retains optional `cwd`, `cursor`, and `_meta`; the response retains
required `sessions` plus optional `nextCursor` and `_meta`. Each `SessionInfo`
still requires `sessionId` and absolute `cwd`, with optional title, RFC 3339
update time, and `_meta`.

The current stable schema additionally permits ordered
`additionalDirectories` on `SessionInfo` when the agent independently
advertises `sessionCapabilities.additionalDirectories`. This additive field
does not widen Swallowtail's first catalogue scope beyond one exact working
resource and grants no attachment authority.

The checked source was commit
`a5b23d65366cdad16122989b490593db7795245d`; its `schema/v1/schema.json`
SHA-256 was
`7f1fba1561163729115247df75b67aeed02085115fbc7ef0131fb01d456c08f9`.
The schema crate source declared `1.6.0` while ACP wire version remained `1`.

## Decision

Freeze the additive field behind its independent capability. Keep metadata and
unknown additive fields as bounded opaque protocol extensions whose raw values
do not enter candidate projection, `Debug`, or diagnostics. Continue to treat
list, load, resume, and deletion as separate authority surfaces.

This finding is promoted into the card 055 stable fixture and shared codec. It
does not change Contract 046 or any production adapter claim.

## Sources

- [ACP v1 session list](https://agentclientprotocol.com/protocol/v1/session-list)
- [Session List stabilization announcement](https://agentclientprotocol.com/announcements/session-list-stabilized)
- [ACP source repository](https://github.com/agentclientprotocol/agent-client-protocol)
- [Pinned v1 schema](https://raw.githubusercontent.com/agentclientprotocol/agent-client-protocol/a5b23d65366cdad16122989b490593db7795245d/schema/v1/schema.json)
