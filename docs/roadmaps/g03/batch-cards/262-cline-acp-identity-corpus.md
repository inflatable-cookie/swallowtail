# 262 Cline ACP Identity Corpus

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../086-cline-acp-route.md`
Depends on: Card 261; Research 143

## Goal

Freeze the official Cline ACP identity and deterministic protocol corpus for `cline.acp`.

## Scope

Record ACP executable/server role, initialize and capability evidence, bounded activities/events, terminal outcomes, cancellation/deadline, authentication boundary, working-resource authority, isolation, and cleanup. Keep Cline headless evidence out of this corpus.

## Out Of Scope

headless JSON execution, driver code, prepared API, production claims, installation, login, live provider work, and version-range claims

## Acceptance Criteria

- [ ] exact ACP source identity and version axis are recorded
- [ ] ACP messages/events and drift-rejecting fixtures are named
- [ ] authority, cancellation, and cleanup limits are explicit
- [ ] no claim changes before card 263

## Validation

`effigy qa:northstar`; source and fixture review only.

## Stop Conditions

Stop if Cline ACP is only registry metadata, undocumented behavior, or requires hidden credential state to establish its wire shape.

## Auto-Continuation

Continue to card 263 after ACP route identity and corpus shape are frozen.

## Evidence

Research 143; https://github.com/cline/cline; https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json
