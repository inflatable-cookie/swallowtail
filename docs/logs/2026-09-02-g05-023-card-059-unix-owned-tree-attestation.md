# 2026-09-02 g05.023 Card 059 Unix Owned-Tree Attestation

Status: complete; evidence stop; no sound mechanism found within current host-local authority on macOS; host stays root-only; no unsafe added; one PR; no merge
Owner: Tom

## Result

Card 059 asked whether the operator-authorized `unsafe`/dependency boundary
lets a supported Unix host construct `ProcessTreeCompletion::OwnedTreeEmpty`
soundly. This is a bounded evidence stop: no sound owned-tree observation was
found and validated within the current ordinary host-local authority on macOS —
`forbid(unsafe_code)`, no privileged capability, and no system extension. Four
native counterexamples falsify the candidate primitives; they falsify primitives
rather than exercising an integrated host implementation. The local host keeps
reporting `RootOnly` on every platform, adds no unsafe, and constructs no
attested state. Contract 019 keeps `claude-agent.sdk` unavailable under that
authority while the tree stays unconfirmed, so g05.023 does not close here.

## Counterexamples Falsified Natively

Four candidate primitives were falsified natively on this arm64 macOS host. The
`attestation` integration tests in `swallowtail-host-local` assert each one and
pass. They falsify primitive candidates, not an integrated host implementation.

- **Session escape defeats a liveness descriptor's completeness partner.** An
  inherited liveness descriptor installed through `CommandExt::pre_exec`
  survives `setsid` because file descriptors are inherited across a session
  change. That is its one strength.
- **Descriptor close/non-inheritance defeats the liveness descriptor.**
  `an_inherited_liveness_descriptor_reaches_eof_while_a_descendant_is_alive`
  forks a child that inherits the pipe write end, closes it, and `exec`s a long
  sleep. The parent observes end-of-file on the read end while the child is
  alive. The host cannot prove a foreign provider descendant never closes or
  fails to inherit the descriptor, so descriptor EOF is not tree emptiness, and
  the oracle's escape clause is not satisfiable here.
- **Session escape defeats process-group enumeration.**
  `a_setsid_descendant_escapes_owned_process_group_enumeration` shows a
  `setsid` descendant becoming its own process-group leader, so its group id
  equals its pid and no longer matches the launcher's owned group. Enumeration
  by `sysctl` or procfs scoped to the owned group reports empty while the
  descendant is alive.
- **Identity reuse forbids probing a released group number.**
  `a_released_owned_group_number_stops_existing_and_frees_its_identity` reaps a
  group leader and then probes its number, which returns `ESRCH`. The only way
  to see the owned group empty is to reap the owner first, but the released
  number is then free for the kernel to reassign, so the probe is unsound. The
  host already refuses to signal a bare group without a live owner.
- **Reparenting defeats an ancestry walk.**
  `a_reparented_descendant_is_orphaned_and_lost_by_an_ancestry_walk` forks a
  grandchild that outlives its intermediate parent, then reaps the parent; the
  orphan's parent becomes pid 1 (`launchd`) while it is alive. macOS has no
  child-subreaper (`PR_SET_CHILD_SUBREAPER` is Linux-only), so an ancestry walk
  through `proc_listchildpids` rooted at the launcher loses the subtree and
  cannot attest emptiness.

## Why No Mechanism Was Found Within Current Authority

A sound observation needs an owned-tree identity a descendant cannot leave by
session change, descriptor drop, or reparenting, with exclusive host ownership
and denied migration out of the owned set. The shapes that could satisfy that
are a kernel-enforced container: a PID namespace, where the launcher is pid 1,
reaps every orphan, and `setsid` cannot cross the namespace; or a delegated
cgroup v2 subtree, whose `cgroup.events` `populated` field reads zero exactly
when no process remains — but only when the provider cannot migrate itself out
by writing `cgroup.procs`, so exclusive host ownership and migration denial are
required, not merely `populated`. Both are Linux-only and privilege-bearing.
Neither could be natively validated from this macOS host, and the card requires
the positive mechanism to run on the platform it claims; a compile check may
support but never replace a native observation. Publishing an unvalidated or
best-effort positive claim is exactly what the oracle and the dispatch
restrictions forbid, so neither is landed here.

This is "not found within current authority", not "impossible on macOS".
Entitlement or system-extension facilities — for example Apple Endpoint
Security, which delivers kernel fork and exit notifications with child process
identity — exist but are outside this card's narrow ordinary host-local
boundary. Evaluating one is a separate lane, not part of this evidence stop.

## Decision Left To The Operator

Unblocking the macOS SDK tree gate is a posture decision, not a card-059
implementation choice. The options are to authorize and validate a container
mechanism on a platform that has one (a Linux PID namespace, or a delegated
cgroup v2 subtree with exclusive host ownership and migration denial), natively
on that platform; to evaluate an out-of-scope entitlement or system-extension
mechanism (such as Apple Endpoint Security) as a separate lane; or to accept
that the subscription-backed SDK route stays unavailable on macOS under the
current authority while owned-tree completion is unconfirmed. Card 058's
caller-bounded close seam is the other g05.023 prerequisite and remains ready.

## Shared Surfaces

- `swallowtail-host-local` `attestation` integration tests (new); the
  `exit_record` doc comment records the card-059 conclusion. No behavior, no
  public API, and no unsafe changed.
- Contracts 010 and 019 gain the durable rule that a bare group, descriptor
  EOF, and an ancestry walk are insufficient, and that a sound observation needs
  an inescapable owned-tree identity with exclusive host ownership and denied
  migration; macOS stays unattestable under current host-local authority.
- `docs/architecture/system-architecture.md` process-tree paragraph.
- `CHANGELOG.md` Unreleased card-057 bullet extended with the card-059 result.
- g05.023 milestone, card 059, batch-card index, g05 index, generation index,
  roadmaps Next Task.
- this log and `docs/logs/README.md`.

`swallowtail-runtime` is unchanged. PR 188 was not touched. Card 058's owned
surfaces were not edited.

## Validation

Card-exact selectors on this worker head:

- `cargo fmt -p swallowtail-host-local -p swallowtail-runtime`
- `effigy validate:focused swallowtail-host-local swallowtail-runtime`
- `effigy package:verify-affected swallowtail-host-local swallowtail-runtime`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

The four counterexample tests are native macOS observations, not compile-only
checks. No provider contact, live probe, package install, release work, tag, or
merge.

## Authority

- [card 059](../roadmaps/g05/batch-cards/059-unix-owned-tree-attestation.md)
- [card 057](../roadmaps/g05/batch-cards/057-owned-process-tree-completion-evidence.md)
- [g05.023](../roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
