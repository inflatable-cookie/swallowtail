# 021 Unmarked Overlay Rows

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.020
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 020, 047, 057
Planning state: cards 059-061 planned pending g04.020

## Problem

Overlay markers key to configured-instance, provider, and model ids.
Catalogue rows that omit `provider_id` stay unmarked. Codex, Claude
Agent ACP, Ollama, and llama.cpp attached hit that gap. Inventing a
catalogue `provider_id` is forbidden. The remaining choice is overlay
keying when provider id is absent, or recording unmarked as the durable
rule.

## Generation Runway Goal

Close remaining 057/047 seams and expand addable coverage on proved
shapes.

## Goals

- [ ] classify unmarked-row overlay without inventing a provider id
- [ ] realize the chosen rule
- [ ] prove it on the unmarked addable catalogues

## Non-Goals

- inventing a catalogue `provider_id`
- flattening mixed gateway rows into one catalogue
- changing 047 `Ready` / `NotReady`
- 047 presentation metadata (g04.023)
- hosted OAuth
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 21.1 — Classification

- [ ] Execute card 059.
- [ ] preferred direction: key instance plus model when `provider_id` is
      absent; do not invent a provider id
- [ ] stop and ask if that would flatten gateways

### Batch 21.2 — Realize The Rule

- [ ] Execute card 060 after card 059.
- [ ] either overlay keys absent-provider rows, or architecture records
      unmarked as durable

### Batch 21.3 — Unmarked-Route Proof

- [ ] Execute card 061 after card 060.
- [ ] Codex, Claude Agent, Ollama, and llama.cpp attached follow the
      chosen rule
- [ ] Anthropic and DeepSeek overlay keying stays unchanged

## Acceptance Criteria

- [ ] no invented catalogue `provider_id`
- [ ] mixed gateway rows remain consumer assembly
- [ ] 047 `Ready` / `NotReady` is unchanged
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.020 config-ref prepare handoff
- this milestone: unmarked overlay rows
- next: g04.022 further addable inventory
- later: 047 presentation metadata
- generation continues toward 30-50; do not roll over

## Decision Gates

- Stop if overlay invents a catalogue provider id.
- Stop if overlay changes `Ready` / `NotReady`.
- Stop if a model can be copied from another instance.
