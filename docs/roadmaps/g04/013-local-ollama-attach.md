# 013 Local Ollama Attach

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.012
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 037, 047, 057
Planning state: cards 036-038 completed
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

- [x] expose an adapter-local local-runtime addable descriptor for
      `ollama.attached`
- [x] admit through the 057 store with no credential field
- [x] reuse `prepare_ollama_attached` after admission
- [x] refresh access status, project 029 update observation, and keep
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

- [x] Execute card 036.
- [x] ship `AddableRouteDescriptor` from `swallowtail-adapter-ollama`
- [x] topology local-runtime; config field API endpoint; no credential
      field

### Batch 13.2 — Admission And Prepare

- [x] Execute card 037 after card 036.
- [x] admit through the 057 store with an opaque endpoint config ref
- [x] no sign-in loop
- [x] `prepare_ollama_attached` still prepares after admission; model tag
      and digest stay prepare-time

### Batch 13.3 — Refresh, Update, And Subject

- [x] Execute card 038 after card 037.
- [x] refresh host-supplied `AccessStatus`; subject stays Absent
- [x] `observe_instance_update` reuses `ollama_runtime_claim`; 032 stays
      unobserved unless an executable is supplied
- [x] unmarked catalogue rows stay unmarked; do not invent a provider id

## Acceptance Criteria

- [x] a consumer can assemble a catalog that includes Ollama attach by
      linking the adapter
- [x] admission writes no secret bytes and no credential refs
- [x] `prepare_ollama_attached` still runs after admission
- [x] update observation reuses 029
- [x] no live runtime start or install
- [x] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.012 installed Codex app-server
- this milestone: local Ollama attach
- next: g04.014 Contract 052 consumer path
- later: hosted OAuth gate

## Decision Gates

- Stop if Swallowtail installs or starts Ollama.
- Stop if overlay invents a catalogue provider id.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if OpenHands gains a production route.
- Stop if this route starts a sign-in loop.
