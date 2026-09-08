---
title: g05 Card 145 Desktop Claude credit-exhaustion diagnostic packet
kind: northstar-handoff
handoff_mode: external-evidence-packet
status: ready
owner: Swallowtail Chatterbox
created: 2026-09-08
updated: 2026-09-08
tags: [claude-agent-sdk, live-evidence, quota, diagnostics]
---

## What This Thread Was Doing

Preparing one Desktop-owned diagnostic open while the operator-confirmed Claude
account has no usage credit. This consumes Card 144's structured failed-open
receipt and the existing Card 132 runner.

## Why It Matters

Card 132 returned only generic `open_rejected`. The current exhausted-credit
state is a time-limited chance to determine whether the real SDK/sidecar exposes
a bounded account/usage cause or collapses it before the harness can classify
it.

## Current State

Card 144 merged at `cc53c81ad903a562830d721b39a669aee010d037`
and closed at `590f8026d114f544515f7dc33d465f17a529ec1c`.
Exact source for this diagnostic is Swallowtail
`6a93f1d916945aa2b402df7329dc994570e005c8`. The Desktop Card 132 runner
merged through PR 170. Desktop's admitted inventory contains
`claude-sonnet-5` and `claude-opus-5`; Sonnet is the cheaper admitted choice.

## Boundaries

Exactly one fresh live open. No prompt, turn, tool call, or control attempt.
Use `claude-sonnet-5`. If open succeeds, close immediately. Never retry,
reconnect, respawn, top up, qualify cells, alter the matrix, or perform release
work. Preserve Card 132 redaction rules and Card 144 receipt truth.

## Important Context

The frozen tuple remains SDK `0.3.259`, native `2.1.259`, Node `22.23.2`,
source-tagged sidecar `0.4.4`, mediated stdio/private loopback, MCP
`2025-11-25`, default permission, and persistence false. Link all Swallowtail
crates to the exact source SHA without committing the machine-local lock.

The capsule must distinguish an observed bounded account/usage subcode, a
generic construction rejection, and an open that succeeds without a turn.
None qualifies `registered_tools` or `consumer_tool_exchange`.

## Suggested Next Move

Adapt the existing Desktop runner to call
`open_route_session_with_receipt`, prove the zero-turn/one-open/redaction
oracle provider-free, then run the single diagnostic open before credit is
restored.

## Completion Protocol

Commit the redacted capsule, focused runner changes, tests, and factual log in
one Desktop PR. Independent review verifies exact source linkage, the one-open
budget, zero turns/tools/controls, receipt fidelity, cleanup, and redaction.
Return the accepted capsule identities to Swallowtail Chatterbox. No tag,
release, qualification, credited suite, or automatic continuation follows.
