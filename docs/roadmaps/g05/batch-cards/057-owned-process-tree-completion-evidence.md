# 057 Owned Process-Tree Completion Evidence

Status: complete; evidence stop; tree-completion vocabulary landed; no local positive attestation
Owner: Tom
Created: 2026-09-02
Milestone: `../023-claude-sdk-shared-lifecycle-prerequisites.md`
Depends on: Contracts 010 and 019; PR 188 exact-head review

## Goal

Add provider-neutral evidence that distinguishes one root process exiting from
the host proving its entire owned descendant tree is empty. Implement the
positive evidence in `swallowtail-host-local` only where its concrete process
ownership mechanism can prove it.

## Scope

1. Reproduce the review counterexample: a root exits while a descendant
   remains alive, and ordinary root `ProcessExit` evidence cannot close the
   tree claim.
2. Extend the runtime process-completion vocabulary additively. Existing
   `ProcessExit::new` callers remain root-only; only an explicit host-produced
   state may attest an empty owned tree. Do not infer it from exit code,
   `request_stop`, `force_stop`, or a successful nearest-child wait.
3. Audit the local Unix process-group owner, termination, root wait, owner
   wait, and reader joins. Produce positive tree-empty evidence only if the
   mechanism can rule out a surviving group member after cleanup without a
   reusable bare process-group identity. Stop if the current primitive cannot
   make that proof honestly.
4. Keep unsupported platforms and fixture hosts root-only unless they implement
   equivalent proof. Do not add a best-effort or platform-name inference.
5. Add deterministic fixtures for root-only, attested-empty, failed
   termination, and the root-exits/descendant-survives counterexample. Mutation
   must catch upgrading root-only evidence or dropping the post-termination
   emptiness check.
6. Update Contracts 010 and 019, directly related architecture/guides, public
   API evidence, card, milestone, log, and front doors for the behavior
   actually proved. Do not edit the paused PR 188 branch or advertise the SDK
   route as ready.

## Out Of Scope

Changing any handle `close` signature; choosing a cleanup duration; Claude SDK
adapter code; Node sidecar code; Windows job objects; provider contact; release
preparation; tags; unrelated process routes.

## Acceptance Criteria

- [x] root exit and owned-tree-empty are distinct provider-neutral evidence
- [x] existing constructors and fixture hosts do not silently gain tree claims
- [ ] local-host positive evidence has a concrete post-termination proof —
      **stopped**: no honest mechanism exists under the current posture, see
      Outcome
- [x] a surviving descendant cannot produce the positive state
- [x] unsupported mechanisms fail closed without weakening existing cleanup
- [x] public API, package, docs, and god-file baselines are reconciled

## Outcome

`ProcessTreeCompletion::{RootOnly, OwnedTreeEmpty}` is the additive
provider-neutral distinction. `ProcessExit::new` stays root-only, so no
existing caller or fixture host gained a tree claim. Only the explicit
`ProcessExit::attesting_empty_owned_tree` constructor can express the positive
state, and no host in this repository calls it.

The local host audit stopped at the card's named stop condition. Enrollment
and termination are proved: the Unix owner is spawned as its own group leader,
the root joins that group, and the group is only ever signalled while the
owner handle is live. Emptiness afterwards is not observable:

- the owner is itself a member of the group it anchors, so a group-directed
  liveness probe reports the owner and never distinguishes an empty group from
  a surviving member
- reaping the owner first would answer the question only by probing a
  released, reusable bare group number, which the card forbids
- an inherited descendant-liveness descriptor needs `CommandExt::pre_exec` and
  process-table enumeration by group needs `sysctl` or a procfs walk; both need
  `unsafe` or a platform dependency, and every crate here is
  `forbid(unsafe_code)` with no such dependency

Unblocking is an operator decision on posture, not a card-057 implementation
choice: authorize `unsafe` (or an encapsulating dependency) for the local host
so it can install an inherited liveness descriptor, or accept a Linux-only
procfs claim that leaves macOS root-only. Contract 019 keeps the SDK route
unavailable while the tree stays unconfirmed, so g05.023 does not close here.

## Validation

```sh
cargo fmt -p swallowtail-runtime -p swallowtail-host-local
effigy validate:focused swallowtail-runtime swallowtail-host-local
effigy package:verify-affected swallowtail-runtime swallowtail-host-local
effigy package:api
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

Do not run provider/live probes, broad workspace tests, release commands, or
edit PR 188.

## Review Oracle

Invariant: positive tree completion means the concrete host observed no member
remaining in the exact owned tree; root exit alone is never enough.

Smallest counterexample: the root exits successfully while a grandchild in the
owned group remains alive.

Expected stop point: the host returns root-only or failure evidence; it must
not construct the tree-empty state. Required proof includes a positive owned
tree fixture and a mutation that removes the final emptiness observation.

## Auto-Continuation

No. Exact-head frontier review before merge. Card 058 remains a separate public
API decision and PR 188 remains paused.

## Stop Conditions

Stop if the current host primitive cannot attest emptiness, if proof requires
signalling a reusable bare group id after ownership is released, if another
platform must be claimed without a native mechanism, or if the change expands
into the close-deadline decision.
