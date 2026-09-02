# 059 Unix Owned-Tree Attestation

Status: ready; operator authorized a narrow unsafe or dependency boundary
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
      confused with a released process group or reused pid
- [ ] `OwnedTreeEmpty` is emitted only after the mechanism observes no owned
      member remaining; root exit and successful signalling stay insufficient
- [ ] a descendant surviving `setsid` or equivalent escape cannot produce the
      positive state
- [ ] inherited-descriptor EOF is not treated as tree emptiness unless closing,
      dropping, or failing to inherit the descriptor while alive is proved
      impossible or independently detected
- [ ] unsupported kernels and every ambiguous/error path remain `RootOnly`
- [ ] any unsafe block has a local safety proof and no provider or adapter code
      gains unsafe authority
- [ ] packaging, public API, platform, docs, and god-file evidence are exact

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
