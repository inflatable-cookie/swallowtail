# g05.023 Claude SDK Shared Lifecycle Prerequisites

Status: stopped; caller-bounded close complete; macOS owned-tree gate unresolved under current authority
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
4. Card 055 then restacks onto both shared prerequisites. The existing worker,
   workspace, branch, and PR remain the continuation identity. The tree-empty
   gate is unresolved on macOS under current authority, so the SDK route cannot
   claim the full tree there yet.

The implementation lanes are parallel, but same-repository merge ordering is
serial: merge card 059 first if both touch runtime/API evidence, then restack
card 058. PR 188 cannot merge ahead of either card.

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
- [058 Caller-Bounded Interactive Session Cleanup](batch-cards/058-caller-bounded-interactive-session-cleanup.md) — complete; exact host services and one caller-selected deadline bound every interactive close and post-expiry session cleanup path
- [059 Unix Owned-Tree Attestation](batch-cards/059-unix-owned-tree-attestation.md) — complete; evidence stop; four native counterexamples falsify the candidate primitives; no sound mechanism found within current host-local authority on macOS, so the host stays root-only and adds no unsafe
- [055 Claude Agent SDK Provider-Free Foundation](batch-cards/055-claude-agent-sdk-provider-free-foundation.md) — blocked on the unresolved macOS owned-tree posture decision; preserve PR 188

## Acceptance

- process completion evidence cannot equate root exit with an empty tree —
  met
- every positive tree-empty claim is produced by a concrete host mechanism and
  fails closed on unsupported platforms — met vacuously: no host in this
  repository can make the claim, so every host fails closed
- session close and post-expiry cleanup have a caller-selected host deadline
  before the SDK route becomes available — met
- PR 188 is restacked only after both shared prerequisites merge
- release readiness remains paused and restarts from a later exact head

## Outcome

The caller-bounded half is complete. Card 058 removed the public unbounded
interactive close seam, migrated every implementation and fixture, and proved
one deadline over interruption, escalation, joins, credential release, and
resource release. Card 059's independent evidence stop remains decisive: no
sound macOS owned-tree observation exists within current ordinary host-local
authority. This milestone therefore stops without making `claude-agent.sdk`
available. The preserved PR 188 cannot advance until the operator selects and
authorizes a new tree-attestation posture or accepts the route's unavailability
on macOS.

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

Stop on a platform that cannot positively attest tree emptiness, a descriptor
that can close or disappear while a descendant survives, a design that
requires guessed tick units, a compatibility shim that leaves unbounded close
callable, or any attempt to merge PR 188 before both prerequisites land.
