# 012 Installed Codex App-Server

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.011
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 032, 037, 047, 057
Planning state: cards 033-035 completed
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

- [x] expose an adapter-local installed addable descriptor for
      `codex.app-server`
- [x] admit through the 057 store without extracting ChatGPT tokens
- [x] reuse `prepare_codex(AppServer)` after admission
- [x] refresh access status, project 029/032 update observation, and keep
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

- [x] Execute card 033.
- [x] ship `AddableRouteDescriptor` from `swallowtail-adapter-codex`
- [x] topology installed; config fields binary path and opaque env;
      ChatGPT path has no API-key field

### Batch 12.2 — Admission And Prepare

- [x] Execute card 034 after card 033.
- [x] admit through the 057 store
- [x] no URL-open, loopback, or device-code ports; no secret extraction
- [x] `prepare_codex(AppServer)` still prepares after admission

### Batch 12.3 — Refresh, Update, And Subject

- [x] Execute card 035 after card 034.
- [x] refresh host-supplied `AccessStatus`; subject stays Absent
- [x] `observe_instance_update` reuses the existing app-server claim and
      optional 032 observation
- [x] unmarked catalogue rows stay unmarked; do not invent a provider id

## Acceptance Criteria

- [x] a consumer can assemble a catalog that includes Codex app-server by
      linking the adapter
- [x] ChatGPT subscription admission writes no secret bytes
- [x] `prepare_codex(AppServer)` still runs after admission
- [x] update observation reuses 029/032
- [x] no live login or install probe
- [x] `public-api-0.3.3` stays immutable

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
