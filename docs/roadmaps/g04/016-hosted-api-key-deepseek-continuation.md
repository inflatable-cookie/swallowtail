# 016 Hosted API-Key DeepSeek Continuation

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.015
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 037, 047, 057
Planning state: cards 045-047 completed
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

- [x] expose an adapter-local hosted addable descriptor for
      `deepseek.continuation`
- [x] collect a secret API-key field as `CredentialRef` and an opaque
      endpoint config ref
- [x] admit the instance through the 057 store, then reuse
      `prepare_deepseek_direct`
- [x] refresh access status and observe subject as Absent without changing
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

- [x] Execute card 045.
- [x] ship `AddableRouteDescriptor` from `swallowtail-adapter-deepseek`
- [x] topology hosted; credential field secret API key; no invented env
      name; config field endpoint

### Batch 16.2 — Admission And API-Key Collection

- [x] Execute card 046 after card 045.
- [x] admit through the 057 store with `CredentialRef` collection
- [x] no URL-open, loopback, or device-code ports

### Batch 16.3 — Refresh, Subject, And 047 Path

- [x] Execute card 047 after card 046.
- [x] refresh host-supplied `AccessStatus`; subject stays Absent
- [x] 047 snapshot plus overlay keys `deepseek`; `Ready` / `NotReady`
      unchanged

## Acceptance Criteria

- [x] a consumer can assemble a catalog that includes DeepSeek continuation
      by linking the adapter
- [x] API-key collection writes `CredentialRef`, never secret bytes
- [x] `prepare_deepseek_direct` still runs after admission
- [x] overlay can key DeepSeek catalogue rows
- [x] no live provider probe
- [x] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.015 second-proof inventory
- this milestone: hosted API-key DeepSeek continuation
- next: g04.018 installed Claude Agent ACP
- later: llama.cpp attached, hosted OAuth gate

## Decision Gates

- Stop if secret bytes enter portable records.
- Stop if this route starts a browser OAuth loop.
- Stop if an environment name is invented.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
