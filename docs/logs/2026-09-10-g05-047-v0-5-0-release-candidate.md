# 2026-09-10 g05.047 v0.5.0 Release Candidate

Date: 2026-09-10
Task: `../roadmaps/g05/047-v0-5-0-release-candidate.md`
No provider call, credential access, Desktop edit, tag creation or push,
registry publication, or live-acceptance inference.

## Result

Source-only `v0.5.0` candidate prepared and merged. The coordinated version
is `0.5.0`, not `0.4.5`: g05.045 withdrew the `v0.4.4` universal native-tool
mediation guarantee, a guaranteed-behavior shrink that Contract 036 treats
as a pre-1.0 minor boundary. One authorized
`effigy --json release prepare --yes --check-gates --version 0.5.0` ran;
all seven cheap gates passed in repository order. Receipt SHA-256
`15f506d72b915a1679a8cea27d4e23578abaf439aefd7f9f778b48592ef376a1`
(`.release-prepared.json`, `prepared_at` `2026-09-10T08:17:26Z`,
`version_override_used: true` against mechanical `0.4.5`).

First review `5615566119` (changes required on `0fce574a`) blocked on two
tests equating frozen Desktop capsule sidecar identity
`swallowtail-claude-agent-sdk-sidecar@0.4.4` with the live
`CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG` (`@0.5.0`). The fix commit binds
historical evidence to the frozen tag and prefix-checks the live tag; no
`0.4.4` baseline, tag, or capsule bytes mutated. Second review `5615719889`
(ready to merge, no blocking findings) at head `2d49aeea`; PR
[#310](https://github.com/inflatable-cookie/swallowtail/pull/310) merged as
`31375966bb4bcb2b62d6c5396490355ba41955c8` on 2026-09-10. Merge tree
`4336c36236d865129638a8129914fab89fbe448c` is identical to the reviewed
candidate tree, so hosted evidence transfers to the merge SHA.

## Validation

PR run
[34456054548](https://github.com/inflatable-cookie/swallowtail/actions/runs/34456054548)
at `2d49aeea`: success, except `Pinned MSRV floor tests` skipped (does not
qualify alone). Merge-SHA push run
[34456800235](https://github.com/inflatable-cookie/swallowtail/actions/runs/34456800235):
all jobs success except `Pinned MSRV floor tests` failure —
`swallowtail-adapter-claude-agent --test integration`, 167 passed, 1 failed:
`structured_run::cancellation_and_deadline_stop_the_turn_then_join_operation_cleanup`
(`structured_run.rs:304`: observed `None`, expected
`swallowtail.session_cleanup.deadline_expired`). Stable shards and the
process-spawning board are green on the same tree; the failure is isolated
to the pinned-MSRV floor and timing-sensitive. Zero provider contact.

## Deferred

The MSRV floor failure needs a Chatterbox ruling (flake re-run versus a
deadline-observation repair lane) before any tag request. No qualifying
all-green hosted run exists yet at the merge tree, so the separate
annotated-tag decision stays pending with Tom. After an authorized tag, run
the source-tag consumer/working-application lane and resume blocked Desktop
g02.051 against the exact coordinated tag.

## Surfaces

`Cargo.toml` / `Cargo.lock` / `CHANGELOG.md` / root `README.md`,
`docs/releases/0.5.0.md`, fresh `release-baselines/*-0.5.0*` (40-package
public API, 49 production routes, 91 internal-dependency edges), the two
sidecar-evidence test files, this log.

`v0.5.0` remains absent locally and remotely. `v0.4.4` remains immutable at
`49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`. No matrix Yes/No, range,
publication, or Desktop pin follows.
