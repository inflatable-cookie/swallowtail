# 011 Hosted API-Key Anthropic Messages

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.010
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 020, 037, 047, 057
Planning state: cards 030-032 ready
Research: 169

## Problem

Anthropic Messages already has a prepared facade. A consumer still cannot
list it as an addable hosted route, collect an API key through Contract 057,
or admit an instance that later prepares without rebuilding that lifecycle
in the app.

## Generation Runway Goal

Prove one hosted API-key shape through the 057 facade, then reuse the
existing Anthropic prepared path.

## Goals

- [ ] expose an adapter-local hosted addable descriptor for
      `anthropic.messages`
- [ ] collect a secret API-key field as `CredentialRef` and an opaque
      endpoint config ref
- [ ] admit the instance through the 057 store, then reuse
      `prepare_anthropic_direct`
- [ ] refresh access status and observe subject as Absent without changing
      047 `Ready` / `NotReady`

## Non-Goals

- hosted interactive OAuth or Claude subscription
- Codex or Ollama descriptors
- OpenHands production wiring
- live provider, billing, or account probes
- extracting secrets from the environment as portable records
- adding overlay metadata to 047
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 11.1 — Addable Descriptor

- [ ] Execute card 030.
- [ ] ship `AddableRouteDescriptor` from `swallowtail-adapter-anthropic`
- [ ] topology hosted; credential field secret API key; config field
      endpoint

### Batch 11.2 — Admission And API-Key Collection

- [ ] Execute card 031 after card 030.
- [ ] admit through the 057 store with `CredentialRef` collection
- [ ] no URL-open, loopback, or device-code ports

### Batch 11.3 — Refresh, Subject, And 047 Path

- [ ] Execute card 032 after card 031.
- [ ] refresh host-supplied `AccessStatus`; subject stays Absent
- [ ] 047 snapshot plus overlay keys; `Ready` / `NotReady` unchanged

## Acceptance Criteria

- [ ] a consumer can assemble a catalog that includes Anthropic Messages by
      linking the adapter
- [ ] API-key collection writes `CredentialRef`, never secret bytes
- [ ] `prepare_anthropic_direct` still runs after admission
- [ ] overlay can key Anthropic catalogue rows
- [ ] no live provider probe
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.010 first-proof inventory
- this milestone: hosted API-key Anthropic Messages
- later: hosted OAuth gate, Codex app-server, Ollama attach, Contract 052
  consumer path

## Decision Gates

- Stop if secret bytes enter portable records.
- Stop if this route starts a browser OAuth loop.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
