# 097 Harness Route Expansion Intake And Watchlist

Status: planned
Owner: Tom
Created: 2026-08-18
Depends on: Research 143; g03.085
Planning state: cards 260-261, 286, and 303 planned

## Purpose

Provide the small cross-route gate for the route-specific roadmaps in this
suite. This document does not own an adapter route. It freezes the source pack,
checks contract fit, admits the primary route order, and later records the
secondary/watchlist disposition.

## Route Roadmaps

- [g03.086 Cline ACP Route](086-cline-acp-route.md)
- [g03.087 Cline Headless Route](087-cline-headless-route.md)
- [g03.088 Goose ACP Route](088-goose-acp-route.md)
- [g03.089 GitHub Copilot CLI ACP Route](089-copilot-cli-acp-route.md)
- [g03.090 Mistral Vibe Headless Route](090-mistral-vibe-headless-route.md)
- [g03.091 Qoder Headless Route](091-qoder-headless-route.md)
- [g03.092 Pi ACP Route](092-pi-acp-route.md)
- [g03.093 OpenHands Agent Server Route](093-openhands-agent-server-route.md)
- [g03.094 Kiro Headless Route](094-kiro-headless-route.md)
- [g03.095 Aider Headless Route](095-aider-headless-route.md)
- [g03.096 Deep Agents ACP Route](096-deep-agents-acp-route.md)

## Shared Cards

- [260](batch-cards/260-primary-wave-source-and-route-gate.md): freeze the
  primary source pack, candidate order, and stop conditions
- [261](batch-cards/261-primary-wave-contract-fit-and-batch-admission.md):
  classify operation shape, authority, topology, lifecycle, and contract fit
- [286](batch-cards/286-secondary-wave-source-and-disposition-gate.md): refresh
  secondary sources and admit/defer/reject/revisit each candidate
- [303](batch-cards/303-watchlist-and-registry-only-disposition.md): close the
  watchlist without creating adapter packages

## Ordering

Complete cards 260 and 261 before card 262. Primary route roadmaps then proceed
in numeric order, with each route allowed to close as accepted, deferred, or
negative evidence. Run card 286 only after the primary route set has been
recomputed. Card 303 may close the watchlist independently.

## Boundaries

- no adapter implementation in this intake roadmap
- no provider, model, endpoint, credential, or route selection policy
- no ACP registry bulk import
- no installation, login, live inference, release, or consumer adoption
- no route claim without an individual route roadmap and deterministic corpus

## Acceptance

- [ ] every route ID in the suite has exactly one route-specific roadmap
- [ ] primary and secondary ordering is explicit
- [ ] every candidate has an evidence-backed stop condition
- [ ] watchlist entries remain outside the production route matrix
- [ ] route-specific roadmaps own their own implementation and acceptance cards

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- Current route matrix: `docs/guides/provider-route-matrix.md`
- ACP registry: https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json
