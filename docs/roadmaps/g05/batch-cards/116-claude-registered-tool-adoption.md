# 116 Claude SDK common registered-tool adoption

Status: complete; runtime PR 266 merged at `dbbcd192`; evidence PR 258 at `d74f5975`; route binding continues as card 125
Owner: Tom
Created: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`

## Goal

Reuse merged Card084 stdio support. Qualify common host-mediated callback or exact private carrier before wiring. Preserve existing omission/resume/permission behavior. HTTP/SSE needs real pinned protocol evidence; do not remake Card084.

## Scope

crates/swallowtail-adapter-claude-agent/**; route fixtures and its public API baseline. Coordinator reserves all docs front doors, contracts, roadmap/card
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

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
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

## Result

Implementation is provider-free and feature-gated. The mounted/callable fixture
uses the real courier binary, kernel listener, lease/generation authority,
dispatcher, and SDK-shaped process launcher. Validation is named explicitly so
the feature path cannot remain dark:

`effigy validate:card116-mediated-stdio`

The promoted Contract 063/060 and Chatterbox ruling are the authority where the
original card-owned adapter wording drifted. No provider, credential, Desktop,
release, or tag action was taken. Exact-head independent review remains the stop
gate.

F6 is load-bearing: an all-`NativeClient` mediated selection returns exact
`unsupported_tool` before listener, lease, or dispatcher work; a mounted mixed
snapshot exposes and dispatches only MCP while rejecting `NativeClient` without
host dispatch.
