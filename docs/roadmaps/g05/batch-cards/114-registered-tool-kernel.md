# 114 Shared registration and operation bridge kernel

Status: complete; PR 260 merged at `c214ac5e`
Owner: Tom
Created: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`

## Goal

Implement Contract063 snapshot, trusted binding/dispatcher, optional port and shared internal watcher/registered profile kernel. Preserve exhaustive HostServiceKind and every existing watcher signature. Provider-free implementation only. No adapter, Longhorn dependency, SSE qualification or daemon.

## Scope

crates/swallowtail-runtime/**; crates/swallowtail-host-local/**; crates/swallowtail-testkit/**; focused tests and public API baselines for those packages. Coordinator reserves all docs front doors, contracts, roadmap/card
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

- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy qa:docs` and `git diff --check`
- Card114 retains unchanged watcher and Claude Code compatibility fixtures;
  coordinator adds `swallowtail-adapter-claude-agent` to its focused compatibility
  round. No broad workspace reruns solely for unrelated docs movement.
- Real route probes use disposable inputs only after deterministic checks and
  explicit exact-route access authority. No tag/release authority in this card.

## Auto-Continuation

Coordinator dispatches the next ready manifest lane after accepted merge and
clean-main closeout. Worker stops at review; it never merges or releases.
