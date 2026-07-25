# 042 Post-Forward-Compatibility Provider Coverage Checkpoint

Status: completed
Owner: Tom
Updated: 2026-07-24

## Purpose

Reassess provider and transport coverage after qualified support and
unverified-newer execution have been separated.

## Generation Runway

Keep g01 active. It contains 42 numbered roadmaps and remains inside the normal
30-50 roadmap range.

## Goals

- [x] Inventory current production drivers, operation shapes, and version
      postures.
- [x] Revalidate materially changed provider and maintained-project evidence.
- [x] Compare another range retrofit with a new transport or lifecycle proof.
- [x] Select one next tranche for architectural information, not provider
      count.
- [x] Leave one contract-ready implementation lane or request operator policy.

## Execution Plan

- [x] Post-forward-compatibility coverage evidence: card 128.

## Cards

- `batch-cards/128-post-forward-compatibility-provider-coverage-evidence.md`
  — completed

## Boundaries

- no provider implementation during the checkpoint
- qualified support and unverified-newer execution remain distinct
- no inferred range, implicit route, credential, endpoint, model, or topology
  fallback
- harness isolation remains optional and capability-scoped
- no Nucleus or Soundcheck edit
- no new generation

## Current Evidence

Swallowtail has production proofs across one-shot CLI, long-lived RPC,
attached HTTP/SSE harness, ACP, hosted HTTP/SSE and WebSocket APIs, managed
remote agents, SDK-native control planes, attached and owned self-hosted
runtimes, realtime media, and local direct continuation. Codex and OpenCode now
prove qualified ranges plus unverified-newer execution. Kimi Code remains a
candidate for capability-negotiation range evidence, but this roadmap does not
preselect it.

Research 028 inventories 21 descriptors and selects exact Kimi Code `0.28.1`
and `0.29.0` ACP segments. The releases keep ACP wire version 1 while changing
reasoning negotiation from boolean `off`/`on` to model-declared effort levels.
Contract 034 fixes typed negotiated session options without a generic provider
configuration surface. Roadmap 043 owns the implementation.
