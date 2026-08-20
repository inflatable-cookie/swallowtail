# 018 Installed Claude Agent ACP

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.016
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 032, 037, 047, 057
Planning state: cards 050-052 ready
Research: 170

## Problem

Claude Agent ACP already has a prepared facade, discovery, and 029/032
classification. A consumer still cannot list it as an addable installed
route or admit an instance through Contract 057. Local subscription is
inherited login state, not hosted URL-open OAuth.

## Generation Runway Goal

Expand addable-route coverage on the proved installed shape, then reuse
`prepare_claude_agent`.

## Goals

- [ ] expose an adapter-local installed addable descriptor for
      `claude-agent.acp`
- [ ] admit through the 057 store on the local subscription profile
      without extracting keychain bytes
- [ ] reuse `prepare_claude_agent` after admission
- [ ] refresh access status, project 029/032 update observation, and keep
      subject Absent

## Non-Goals

- hosted interactive OAuth
- advertising `claude-code.headless` or `claude-code.response-only`
- API-key billing as this addable row
- llama.cpp descriptors
- OpenHands production wiring
- inventing a catalogue `provider_id` so overlay can mark rows
- live login, install, or billing probes
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 18.1 — Addable Descriptor

- [ ] Execute card 050.
- [ ] ship `AddableRouteDescriptor` from
      `swallowtail-adapter-claude-agent`
- [ ] topology installed; config fields binary path and opaque env; no
      credential field

### Batch 18.2 — Admission And Prepare

- [ ] Execute card 051 after card 050.
- [ ] admit through the 057 store
- [ ] no URL-open, loopback, or device-code ports; no keychain extraction
- [ ] `prepare_claude_agent` still prepares after admission with
      `LocalUnauthenticated` + `SubscriptionAllowance`

### Batch 18.3 — Refresh, Update, And Subject

- [ ] Execute card 052 after card 051.
- [ ] refresh host-supplied `AccessStatus`; subject stays Absent
- [ ] `observe_instance_update` reuses `claude_agent_acp_claim` and
      optional 032 observation
- [ ] unmarked catalogue rows stay unmarked; do not invent a provider id

## Acceptance Criteria

- [ ] a consumer can assemble a catalog that includes Claude Agent ACP by
      linking the adapter
- [ ] subscription admission writes no secret bytes and no credential
      refs
- [ ] `prepare_claude_agent` still runs after admission
- [ ] update observation reuses 029/032
- [ ] no live login or install probe
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.016 hosted API-key DeepSeek continuation
- this milestone: installed Claude Agent ACP
- later: llama.cpp attached, hosted OAuth gate

## Decision Gates

- Stop if keychain bytes or API keys enter portable records.
- Stop if this route is classified as hosted URL-open OAuth.
- Stop if overlay invents a catalogue provider id.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
- Stop if headless or response-only is advertised from this row.
