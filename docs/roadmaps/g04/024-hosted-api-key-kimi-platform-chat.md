# 024 Hosted API-Key Kimi Platform Chat

Status: planned
Owner: Tom
Created: 2026-08-21
Depends on: completed g04.023
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 037, 047, 052, 057
Planning state: cards 076-078 ready; implementation not started
Research: 171

## Problem

`kimi-platform.chat` has a prepared hosted direct facade but is not in the
Contract 057 addable catalog. A consumer cannot yet collect its Platform API
key as a `CredentialRef`, admit an instance, and reuse
`prepare_kimi_platform_direct` through the connection lifecycle.

## Generation Runway Goal

Extend the proved hosted API-key shape by one stateless direct HTTP/SSE route.

## Goals

- [ ] expose an adapter-local hosted addable descriptor for
  `kimi-platform.chat`
- [ ] describe a secret Platform API-key field without inventing an environment
  name
- [ ] describe the approved `api.moonshot.ai` endpoint as an opaque config field
- [ ] admit the instance through Contract 057, then reuse
  `prepare_kimi_platform_direct`
- [ ] keep subject `Absent` and 047 `Ready` / `NotReady` semantics unchanged

## Non-Goals

- hosted URL-open OAuth
- Kimi Membership, Kimi Code, or Kimi local-server access
- tools, reusable sessions, provider-state management, or retry authority
- live provider, install, login, billing, or account work
- OpenHands production wiring
- changing Contract 057 or the public route matrix
- rewriting `release-baselines/public-api-0.3.3`

## Named Scope

The later implementation must keep the existing prepared route's boundaries:

- hosted topology, Credential host service, Platform API-key audience
- opaque endpoint and credential references; no raw secret or URL in records
- catalogue and one explicit K3 attempt only
- exact `kimi-platform-chat-2026-07-21` facade binding
- no provider session, continuation, tools, or fallback

## Execution Plan

### Batch 24.1 — Addable Descriptor

- [ ] Execute card 076.
- [ ] expose one adapter-local hosted descriptor
- [ ] describe the secret Platform API-key field with no environment name
- [ ] describe the endpoint as an opaque host-owned config field

### Batch 24.2 — Admission And Prepare Handoff

- [ ] Execute card 077 after card 076.
- [ ] collect only a `CredentialRef` through the API-key sign-in loop
- [ ] retype the admitted endpoint and credential refs into
      `KimiPlatformPreparationInput`
- [ ] preserve the exact Platform audience and facade binding

### Batch 24.3 — Refresh, Catalogue, And 047 Path

- [ ] Execute card 078 after card 077.
- [ ] refresh host-supplied access status; subject stays `Absent`
- [ ] prepare the existing catalogue and one explicit K3 attempt
- [ ] project the consumer-assembled 047 snapshot without changing
      `Ready` / `NotReady`

## Acceptance Criteria

- [ ] linking `swallowtail-adapter-kimi-platform` can add exactly
      `kimi-platform.chat`
- [ ] portable records contain opaque endpoint and credential refs, never
      their values
- [ ] `prepare_kimi_platform_direct` remains after admission
- [ ] catalogue and inference keep exact `moonshot` / `kimi-k3` identity and
      explicit reasoning selection
- [ ] subject remains `Absent`; overlay and 047 readiness rules stay unchanged
- [ ] no live provider, billing, login, OAuth, or retry work runs
- [ ] `release-baselines/public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.023 047 presentation metadata
- this milestone: hosted API-key Kimi Platform Chat
- later: named installed descriptor candidates and separate gated route proofs
- hosted OAuth stays parked; generation continues toward 30-50

## Decision Gates

- Stop if the Platform API key enters a portable record or an environment
  value is invented.
- Stop if Kimi Membership, Kimi Code, or a regional Platform key is accepted
  as this route's audience.
- Stop if preparation moves before admission or selects a model implicitly.
- Stop if a provider-state, realtime, cloud-SDK, or owned-server route is
  folded into this descriptor.
- Stop if OpenHands gains a production route.
