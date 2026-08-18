# 299 Deep Agents ACP Identity Corpus

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../096-deep-agents-acp-route.md`
Depends on: Card 286; Research 143

## Goal

Freeze source and route-shape evidence for Deep Agents ACP. Candidate route is `deepagents.acp`. Do not edit production claims.

## Scope

Record official executable/server identity, transport, selected help/schema/event samples, authentication and working-resource authority, topology, persistence, cancellation, cleanup, and overlap with existing routes. Name the smallest deterministic corpus.

## Out Of Scope

driver implementation, prepared API, production matrix changes, live provider work, installation, login, and version-range claims

## Acceptance Criteria

- [ ] Source identity and route disposition are explicit.
- [ ] Route adds material information gain or records why it does not.
- [ ] Authority and cleanup boundaries are named.
- [ ] No claim changes before driver work.

## Validation

`effigy qa:northstar`; source and fixture review only.

## Stop Conditions

Stop if the surface is only a UI/TUI, wrapper/fork without divergence, or hidden remote/provider state is needed to establish the route.

## Auto-Continuation

Continue to card 300 only after route identity is admitted.

## Evidence

Research 143; https://github.com/langchain-ai/deepagents; https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json
