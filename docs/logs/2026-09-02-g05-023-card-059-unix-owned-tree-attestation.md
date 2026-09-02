# 2026-09-02 g05.023 Card 059 Unix Owned-Tree Attestation

Status: complete; evidence stop; macOS cannot attest under the authorized boundary; host stays root-only; no unsafe added; one PR; no merge
Owner: Tom

## Result

Card 059 asked whether the operator-authorized `unsafe`/dependency boundary
lets a supported Unix host construct `ProcessTreeCompletion::OwnedTreeEmpty`
soundly. The answer on macOS is no. Every candidate mechanism the boundary
permits fails at least one of the three review counterexamples, and the only
sound mechanism is a kernel-enforced owned-tree container that macOS does not
provide. The local host keeps reporting `RootOnly` on every platform, adds no
unsafe, and constructs no attested state. Contract 019 keeps
`claude-agent.sdk` unavailable while the tree stays unconfirmed, so g05.023
does not close here.

## Counterexamples Attacked Natively

The three review-oracle counterexample classes were driven through the
candidate mechanisms natively on this arm64 macOS host. The `attestation`
integration tests in `swallowtail-host-local` assert each one and pass.

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
- **Reparenting defeats an ancestry walk.** An orphaned descendant is
  reparented to `launchd` (pid 1) after its intermediate parent exits. macOS
  has no child-subreaper (`PR_SET_CHILD_SUBREAPER` is Linux-only), so an
  ancestry walk through `proc_listchildpids` loses the subtree and cannot
  attest emptiness.

## Why macOS Cannot Attest

A sound observation needs a non-forgeable owned-tree identity that a descendant
cannot leave by session change, descriptor drop, or reparenting. That is a
kernel-enforced container: a PID namespace, where the launcher is pid 1 and
reaps every orphan and `setsid` cannot cross the namespace; or a cgroup v2
`cgroup.events` `populated` view, which reads zero exactly when no process
remains in the cgroup and which `setsid` cannot escape. Both are Linux-only and
privilege-bearing. macOS provides neither, and no subreaper, so no mechanism
under the authorized boundary is sound there.

Landing a Linux-only positive claim was rejected on two grounds. It cannot be
natively validated from this macOS host, and the card requires the positive
mechanism to run on the platform it claims; a compile check may support but
never replace a native observation. Publishing an unvalidated or best-effort
positive claim is exactly what the oracle and the dispatch restrictions forbid.

## Decision Left To The Operator

Unblocking the macOS SDK tree gate is a posture decision, not a card-059
implementation choice. The options are to authorize a kernel-container
mechanism on a platform that has one (a Linux PID namespace or delegated cgroup
v2), validated natively on that platform, or to accept that the
subscription-backed SDK route stays unavailable on macOS while owned-tree
completion is unconfirmed. Card 058's caller-bounded close seam is the other
g05.023 prerequisite and remains ready.

## Shared Surfaces

- `swallowtail-host-local` `attestation` integration tests (new); the
  `exit_record` doc comment records the card-059 conclusion. No behavior, no
  public API, and no unsafe changed.
- Contracts 010 and 019 gain the durable rule that attestation requires a
  kernel-enforced owned-tree container and that a bare group, descriptor EOF,
  or ancestry walk is insufficient; macOS stays unattestable.
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

The three counterexample tests are native macOS observations, not compile-only
checks. No provider contact, live probe, package install, release work, tag, or
merge.

## Authority

- [card 059](../roadmaps/g05/batch-cards/059-unix-owned-tree-attestation.md)
- [card 057](../roadmaps/g05/batch-cards/057-owned-process-tree-completion-evidence.md)
- [g05.023](../roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
