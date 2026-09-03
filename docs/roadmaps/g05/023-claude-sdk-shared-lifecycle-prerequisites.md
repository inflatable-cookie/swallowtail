# g05.023 Claude SDK Shared Lifecycle Prerequisites

Status: completed; caller-bounded close and macOS degraded cleanup posture accepted
Owner: Tom
Created: 2026-09-02
Depends on: g05.022 cards 053-056; Contracts 010 and 019; PR 188 review
Vision tags: Claude Agent SDK, process trees, cleanup deadlines, lifecycle

## Purpose

Close the two provider-neutral lifecycle gaps that first prevented the
`claude-agent.sdk` route from satisfying Contract 019, then record the exact
macOS posture after tree-empty attestation stopped under ordinary authority.
Later rejected review and PR 193 containment supersede the original PR 188
restack plan; g05.025 now owns the remaining shared-runtime prerequisite.

## Runway

1. Card 057 added honest process-tree completion evidence. The provider-neutral
   distinction between root exit and an attested-empty owned tree landed. The
   concrete local mechanism did not: the Unix group owner can enrol and
   terminate the tree but cannot observe it empty without probing a released
   bare group number or using `unsafe`. The local host stays root-only on
   every platform and constructs no tree claim.
2. On 2026-09-02 the operator authorized the narrow unsafe/dependency posture
   needed for a sound Unix tree observation and the v0.4 breaking close seam.
3. Cards 058 and 059 ran in parallel. Card 058 made every public interactive
   close and post-expiry session cleanup path caller-bounded across all 22
   interactive adapter packages. Card 059 attacked exact Unix owned-tree
   attestation and stopped: four native counterexamples falsify the candidate
   primitives (`setsid` escape, descriptor EOF with a live child, released-group
   identity, and reparenting to `launchd`), and no sound owned-tree observation
   was found within the current ordinary host-local authority on macOS, so the
   local host stays root-only and adds no unsafe. A sound observation would need
   an inescapable owned-tree identity with exclusive host ownership and denied
   migration; that shape is Linux-territory (a PID namespace or a delegated
   cgroup v2 subtree) or an out-of-scope entitlement/system-extension mechanism,
   none validated from this macOS host. macOS remains unattestable under current
   authority pending an operator decision.
4. On 2026-09-03 the operator selected the bounded degraded posture for ordinary
   macOS. `OwnedTreeEmpty` remains the only basis for `Clean`; a confirmed root
   exit after the declared termination attempt is `Degraded`, while an
   unconfirmed root or observed survivor is `Failed`.
5. Card 055 was expected to restack onto both shared prerequisites and this
   accepted posture. Later rejected exact-head review proved those prerequisites
   insufficient; PR 188's route is withdrawn and any re-entry starts later from
   canonical main after g05.025/card 061.

The original implementation lanes were parallel, with serial same-repository
merge ordering for cards 059 and 058. That historical order is complete and no
longer authorizes PR 188 or adapter re-entry.

Contract 019 permits an explicitly qualified root-only platform only through
the bounded degraded rule. It does not infer tree emptiness or widen any other
route or platform.

## Boundaries

- Do not infer tree emptiness or `Clean` from root exit, or invent a conversion
  for host-defined monotonic ticks.
- Do not expose `claude-agent.sdk` when sidecar/root join is unbounded or
  unconfirmed; macOS support must surface the accepted degraded cleanup truth.
- This milestone never authorized restoring PR 188's route. The later card 055
  restoration entered from canonical main after g05.025 card 061, not from the
  reverted PR head. Its valid model,
  tool, credential, identity, sidecar, and provider-free test work remains
  historical evidence for card 055 after g05.025.
- No provider contact, Claude login, token read, package install, release
  preparation, tag, or push is authorized.

## Batch Cards

- [057 Owned Process-Tree Completion Evidence](batch-cards/057-owned-process-tree-completion-evidence.md) — complete; evidence stop; additive runtime vocabulary landed; no local positive attestation
- [058 Caller-Bounded Interactive Session Cleanup](batch-cards/058-caller-bounded-interactive-session-cleanup.md) — complete; exact host services and one caller-selected deadline bound every interactive close and post-expiry session cleanup path
- [059 Unix Owned-Tree Attestation](batch-cards/059-unix-owned-tree-attestation.md) — complete; evidence stop; four native counterexamples falsify the candidate primitives; no sound mechanism found within current host-local authority on macOS, so the host stays root-only and adds no unsafe
- [055 Claude Agent SDK Provider-Free Foundation](batch-cards/055-claude-agent-sdk-provider-free-foundation.md) — complete; re-entered from canonical main after g05.025 card 061 and restored the route under one enclosing cleanup guardian

## Acceptance

- process completion evidence cannot equate root exit with an empty tree —
  met
- every positive tree-empty claim is produced by a concrete host mechanism and
  fails closed on unsupported platforms — met vacuously: no host in this
  repository can make the claim, so every host fails closed
- session close and post-expiry cleanup have a caller-selected host deadline
  before the SDK route becomes available — met
- root-only macOS support never reports `Clean`; confirmed root completion is
  `Degraded`, while unconfirmed root or observed survivor is `Failed` — accepted
- PR 188's route remained withdrawn after this milestone and later g05.024/card
  060 proved insufficient for safe caller-expiry cleanup
- release readiness remains frozen and restarts only after later exact-head
  acceptance

## Outcome

The prerequisite milestone is complete. Card 058 removed the public unbounded
interactive close seam, migrated every implementation and fixture, and proved
one deadline over interruption, escalation, joins, credential release, and
resource release. Card 059's independent evidence stop remains decisive: no
sound macOS owned-tree observation exists within current ordinary host-local
authority. The operator accepted explicit macOS availability with bounded
root-only `Degraded` cleanup, never `Clean`. That made PR 188 eligible to
advance until later exact-head review found the narrower task ownership gap;
g05.024/card 060 owned that prerequisite, and g05.025/card 061 completed it
before card 055 resumed.

Subsequent exact-head review first found one narrower shared ownership gap: an
unfinished `JoinedTask` still blocks when its handle is dropped at the caller
deadline. g05.024/card 060 closed the successful-transfer case. Later rejected
review proved refusal, cleanup-continuation lease ownership, and the integrated
  reaper proof still unsafe. g05.025/card 061 owns the new provider-neutral
  shared-runtime reservation prerequisite without reopening this milestone's
  process-tree decision.

## Review Oracle

Invariant: no public operation exceeds its cleanup deadline or reports `Clean`
while descendant-tree emptiness remains unobserved.

Smallest counterexample: the root process exits zero while a grandchild stays
alive, or the caller deadline expires while an abort/join future remains
pending indefinitely.

Required proof: provider-neutral negative and positive process-completion
states, concrete host descendant fixtures, caller-selected deadline flow,
post-expiry cleanup races, public-API evidence, and the unavailable-to-ready
transition on the SDK route.

## Stop Conditions

Stop on a descriptor that can close or disappear while a descendant survives
being treated as tree-empty evidence, `Clean` from `RootOnly`, a design that
requires guessed tick units, a compatibility shim that leaves unbounded close
callable, or any adapter re-entry before g05.025/card 061 lands and passes
exact-head review.
