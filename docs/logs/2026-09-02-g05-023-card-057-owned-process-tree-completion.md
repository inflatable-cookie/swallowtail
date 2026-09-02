# 2026-09-02 g05.023 Card 057 Owned Process-Tree Completion Evidence

Status: complete; vocabulary landed; local positive attestation stopped; one PR; no merge
Owner: Tom

## Result

Card 057 separates two facts that the process service previously conflated.
`ProcessTreeCompletion::RootOnly` says the one spawned root process ended and
nothing more. `ProcessTreeCompletion::OwnedTreeEmpty` says the host observed
that no member of its exact owned descendant tree remains. `ProcessExit::new`
keeps producing `RootOnly`, so no existing caller, adapter, or fixture host
gained a tree claim. The positive state has one explicit constructor,
`ProcessExit::attesting_empty_owned_tree`, and no host in this repository
calls it.

The local host stopped at the card's named stop condition. It does not
construct the attested state on any platform.

## Causal Audit Of The Local Unix Owner

Traced spawn, termination, root wait, owner wait, and reader joins.

Enrollment and termination are proved. `spawn_process_group_owner` starts
`/bin/sleep` with `process_group(0)`, so the owner is its own group leader and
the group id equals the owner pid. The root process is spawned with
`process_group(<owner pid>)`. `terminate_process_tree` refuses to run without
a live owner handle, so the host never signals a bare process-group number,
and the unreaped owner keeps the identity from being recreated by anyone else.

Emptiness afterwards is not observable, for three independent reasons:

1. The owner is itself a member of the group it anchors. `killpg(group, 0)`
   answers "the owner is still here" whether or not another member survives —
   a zombie is still a group member until reaped.
2. Reaping the owner first does answer the question, but only by probing a
   released, reusable bare group number. The card forbids exactly that, and
   the existing host invariant already forbids it.
3. `waitpid` cannot help. Reparented grandchildren are init's children, not
   ours, so a group-scoped wait sees only the root and the owner.

The two mechanisms that would observe emptiness honestly are both unavailable
under the current posture. An inherited descendant-liveness descriptor — one
extra pipe write end that every enrolled process holds across fork and exec,
whose read end reaches end-of-file only when the tree is gone — needs
`CommandExt::pre_exec`, because only stdin, stdout, and stderr are settable
through the safe `Command` API and all three are already committed. Process-
table enumeration by group needs `sysctl` on macOS or a procfs walk on Linux.
Both need `unsafe` or a platform dependency; every crate here is
`forbid(unsafe_code)` and `swallowtail-host-local` has no such dependency.

Terminating every member without being able to observe emptiness is exactly
the tension the milestone anticipated. Successful termination is a request
that succeeded, not an observation, and a member forked after the signal would
escape it.

## Falsification

The review counterexample is reproduced and asserted, not just described. The
`spawn-escaped-descendant-closed-pipes` fixture leaves a `setsid` grandchild
alive and independently observable by pid after ordinary group cleanup; its
test now asserts the root exit is successful *and* `RootOnly`.

Mutating `exit_record` to `attesting_empty_owned_tree` fails four tests:
the escaped-descendant counterexample, the clean root exit, the in-group
descendant case, and the forced stop. Any future host that upgrades local
evidence must retire those assertions deliberately.

## Operator Decision Required

g05.023 now has two gates, not one. Beyond the card 058 close signature, the
local host cannot attest tree completion until the operator chooses one of:

- authorize `unsafe` in `swallowtail-host-local`, or an encapsulating
  dependency, so the launch recipe can install an inherited liveness
  descriptor — portable across Unix, sound, fails closed when a descendant
  closes the descriptor
- accept a Linux-only procfs claim that leaves macOS root-only, which does not
  unblock the SDK route on the current development platform

Contract 019 keeps `claude-agent.sdk` unavailable while the tree stays
unconfirmed, so card 055 and PR 188 cannot restack on the close-signature
decision alone.

## Shared Surfaces

- `swallowtail-runtime` public API: `ProcessTreeCompletion`,
  `ProcessExit::attesting_empty_owned_tree`, `ProcessExit::tree_completion`
- Contracts 010 and 019
- `docs/architecture/system-architecture.md`
- `CHANGELOG.md` Unreleased
- g05.023 milestone, card 057, batch-card index, generation index, roadmaps
  Next Task
- this log and `docs/logs/README.md`

`swallowtail-host-local` behavior is unchanged: no spawn, termination, wait,
or cleanup path moved. PR 188 was not touched.

## Validation

Card-exact selectors on this worker head:

- `cargo fmt -p swallowtail-runtime -p swallowtail-host-local`
- `effigy validate:focused swallowtail-runtime swallowtail-host-local`
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No provider contact, live probe, package install, release work, tag, or merge.

## Authority

- [card 057](../roadmaps/g05/batch-cards/057-owned-process-tree-completion-evidence.md)
- [g05.023](../roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
