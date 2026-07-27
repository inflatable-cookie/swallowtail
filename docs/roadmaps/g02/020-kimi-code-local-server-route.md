# 020 Kimi Code Local Server Route

Status: complete
Owner: Tom
Created: 2026-07-27
Depends on: g02.015
Vision tags: transport diversity, provider-session lifecycle, prepared facade
Contract refs: 005-011, 014, 017, 029, 032-033, 037-038
Planning state: cards 061-065 completed

## Problem

Kimi Code supports persistent sessions through ACP, but its selected ACP route
does not expose provider-session archive, restore, or delete. Current Kimi Code
also documents a separate foreground local server used by its Web UI. That
server exposes REST, WebSocket, OpenAPI, AsyncAPI, and reversible archive and
restore.

Swallowtail needs a separate local-server driver. It must not relabel the
surface as ACP, treat the server bearer as Kimi account access, infer hard
deletion, or make consumers assemble process, endpoint, credential, version,
and lifecycle details manually.

## Goals

- [x] Freeze exact `0.28.1` and `0.29.0` local-server protocol evidence.
- [x] Implement attached and owned-foreground host composition without a
      container.
- [x] Qualify inactive-session archive and restore without claiming deletion.
- [x] Import an ACP-created session only through explicit cross-transport
      authority checks.
- [x] Add the Kimi local-server interactive route as a later, separate tranche.
- [x] Prove the route through packaged artifacts without publication.

## Non-Goals

- changing `kimi-code.acp`
- reading or mutating Kimi session files directly
- disabling local-server authentication
- hard deletion
- selecting a Kimi account, model, endpoint, or state root implicitly
- claiming a harness sandbox
- editing Nucleus or Soundcheck

## Execution Plan

### Batch 20.1 — Exact Server Corpus

- [x] Execute card 061.

### Batch 20.2 — Lifecycle Driver

- [x] Execute card 062 after card 061 closes.

### Batch 20.3 — Binding Import And Lifecycle Conformance

- [x] Execute card 063 after card 062 closes.

### Batch 20.4 — Interactive Driver

- [x] Execute card 064 after lifecycle conformance passes.

### Batch 20.5 — Provider-Wide Acceptance

- [x] Execute card 065 after the interactive route closes.

## Acceptance Criteria

- [x] driver, transport, executable, server API, WebSocket protocol, endpoint,
      configured instance, state root, credential, and provider session remain
      separate identities
- [x] exact `0.28.1` and `0.29.0` behavior is qualified; later stable releases
      remain visibly unverified
- [x] archive and restore use native REST operations and require an inactive
      bound target
- [x] delete is unsupported before effects
- [x] owned foreground work joins the child; attached work preserves it
- [x] token values, session ids, paths, and payloads stay out of diagnostics
- [x] ACP import cannot succeed from a raw id or mismatched host, version,
      state root, endpoint, credential, or target
- [x] interactive execution preserves Kimi-specific lifecycle and event
      behavior without a lowest-common-denominator prompt API
- [x] package and route-matrix evidence passes without provider access or
      publication

## Decision Gates

- Stop card 061 if either qualified release lacks a stable metadata, auth,
  archive, restore, or WebSocket boundary.
- Stop card 062 if the bearer cannot be obtained through an opaque host
  credential lease without appearing in diagnostics.
- Stop card 063 if shared-state identity cannot be proven without adapter-owned
  filesystem inspection.
- Keep interactive execution planned if the lifecycle route proves useful but
  the larger WebSocket surface remains too volatile.

## Next Planning Checkpoint

Operator review now decides whether to authorize the separate card 060 Nucleus
lifecycle handoff. ACP and local server remain independently supported; neither
is a fallback or universal recommendation.
