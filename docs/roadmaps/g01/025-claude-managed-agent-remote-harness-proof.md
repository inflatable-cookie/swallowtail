# 025 Claude Managed Agent Remote Harness Proof

Status: completed
Owner: Tom
Updated: 2026-07-21

## Purpose

Prove one provider-hosted agent harness with explicit remote-resource
ownership, durable retention, provider-managed recovery, authoritative event
history, callbacks, interruption, deletion, and joined cleanup.

## Generation Runway

Keep g01 active. It contains 25 numbered roadmaps and remains below the normal
30-50 roadmap rollover range. This lane advances provider and transport
coverage with a remote harness shape, not another direct HTTP inference route.

## Contracts

- Contract 005: Integration Identity And Transport Diversity
- Contract 006: Execution Layer And Access Boundary
- Contract 008: Runtime Registration And Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services And Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 012: Interactive Session Options And Callback Exchange
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 016: Connection-Scoped Direct Sessions And Billed Cost
- Contract 022: Provider-Managed Agent Resource And Durable Session Boundary

Research 016 selects the first-party Claude Managed Agents beta and fixes the
bounded resource-free subset. Contract 022 governs every production step.

## Goals

- [x] Add the smallest provider-neutral durable-retention, provider-recovery,
      and remote-deletion records and preflight rules.
- [x] Freeze the dated Managed Agents REST/SSE boundary with deterministic
      fixtures and no live provider access.
- [x] Implement one separately registered resource-free remote-harness driver
      inside the Anthropic integration family.
- [x] Prove callbacks, disconnect reconciliation, interruption, deletion,
      redaction, topology, and joined cleanup.

## Execution Plan

- [x] Shared records, exact protocol corpus, and loopback fixtures: card 077.
- [x] Production Managed Agents driver: card 078.
- [x] Provider-managed harness conformance and closeout: card 079.

## Cards

- `batch-cards/077-claude-managed-agent-records-and-fixtures.md` — completed
- `batch-cards/078-claude-managed-agent-driver.md` — completed
- `batch-cards/079-claude-managed-agent-conformance-and-closeout.md` — completed

## Planning Checkpoint

The first proof binds one operator-owned agent at an exact version and model,
one driver-owned limited-network environment, and one driver-owned session. It
uses a text task, bounded declared custom tools, authoritative persisted
events, one bounded history reconciliation, native interrupt, explicit durable
retention, and explicit provider-managed rescheduling.

It grants no repository, consumer file, provider filesystem, built-in tool,
external sandbox network, MCP, skill, memory, multiagent, GitHub, schedule,
webhook, or cross-process resume authority. It creates no local container and
does not select Claude as a consumer default.

After card 079, return to provider-coverage evidence. Cursor remains behind an
operator decision on repository and remote-mutation authority; remote ACP
remains behind a stable transport and reconnect authority surface.

The proof closes with ten provider-neutral profiles and one production driver
passing the same managed-harness seam under local and remote-authoritative host
identities. Full repository QA passes with 330 tests. Default validation uses
no live account, remote resource, external request, or paid inference.

## Stop Condition

Stop before production if the exact limited environment needs external host
access, the provider beta no longer exposes the frozen resource and event
boundary, operator-owned and driver-owned resources cannot be separated, or
cleanup cannot attempt session-before-environment deletion before credential
release.
