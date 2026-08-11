# 123 Claude Code Response-Only Thinking Progress

Status: promoted
Owner: Tom
Date: 2026-08-11

## Question

Can exact Claude Code `2.1.227` medium-effort response-only runs classify the
consumer-observed `system/thinking_tokens` event without exposing thought
content or weakening the route's tool-free boundary?

## Evidence

Figmatic unit `fc335758-3c1a-4bda-bb71-a8c6119fe876` reached the exact
response-only route, observed empty init tool and MCP lists, then failed safe
on `system/thinking_tokens` before terminal output.

A separately gated local Max/OAuth replay used the qualified command with
`--effort medium`, no API key, empty tools, and strict empty MCP configuration.
The exact stream reported cumulative estimates `50`, `200`, `350`, `550`,
`750`, `950`, and `1050`, with deltas `50`, `150`, `150`, `200`, `200`, `200`,
and `100`. Every frame carried the init session id. The stream then emitted
one assistant message containing a private `thinking` block with empty text
and an opaque signature, one assistant text message, and one successful result
with `num_turns=1`. Tools and MCP
servers remained empty.

The estimate is provider progress, not billed usage and not readable
reasoning. Contract 044 prohibits exposing the private thought block. Exact
validation plus discard preserves that boundary and still yields one
observable assistant text response.

## Decision

Amend Contract 039 for exact `2.1.227`. Accept positive integer cumulative
estimates up to 1,000,000 only after init and before assistant text; require
each delta to equal the cumulative increase from zero; emit content-free
coalescible `ProgressSnapshot` events. Validate and discard at most one exact
private-thinking assistant record before the one text record. Reject session,
sequence, numeric, content, or unknown-shape drift.

Keep `claude-code.headless`, route authority, capabilities, command, access,
retention, cancellation, cleanup, and safe-failure behavior unchanged.
