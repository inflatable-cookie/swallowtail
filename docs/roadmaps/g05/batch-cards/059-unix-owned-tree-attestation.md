# 059 Unix Owned-Tree Attestation

Status: complete; evidence stop; macOS cannot attest under the authorized boundary; host stays root-only
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
      **stopped**: no such identity exists on macOS under the authorized
      boundary, see Outcome
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

The three review counterexamples were driven natively through every candidate
mechanism the authorized boundary permits (see the `attestation` integration
tests in `swallowtail-host-local`). Each candidate fails at least one:

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
  (`PR_SET_CHILD_SUBREAPER` is Linux-only).

The only sound mechanism is a kernel-enforced owned-tree container — a PID
namespace or a cgroup v2 `populated` view — where the kernel owns a
non-forgeable identity that `setsid` cannot escape and no descriptor can drop.
macOS provides neither, nor a subreaper, so it cannot make a positive claim
under the authorized boundary. A PID-namespace or cgroup route is Linux-only,
privilege-bearing, and cannot be natively validated from this macOS host, so it
is not landed here: publishing an unvalidated positive claim is exactly what the
oracle forbids.

`swallowtail-host-local` therefore keeps reporting
`ProcessTreeCompletion::RootOnly` on every platform, including exits where
termination succeeded, and no host constructs
`ProcessExit::attesting_empty_owned_tree`. No unsafe was added. Contract 019
keeps `claude-agent.sdk` unavailable while owned-tree completion stays
unconfirmed, so g05.023 does not close here. Unblocking macOS remains an
operator decision: authorize a kernel-container mechanism on a platform that has
one (Linux), or accept that the subscription-backed SDK route stays unavailable
on macOS.

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
