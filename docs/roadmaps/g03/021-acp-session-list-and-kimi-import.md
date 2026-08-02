# 021 ACP Session List And Kimi Import

Status: active
Owner: Tom
Created: 2026-08-01
Depends on: g03.020
Vision tags: shared protocol, Kimi continuity, capability fidelity
Contract refs: 009-011, 015, 017, 029, 035, 037, 046
Planning state: cards 055-056 completed; card 057 ready

## Problem

Stable ACP now defines optional `session/list`, while Swallowtail's common ACP
codec and production adapters do not expose it. Capability advertisement alone
cannot qualify list, replay, or import for every ACP agent.

## Goals

- [x] freeze stable ACP list schema and compatibility identity
- [x] add bounded list framing and candidate projection to the shared codec
- [x] qualify Kimi Code ACP as the first complete ACP import route
- [x] preserve Kimi's exact state-root and working-resource binding
- [ ] classify Claude and Cursor without widening their current claims
- [ ] prove common wire behavior across host topologies

## Execution Plan

### Batch 21.1 — Stable ACP List Corpus And Codec

- [x] Execute card 055.
- [x] freeze capability, request, response, cursor, cwd, title, time, and `_meta`
  shapes from the current stable schema
- [x] add bounded provider-neutral ACP message projection
- [x] reject unsupported, malformed, oversized, and cross-request responses

### Batch 21.2 — Kimi ACP Catalogue And Import

- [x] Execute card 056 after card 055 passes.
- [x] freeze exact Kimi list behavior across qualified milestones
- [x] require matching execution host, Kimi state root, working resource,
  access, model, and session policy
- [x] route imported sessions through existing Kimi load replay and resume

### Batch 21.3 — ACP Conformance And Route Classification

- [ ] Execute card 057 after card 056 passes.
- [ ] prove Kimi under local and remote-authoritative host identities
- [ ] keep Claude unavailable for listing until it advertises and implements it
- [ ] keep Cursor load/list unavailable until exact behavior is qualified
- [ ] reconcile ACP and Kimi prepared guidance and package evidence

## Boundaries

- no claim from stable ACP wire support alone
- no arbitrary `_meta` projection or raw cwd diagnostic
- no relabelling of Kimi ACP bindings as local-server bindings
- no process-state directory scan or account-wide Kimi listing
- no Claude, Cursor, Gemini, or Grok production capability change
- no remote ACP production-provider claim beyond existing topology proof
- no live prompt, authentication mutation, or broad workspace suite

## Acceptance Criteria

- [ ] list capability is negotiated independently from load, resume, and delete
- [ ] ACP cursors and candidates are bounded and request-scoped
- [ ] Kimi import revalidates exact state root and resource authority
- [ ] Kimi load replay completes before the imported handle is ready
- [ ] unsupported ACP agents fail before list or import dispatch
- [ ] local and remote-authoritative deterministic behavior agrees
- [ ] focused ACP, Kimi, and affected-package validation pass

## Next Planning Checkpoint

After card 057, continue to the independent attached-HTTP proof in g03.022.
Do not generalize Kimi success to another ACP adapter.
