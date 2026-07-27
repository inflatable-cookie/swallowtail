# 021 Model Catalogue Coverage

Status: completed
Owner: Tom
Created: 2026-07-27
Depends on: g02.007-g02.011
Vision tags: provider discovery, prepared facade, version ranges
Contract refs: 005-011, 014, 020, 029, 032-033, 037
Planning state: cards 066-070 completed

## Problem

The solution feature matrix reported thirteen catalogue `No` values. The first
audit found six ready interfaces, two ACP session-negotiated sources, two
not-applicable routes, one caller-supplied source, and two unresolved
surfaces. A closure audit then found qualified Qwen and Alibaba control-plane
interfaces. Every machine-readable source now has a Swallowtail path.

## Goals

- [x] Classify every current catalogue `No` by source and timing.
- [x] Add the first missing dedicated harness catalogue through Pi RPC.
- [x] Add Kimi local-server attached catalogue coverage.
- [x] Preserve ACP model options as session evidence, not hidden discovery
      sessions.
- [x] Add reusable hosted catalogue branches for OpenAI, Gemini, and xAI
      without inferring route capability.
- [x] Close the Qwen and Alibaba evidence gaps and remove every unexplained
      matrix `No`.

## Execution Plan

### Batch 21.1 — Evidence And Contract

- [x] Execute card 066.

### Batch 21.2 — Pi RPC Catalogue

- [x] Execute card 067.

### Batch 21.3 — Kimi And ACP Model Evidence

- [x] Execute card 068 after Pi closes.

### Batch 21.4 — Hosted Provider Catalogues

- [x] Execute card 069 after source-scoped HTTP catalogue reuse is reviewed.

### Batch 21.5 — No-Closure Audit

- [x] Execute card 070.

## Acceptance Criteria

- [x] every implemented catalogue binds one exact source, host, access profile,
      configured instance, interface version, deadline, and operation
- [x] catalogue observations never select a model or imply entitlement,
      invocation support, billing readiness, or route capability
- [x] no provider session is created only to populate a pre-session picker
- [x] serving-only routes remain explicitly not applicable
- [x] guaranteed compatibility retains baselines and ordered milestones;
      later stable versions remain visible unverified-newer where allowed
- [x] fixtures require no live credential, provider call, model inference, or
      publication

## Decision Gates

- Stop a route if the only model list is documentation or a consumer-supplied
  allowlist.
- Keep ACP session options separate unless an already-authorized session can
  expose them without inventing a catalogue operation.
- Do not combine provider facades merely to reuse an HTTP codec. Separate
  endpoint, access, lifecycle, and role identities remain visible.

## Next Planning Checkpoint

The catalogue lane is closed. Cards 068-070 retain separate harness, attached,
hosted, cloud-control-plane, negotiated-session, caller-supplied, and
not-applicable semantics. Card 060 remains the next operator-held adoption
decision.
