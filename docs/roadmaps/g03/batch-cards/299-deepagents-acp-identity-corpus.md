# 299 Deep Agents ACP Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../096-deep-agents-acp-route.md`
Depends on: Card 286; Research 153; g03.094 closeout

## Goal

Freeze source and route-shape evidence for Deep Agents ACP. Candidate route is `deepagents.acp`. Do not edit production claims.

## Scope

Record official executable/server identity, transport, selected help/schema/event samples, authentication and working-resource authority, topology, persistence, cancellation, cleanup, and overlap with existing routes. Name the smallest deterministic corpus.

## Out Of Scope

driver implementation, prepared API, production matrix changes, live provider work, installation, login, and version-range claims

## Acceptance Criteria

- [x] Source identity and route disposition are explicit.
- [x] Route adds material information gain or records why it does not.
- [x] Authority and cleanup boundaries are named.
- [x] No claim changes before driver work.

## Validation

`effigy qa:northstar`; source and fixture review only.

## Stop Conditions

Stop if the surface is only a UI/TUI, wrapper/fork without divergence, or hidden remote/provider state is needed to establish the route.

## Auto-Continuation

Continue to card 300 only after route identity is admitted.

## Evidence

Research 157; `docs/logs/2026-08-19-deepagents-acp-0-1-25-identity.md`;
fixtures under `crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.25/`.
Admitted first-party npm `deepagents-acp@0.1.25` stdio ACP. No production claim.
