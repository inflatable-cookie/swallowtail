# g05.023 Claude SDK Shared Lifecycle Prerequisites

Status: blocked; card 057 complete on an evidence stop; two operator decisions open
Owner: Tom
Created: 2026-09-02
Depends on: g05.022 cards 053-056; Contracts 010 and 019; PR 188 review
Vision tags: Claude Agent SDK, process trees, cleanup deadlines, lifecycle

## Purpose

Close the two provider-neutral lifecycle gaps that prevent the
`claude-agent.sdk` route from satisfying Contract 019. Keep PR 188 paused and
unmerged until both prerequisites land, then restack that existing branch and
finish card 055.

## Runway

1. Card 057 added honest process-tree completion evidence. The provider-neutral
   distinction between root exit and an attested-empty owned tree landed. The
   concrete local mechanism did not: the Unix group owner can enrol and
   terminate the tree but cannot observe it empty without probing a released
   bare group number or using `unsafe`. The local host stays root-only on
   every platform and constructs no tree claim.
2. Card 058 makes session cleanup caller-bounded. It is planned pending the
   operator's decision on a breaking public close signature for v0.4.0.
3. Card 055 then restacks onto both shared prerequisites. The existing worker,
   workspace, branch, and PR remain the continuation identity.

The cards are serial where they touch shared runtime/public API authority.
Card 057 is complete as far as it can be proved. Card 058 is not ready until
the public-API decision is recorded. PR 188 cannot merge ahead of either card.

Two operator decisions now gate this milestone, not one:

- the breaking caller-bounded close signature for card 058
- whether the local host may use `unsafe` (directly or through an
  encapsulating dependency) to install an inherited descendant-liveness
  descriptor, or whether a Linux-only procfs claim that leaves macOS root-only
  is acceptable

Contract 019 keeps `claude-agent.sdk` unavailable while owned-tree completion
stays unconfirmed, so card 055 cannot restack on the first decision alone.

## Boundaries

- Do not weaken Contract 019, infer tree emptiness from root exit, or invent a
  conversion for host-defined monotonic ticks.
- Do not expose `claude-agent.sdk` as available while its close contract is
  unmet.
- Do not replace or recreate PR 188. Preserve its valid model, tool,
  credential, identity, sidecar, and provider-free test work.
- No provider contact, Claude login, token read, package install, release
  preparation, tag, or push is authorized.

## Batch Cards

- [057 Owned Process-Tree Completion Evidence](batch-cards/057-owned-process-tree-completion-evidence.md) — complete; evidence stop; additive runtime vocabulary landed; no local positive attestation
- [058 Caller-Bounded Interactive Session Cleanup](batch-cards/058-caller-bounded-interactive-session-cleanup.md) — planned; awaiting operator acceptance of the breaking close seam
- [055 Claude Agent SDK Provider-Free Foundation](batch-cards/055-claude-agent-sdk-provider-free-foundation.md) — blocked on 057-058; preserve PR 188 for restack

## Acceptance

- process completion evidence cannot equate root exit with an empty tree —
  met
- every positive tree-empty claim is produced by a concrete host mechanism and
  fails closed on unsupported platforms — met vacuously: no host in this
  repository can make the claim, so every host fails closed
- session close and post-expiry cleanup have a caller-selected host deadline
  before the SDK route becomes available
- PR 188 is restacked only after both shared prerequisites merge
- release readiness remains paused and restarts from a later exact head

## Review Oracle

Invariant: no public operation reports cleanup completion while either its
deadline or any owned descendant remains unobserved.

Smallest counterexample: the root process exits zero while a grandchild stays
alive, or the caller deadline expires while an abort/join future remains
pending indefinitely.

Required proof: provider-neutral negative and positive process-completion
states, concrete host descendant fixtures, caller-selected deadline flow,
post-expiry cleanup races, public-API evidence, and the unavailable-to-ready
transition on the SDK route.

## Stop Conditions

Stop on a platform that cannot positively attest tree emptiness, a design that
requires guessed tick units, a compatibility shim that leaves unbounded close
callable, an unresolved public-API choice, or any attempt to merge PR 188
before both prerequisites land.
