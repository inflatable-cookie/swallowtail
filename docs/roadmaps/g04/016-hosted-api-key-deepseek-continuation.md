# 016 Hosted API-Key DeepSeek Continuation

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.015
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 037, 047, 057
Planning state: cards 045-047 ready
Research: 170

## Problem

DeepSeek continuation already has a prepared facade. A consumer still
cannot list it as an addable hosted route, collect an API key through
Contract 057, or admit an instance that later prepares without rebuilding
that lifecycle in the app.

## Generation Runway Goal

Expand addable-route coverage on the proved hosted API-key shape, then
reuse `prepare_deepseek_direct`.

## Goals

- [ ] expose an adapter-local hosted addable descriptor for
      `deepseek.continuation`
- [ ] collect a secret API-key field as `CredentialRef` and an opaque
      endpoint config ref
- [ ] admit the instance through the 057 store, then reuse
      `prepare_deepseek_direct`
- [ ] refresh access status and observe subject as Absent without changing
      047 `Ready` / `NotReady`

## Non-Goals

- hosted interactive OAuth
- inventing an environment-variable name
- Claude Agent or llama.cpp descriptors
- OpenHands production wiring
- live provider, billing, or account probes
- extracting secrets from the environment as portable records
- adding overlay metadata to 047
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 16.1 — Addable Descriptor

- [ ] Execute card 045.
- [ ] ship `AddableRouteDescriptor` from `swallowtail-adapter-deepseek`
- [ ] topology hosted; credential field secret API key; no invented env
      name; config field endpoint

### Batch 16.2 — Admission And API-Key Collection

- [ ] Execute card 046 after card 045.
- [ ] admit through the 057 store with `CredentialRef` collection
- [ ] no URL-open, loopback, or device-code ports

### Batch 16.3 — Refresh, Subject, And 047 Path

- [ ] Execute card 047 after card 046.
- [ ] refresh host-supplied `AccessStatus`; subject stays Absent
- [ ] 047 snapshot plus overlay keys `deepseek`; `Ready` / `NotReady`
      unchanged

## Acceptance Criteria

- [ ] a consumer can assemble a catalog that includes DeepSeek continuation
      by linking the adapter
- [ ] API-key collection writes `CredentialRef`, never secret bytes
- [ ] `prepare_deepseek_direct` still runs after admission
- [ ] overlay can key DeepSeek catalogue rows
- [ ] no live provider probe
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.015 second-proof inventory
- this milestone: hosted API-key DeepSeek continuation
- later: Claude Agent ACP, llama.cpp attached, hosted OAuth gate

## Decision Gates

- Stop if secret bytes enter portable records.
- Stop if this route starts a browser OAuth loop.
- Stop if an environment name is invented.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
