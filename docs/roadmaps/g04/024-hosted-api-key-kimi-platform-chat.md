# 024 Hosted API-Key Kimi Platform Chat

Status: planned
Owner: Tom
Created: 2026-08-21
Depends on: completed g04.023
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 037, 047, 052, 057
Planning state: named after g04.023; implementation cards not compiled
Research: 171

## Problem

`kimi-platform.chat` has a prepared hosted direct facade but is not in the
Contract 057 addable catalog. A consumer cannot yet collect its Platform API
key as a `CredentialRef`, admit an instance, and reuse
`prepare_kimi_platform_direct` through the connection lifecycle.

## Generation Runway Goal

Extend the proved hosted API-key shape by one stateless direct HTTP/SSE route.

## Goals

- expose an adapter-local hosted addable descriptor for
  `kimi-platform.chat`
- describe a secret Platform API-key field without inventing an environment
  name
- describe the approved `api.moonshot.ai` endpoint as an opaque config field
- admit the instance through Contract 057, then reuse
  `prepare_kimi_platform_direct`
- keep subject `Absent` and 047 `Ready` / `NotReady` semantics unchanged

## Non-Goals

- starting implementation before g04.023 closes
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
