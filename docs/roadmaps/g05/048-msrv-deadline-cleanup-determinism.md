# g05.048 MSRV Deadline-Cleanup Determinism

Status: ready; operator-authorized repair and qualifying hosted gate
Owner: Tom
Created: 2026-09-10
Depends on: g05.047; Contract 036; release playbook
Vision tags: release reliability, deadline, cleanup, MSRV

## Outcome

Remove the recurring pinned-MSRV race from the Claude ACP structured-run
deadline/cleanup proof without weakening either production guarantee. Prove
operation timeout and cleanup-deadline expiry through separately controlled
observations, merge the independently reviewed repair, and obtain one
qualifying all-green hosted run on the exact `0.5.0` source tree before the
tag decision returns to Tom.

## Decision

The same test failed in the `v0.4.4` release lane and again in merge-SHA run
34456800235 for the `v0.5.0` candidate. A blind rerun is not acceptable
evidence. The current fixture's `with_immediate_deadline()` makes every
deadline wait resolve immediately, while the assertion assumes that an
operation timeout necessarily forces session cleanup past its independent
deadline. Thread scheduling can instead let cleanup finish cleanly, producing
the observed `TimedOut` terminal status with no cleanup diagnostic.

Tom's 2026-09-10 `Continue` authorizes the recommended deterministic repair,
independent review, and one new qualifying hosted gate. It does not authorize
a release prepare rerun, workflow-file edit, provider call, Desktop mutation,
tag, publication, or release artifact.

## Ready-State Rubric

- [x] g05.047 prepared version `0.5.0` exactly once and merged PR #310 as
      `31375966bb4bcb2b62d6c5396490355ba41955c8`, tree
      `4336c36236d865129638a8129914fab89fbe448c`.
- [x] Independent review `5615719889` accepted the candidate source.
- [x] Push run 34456800235 passed every job except one pinned-MSRV test:
      167 passed and
      `structured_run::cancellation_and_deadline_stop_the_turn_then_join_operation_cleanup`
      failed with cleanup diagnostic `None` instead of
      `swallowtail.session_cleanup.deadline_expired`.
- [x] The same exact test failed in prior `v0.4.4` run 34348374964.
- [x] `v0.5.0` remains absent locally and remotely; `v0.4.4` remains immutable
      at `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`.
- [x] The integration checkout is clean on pushed `main` at g05.047 closeout
      `0c75aee553d35f21bfbf35c51eba2c3d6acf12bd` before this planning change.

## Dispatch Manifest

- **State:** ready; dispatch after this planning commit reaches pushed `main`.
- **Completion:** deterministic fixture/test repair; exact pinned-MSRV focused
  proof; focused package validation; independent exact-head review; merged
  repair with unchanged `0.5.0` release surfaces; qualifying all-green hosted
  CI at the repair head or identical tree; updated candidate identity and
  closeout.
- **Owned mutable paths:**
  `crates/swallowtail-adapter-claude-agent/tests/structured_run.rs`, the narrow
  fixture controls under
  `crates/swallowtail-adapter-claude-agent/tests/support/` required to make
  deadline observations explicit, this task, one new log and their indexes;
  `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`.
- **Worker:** provider-free Rust test worker.
- **Feature freeze:** preserve the `0.5.0` candidate contents and coordinated
  version through repair disposition. The accepted repair merge becomes the
  exact candidate source identity presented for tag authority.
- **Excluded:** production runtime or adapter source; Cargo/version/lock,
  changelog, release note, release baselines, `.release-prepared.json`,
  `.github/workflows/`, earlier tags or evidence; release prepare/execute;
  providers, credentials, Desktop or other consumer mutation; local or remote
  tag; registry, GitHub Release, artifact, binary, sidecar, installer, or model
  publication.
- **Escalation:** stop to Chatterbox if deterministic proof exposes a
  production runtime defect, needs a workflow change, or cannot preserve both
  timeout and cleanup contracts. Tom owns the exact-SHA tag decision.

## Work

1. Reconfirm clean pushed `main`, version `0.5.0`, absent `v0.5.0`, immutable
   `v0.4.4`, and the two exact failing hosted records. Do not rerun either old
   CI event.
2. Separate the fixture's operation-deadline observation from its cleanup-
   deadline observation. Use controlled monotonic observations, barriers, or
   equivalent deterministic signals; do not use sleeps, wall-clock margins,
   probabilistic retries, ignored tests, relaxed assertions, or compiler-
   version branches.
3. Prove both independent semantics: an operation deadline yields
   `TerminalStatus::TimedOut` and joins cleanup; a cleanup that crosses its
   caller boundary yields
   `swallowtail.session_cleanup.deadline_expired`. A clean cleanup completed
   before its boundary remains `CleanupOutcome::Clean` and must not be forced
   into a failure merely to satisfy the old assertion.
4. Run the exact test repeatedly on pinned Rust `1.95.0` in one bounded local
   stress proof, then run `effigy validate:focused
   swallowtail-adapter-claude-agent`. Record commands, counts, and results.
5. Open one repair PR and obtain independent exact-head review. The reviewer
   must inspect the scheduling counterexample and confirm no production,
   release, historical-evidence, or workflow surface changed.
6. After local proof and review, obtain one qualifying hosted CI run on the
   exact repair head. Merge only when every job, including `Pinned MSRV floor
   tests`, is green. If the merge tree differs, run the qualifying gate at the
   merge SHA; otherwise the identical-tree evidence transfers.
7. Record repair head, merge SHA/tree, review, hosted run ID/trigger/job
   conclusions, pinned-MSRV repetition proof, unchanged release receipt and
   surfaces, zero provider contact, and absent `v0.5.0`. Return the exact
   candidate SHA to Tom for the separate tag decision.

## Review Oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Timeout and cleanup deadlines stay distinct | one immediate clock drives both and assumes the same result | deterministic fixture controls and assertions name each observation separately |
| Production semantics stay strong | accept either result, delete the diagnostic assertion, or weaken runtime cleanup | explicit timeout, clean-before-boundary, and cleanup-expired cases all pass |
| The proof is scheduler-independent | sleeps, generous margins, retries, or a one-off green run | bounded repeated exact test on Rust 1.95.0 with zero failures |
| Repair scope stays narrow | change runtime, workflow, version, release receipt, or frozen evidence | exact diff contains only the owned test/fixture and closeout surfaces |
| Hosted release gate is real | rerun old failure, rely on PR CI with skipped floor tests, or ignore one red job | new exact-head/identical-tree run has every job green including pinned-MSRV tests |
| Tag gate stays closed | create or push `v0.5.0` after repair | local and remote tag remain absent; closeout asks for exact-SHA authorization |

## Stop Conditions

- Stop if the worktree is dirty, canonical remote differs, version is not
  `0.5.0`, `v0.5.0` exists, or an earlier tag/evidence surface would change.
- Stop if the failure requires production runtime/adapter behavior or a
  workflow edit; return the deterministic counterexample to Chatterbox.
- Stop if the repair relies on sleeping, wall-clock tolerance, retry-until-
  green, ignoring a test, weakening an assertion, or special-casing Rust
  `1.95.0`.
- Stop on any focused, review, or hosted failure. Do not consume another
  hosted run silently.
- Stop before release prepare/execute, provider contact, Desktop mutation,
  tag creation or push, publication, or artifact upload.

## Evidence

On completion record the exact changed test/fixture paths, deterministic
counterexample and repaired oracle, local pinned-MSRV repetition count,
focused validation, PR/head/review/merge/tree, qualifying hosted run and every
job conclusion, unchanged `0.5.0` release receipt/surfaces, zero provider
contact, and local/remote tag absence.

## Next Task

Return the repaired exact `v0.5.0` candidate SHA and qualifying all-green CI
to Tom for the separate annotated-tag decision. After an authorized tag, run
the source-tag consumer/working-application lane and resume blocked Desktop
g02.051 against that exact tag. Nothing follows automatically.
