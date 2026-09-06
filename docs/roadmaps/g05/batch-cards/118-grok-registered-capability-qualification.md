# 118 Grok common capability qualification

Status: planned; evidence preparation ready; runtime blocked until cards114-115 merge and consumer-tool protocol evidence exists
Owner: Tom
Created: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`

## Goal

Preserve merged one-shot permission semantics. Freeze exact Grok consumer-tool surface before implementing registration; an unsupported finding is truthful but does not complete operator full-MCP goal. No empty-list-to-support flag change.

## Scope

crates/swallowtail-adapter-grok/**; route research/fixtures and its public API baseline. Coordinator reserves all docs front doors, contracts, roadmap/card
status, shared Cargo manifests and unrelated owners' paths for serial closeout.
No product repository, credential/global settings, live client data or release.

## Acceptance And Review Oracle

Contract063 is the falsification oracle, including binding/revocation, bounded
payloads, lifecycle/cancellation/unknown outcomes and no mutating replay.
Exact-head independent cross-model review must inspect actual mounted/callable
wiring, not just isolated type fixtures. Missing evidence remains a blocked
capability. Public API changes are additive; incompatible surfaces stop before
mutation. Provider cards prove their exact route only, never another provider.

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy qa:docs` and `git diff --check`
- Card114 retains unchanged watcher and Claude Code compatibility fixtures;
  coordinator adds `swallowtail-adapter-claude-agent` to its focused compatibility
  round. No broad workspace reruns solely for unrelated docs movement.
- Real route probes use disposable inputs only after deterministic checks and
  explicit exact-route access authority. No tag/release authority in this card.

## Auto-Continuation

Coordinator dispatches the next ready manifest lane after accepted merge and
clean-main closeout. Worker stops at review; it never merges or releases.

## Independent Evidence Preparation

The retained route owner may start bounded read-only protocol/corpus research
now, before114/115. Publish exact surface evidence and proposed adapter mapping;
no runtime edits, live credential mutation or support claim in that preparation.
Runtime adoption waits for114/115 and a qualified surface. Missing evidence is
the preparation outcome to resolve, not a prerequisite for starting research.
