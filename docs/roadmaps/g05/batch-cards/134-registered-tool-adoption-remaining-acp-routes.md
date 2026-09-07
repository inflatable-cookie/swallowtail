# 134 Registered-Tool Adoption For Remaining ACP Routes

Status: planned; backlog stub only; no dispatch authorization
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 114 merged (the shared registered-tool kernel and bridge)

## Purpose

Give the `client_mcp_servers` producer gap on the remaining ACP routes a
durable, honest reference: `cline.acp`, `copilot-cli.acp`, `gemini-cli.acp`
(with `gemini-cli.headless`), `goose.acp`, `kiro.acp`, and `deepagents.acp`.

Card 114 built the shared registration snapshot, bridge lease, and dispatch
kernel, so the gap on these routes is no longer "the kernel does not exist".
It is that no route-local adoption has been built and no consumer has
required one. Each route would need its own exact surface evidence before
any binding, exactly as cards 116, 117, and 118 did for Claude, Codex, and
Grok.

## Scope If Promoted

Per route, and never as a batch: freeze the exact ACP MCP declaration surface
and version segment for that route; map it to the shared snapshot and bridge
without inventing a second registry or lease; prove it provider-free; then a
real-route gate under the consumer's own test authority.

## Out Of Scope

This stub authorizes no dispatch, runtime, probe, claim, or matrix change.
Its only function is to give those crosses a reference that names who owes
the work and why it is unbuilt. Promotion requires a consumer requirement and
the operator's direction, in the same way the Bovine three were promoted.

## Review Oracle

An unavailable `client_mcp_servers` cell on these routes names this card
because Swallowtail has not built route-local adoption; it must not be
reported as a provider limitation without that route's frozen evidence, and
it must not imply a scheduled lane.
