# 013 Local Ollama Attach

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.012
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 037, 047, 057
Planning state: cards 036-038 ready
Research: 169

## Problem

Ollama attach already has a prepared facade and runtime compatibility
claim. A consumer still cannot list it as an addable local-runtime route or
admit an instance through Contract 057. There is no credential. Swallowtail
does not install or start Ollama.

## Generation Runway Goal

Prove one local-runtime shape through the 057 facade, then reuse
`prepare_ollama_attached`.

## Goals

- [ ] expose an adapter-local local-runtime addable descriptor for
      `ollama.attached`
- [ ] admit through the 057 store with no credential field
- [ ] reuse `prepare_ollama_attached` after admission
- [ ] refresh access status, project 029 update observation, and keep
      subject Absent

## Non-Goals

- hosted interactive OAuth
- Anthropic or Codex descriptor changes
- OpenHands production wiring
- inventing a catalogue `provider_id` so overlay can mark rows
- installing, starting, or pulling Ollama
- live `/api/version` probes in the addable row
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 13.1 — Addable Descriptor

- [ ] Execute card 036.
- [ ] ship `AddableRouteDescriptor` from `swallowtail-adapter-ollama`
- [ ] topology local-runtime; config field API endpoint; no credential
      field

### Batch 13.2 — Admission And Prepare

- [ ] Execute card 037 after card 036.
- [ ] admit through the 057 store with an opaque endpoint config ref
- [ ] no sign-in loop
- [ ] `prepare_ollama_attached` still prepares after admission; model tag
      and digest stay prepare-time

### Batch 13.3 — Refresh, Update, And Subject

- [ ] Execute card 038 after card 037.
- [ ] refresh host-supplied `AccessStatus`; subject stays Absent
- [ ] `observe_instance_update` reuses `ollama_runtime_claim`; 032 stays
      unobserved unless an executable is supplied
- [ ] unmarked catalogue rows stay unmarked; do not invent a provider id

## Acceptance Criteria

- [ ] a consumer can assemble a catalog that includes Ollama attach by
      linking the adapter
- [ ] admission writes no secret bytes and no credential refs
- [ ] `prepare_ollama_attached` still runs after admission
- [ ] update observation reuses 029
- [ ] no live runtime start or install
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.012 installed Codex app-server
- this milestone: local Ollama attach
- later: hosted OAuth gate, Contract 052 consumer path

## Decision Gates

- Stop if Swallowtail installs or starts Ollama.
- Stop if overlay invents a catalogue provider id.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
- Stop if this route starts a sign-in loop.
