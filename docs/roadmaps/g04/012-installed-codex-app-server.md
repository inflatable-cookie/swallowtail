# 012 Installed Codex App-Server

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.011
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 032, 037, 047, 057
Planning state: cards 033-035 ready
Research: 169

## Problem

Codex app-server already has a prepared facade, discovery, and 029/032
classification. A consumer still cannot list it as an addable installed
route or admit an instance through Contract 057. ChatGPT access is cached
local login, not hosted URL-open OAuth.

## Generation Runway Goal

Prove one installed harness shape through the 057 facade, then reuse
`prepare_codex(AppServer)`.

## Goals

- [ ] expose an adapter-local installed addable descriptor for
      `codex.app-server`
- [ ] admit through the 057 store without extracting ChatGPT tokens
- [ ] reuse `prepare_codex(AppServer)` after admission
- [ ] refresh access status, project 029/032 update observation, and keep
      subject Absent

## Non-Goals

- hosted interactive OAuth
- Anthropic Messages changes
- Ollama descriptors
- OpenHands production wiring
- inventing a catalogue `provider_id` so overlay can mark rows
- live login, install, or billing probes
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 12.1 — Addable Descriptor

- [ ] Execute card 033.
- [ ] ship `AddableRouteDescriptor` from `swallowtail-adapter-codex`
- [ ] topology installed; config fields binary path and opaque env;
      ChatGPT path has no API-key field

### Batch 12.2 — Admission And Prepare

- [ ] Execute card 034 after card 033.
- [ ] admit through the 057 store
- [ ] no URL-open, loopback, or device-code ports; no secret extraction
- [ ] `prepare_codex(AppServer)` still prepares after admission

### Batch 12.3 — Refresh, Update, And Subject

- [ ] Execute card 035 after card 034.
- [ ] refresh host-supplied `AccessStatus`; subject stays Absent
- [ ] `observe_instance_update` reuses the existing app-server claim and
      optional 032 observation
- [ ] unmarked catalogue rows stay unmarked; do not invent a provider id

## Acceptance Criteria

- [ ] a consumer can assemble a catalog that includes Codex app-server by
      linking the adapter
- [ ] ChatGPT subscription admission writes no secret bytes
- [ ] `prepare_codex(AppServer)` still runs after admission
- [ ] update observation reuses 029/032
- [ ] no live login or install probe
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.011 hosted API-key Anthropic Messages
- this milestone: installed Codex app-server
- later: Ollama attach, hosted OAuth gate, Contract 052 consumer path

## Decision Gates

- Stop if ChatGPT tokens enter portable records.
- Stop if this route starts hosted URL-open OAuth.
- Stop if overlay invents a catalogue provider id.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
