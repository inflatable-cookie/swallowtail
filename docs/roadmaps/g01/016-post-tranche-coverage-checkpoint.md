# 016 Post-Tranche Coverage Checkpoint

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Re-rank the next provider proof after HTTP harness, hosted direct API, ACP, and
attached self-hosted shapes have constrained the runtime.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 007: Model Artifact and Serving Boundary
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 015: ACP v1 Negotiation And Client Callbacks

## Goals

- [x] Revalidate the highest-information remaining provider surfaces.
- [x] Select one next proof without using provider count as value.
- [x] Promote any missing contract before implementation cards.

## Current Ranking

1. xAI direct WebSocket or its current provider-supported successor
2. a second independent ACP agent
3. owned self-hosted lifecycle after the Monkey/artifact boundary is explicit
4. SDK-native lifecycle when it differs materially from proven wire routes
5. additional HTTP/SSE hosted providers and one-shot JSONL harnesses

This is a research order, not provider selection authority. Current official
evidence may reorder it.

## Execution Plan

- [x] Coverage evidence and next-proof selection: card 051.

## Cards

- `batch-cards/051-post-tranche-coverage-evidence.md` — completed

## Outcome

xAI Responses WebSocket ranks first. It adds connection-scoped direct-session
lifecycle and provider-billed-cost evidence. Kimi Code ACP `0.28.0` ranks
second, owned llama.cpp lifecycle third, and SDK-native work waits for a real
Rust embedding or an explicit language bridge.

## Stop Condition

Stop for operator direction if current evidence leaves two materially
different first choices and choosing one would establish provider, access, or
support policy.
