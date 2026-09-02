# 059 Unix Owned-Tree Attestation

Status: complete; evidence stop; no sound mechanism found within current host-local authority on macOS; host stays root-only
Owner: Tom
Created: 2026-09-02
Milestone: `../023-claude-sdk-shared-lifecycle-prerequisites.md`
Depends on: Contracts 010 and 019; completed card 057; operator decision
  recorded 2026-09-02

## Goal

Give supported Unix hosts a concrete, non-forgeable observation that the exact
owned Claude SDK sidecar/native descendant tree is empty. Use narrowly scoped
unsafe platform code or an encapsulating dependency where necessary. Do not
equate permission to use unsafe with proof that a mechanism is sound.

## Scope

1. Re-derive the sidecar/native process topology and the local host's current
   process-group ownership from source and provider-free fixtures.
2. Select the smallest Unix mechanism that can observe the exact owned tree
   after bounded termination without probing a released/reusable bare process
   group or process id.
3. Confine any unsafe code behind one reviewed host-local module or use one
   narrowly scoped dependency with an audited transitive tree. Document the
   safety invariant at the boundary.
4. Emit `ProcessTreeCompletion::OwnedTreeEmpty` only from the positive host
   observation. Preserve `RootOnly` on unsupported platforms, incomplete
   enrollment, timeout, observation failure, or ambiguous identity.
5. Add deterministic descendants covering ordinary inheritance, nested spawn,
   session/group escape, root-first exit, forced cleanup, and identity reuse.
6. Reconcile Contracts 010/019, architecture, public API evidence, packaging,
   platform support, changelog, roadmap state, and god-file baseline.

## Out Of Scope

Claude SDK adapter restack; caller-bounded session close; provider contact;
Claude login; token access; release preparation; a process-table claim on an
unsupported platform; changing unrelated process routes to require positive
tree attestation.

## Acceptance Criteria

- [ ] one concrete supported-Unix mechanism owns an identity that cannot be
      confused with a released process group or reused pid —
      **stopped**: none was found and validated within the current ordinary
      host-local authority on macOS, see Outcome
- [x] `OwnedTreeEmpty` is emitted only after the mechanism observes no owned
      member remaining; root exit and successful signalling stay insufficient —
      no host constructs it; the constructor stays unused
- [x] a descendant surviving `setsid` or equivalent escape cannot produce the
      positive state
- [x] inherited-descriptor EOF is not treated as tree emptiness; a native
      counterexample shows EOF while a descendant is alive, and the host makes
      no descriptor-EOF claim
- [x] unsupported kernels and every ambiguous/error path remain `RootOnly`
- [x] any unsafe block has a local safety proof and no provider or adapter code
      gains unsafe authority — vacuous: no unsafe was added, the crate stays
      `forbid(unsafe_code)`
- [x] packaging, public API, platform, docs, and god-file evidence are exact

## Outcome

This is a bounded evidence stop. Four native counterexamples in the
`attestation` integration tests in `swallowtail-host-local` falsify the
candidate primitives; they falsify primitives rather than exercising an
integrated host implementation. Each candidate is insufficient:

- an inherited liveness descriptor installed through `CommandExt::pre_exec`
  defeats `setsid`, but a live descendant may close or not inherit it; the
  `an_inherited_liveness_descriptor_reaches_eof_while_a_descendant_is_alive`
  test observes EOF while the descendant is alive. The host cannot prove a
  foreign provider descendant preserves the descriptor, so EOF is not tree
  emptiness;
- process-group enumeration through `sysctl` or a procfs walk cannot see a
  `setsid` descendant that left the owned group
  (`a_setsid_descendant_escapes_owned_process_group_enumeration`), and the only
  way to observe the group empty is to reap the owner and probe a released,
  reusable group number
  (`a_released_owned_group_number_stops_existing_and_frees_its_identity`);
- an ancestry walk loses a descendant reparented to `launchd` after its
  intermediate parent exits, because macOS has no child-subreaper
  (`PR_SET_CHILD_SUBREAPER` is Linux-only); the native
  `a_reparented_descendant_is_orphaned_and_lost_by_an_ancestry_walk` test shows
  the orphan's parent become pid 1 while it is alive.

No sound owned-tree observation was found and validated within the current
ordinary host-local authority: `forbid(unsafe_code)`, no privileged capability,
and no system extension. A sound one would require a mechanism whose owned-tree
identity a descendant cannot escape by session change, descriptor drop, or
reparenting, with exclusive host ownership and denied migration out of the owned
set. A PID namespace, or a delegated cgroup v2 subtree the provider cannot write
itself out of via `cgroup.procs`, are the shapes that could satisfy that; both
are Linux-only, privilege-bearing, and cannot be natively validated from this
macOS host, so neither is landed here — publishing an unvalidated positive claim
is exactly what the oracle forbids. Entitlement or system-extension facilities
such as Apple Endpoint Security's fork/exit notifications exist but are outside
this card's narrow host-local boundary; this stop is "not found within current
authority", not "impossible on macOS".

`swallowtail-host-local` therefore keeps reporting
`ProcessTreeCompletion::RootOnly` on every platform, including exits where
termination succeeded, and no host constructs
`ProcessExit::attesting_empty_owned_tree`. No unsafe was added. Contract 019
keeps `claude-agent.sdk` unavailable under the current authority while owned-tree
completion stays unconfirmed, so g05.023 does not close here. Unblocking macOS
remains an operator decision: authorize and validate a container mechanism on a
platform that has one, evaluate an out-of-scope entitlement or system-extension
mechanism as a separate lane, or accept that the subscription-backed SDK route
stays unavailable on macOS.

## Validation

```sh
cargo fmt -p swallowtail-host-local -p swallowtail-runtime
effigy validate:focused swallowtail-host-local swallowtail-runtime
effigy package:verify-affected swallowtail-host-local swallowtail-runtime
effigy package:api
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

Run the positive mechanism only on platforms it claims. Cross-platform compile
checks may support, but never replace, a native positive observation.

## Review Oracle

Invariant: `OwnedTreeEmpty` means no process remains in the exact host-owned
tree, even when the root exits first and a descendant attempts to escape the
ordinary process group.

Smallest counterexamples: a `setsid` grandchild survives while the root exits
zero; a live descendant closes or never inherits a liveness descriptor and the
parent observes EOF; or a released group/pid is reused before a liveness probe.

Required proof: native process identity/enrollment evidence, all three
counterexamples, mutation of the positive constructor to each insufficient
signal, and a platform-negative path that stays root-only.

## Stop Conditions

Stop if the mechanism cannot defeat descriptor-close/non-inheritance,
`setsid`, and identity-reuse counterexamples; if it needs broad unsafe or a
large unrelated dependency; if macOS cannot make a positive claim; or if the
only viable result requires weakening Contract 019. Return the exact evidence
and keep the SDK route unavailable rather than publishing a best-effort claim.

## Auto-Continuation

No. Exact-head frontier review before merge. Merge before card 058 if both
branches touch shared runtime/API evidence; restack card 058 without changing
its accepted semantics.
