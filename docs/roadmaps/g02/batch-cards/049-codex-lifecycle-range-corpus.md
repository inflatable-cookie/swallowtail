# 049 Codex Lifecycle Range Corpus

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../016-codex-thread-lifecycle-proof.md`

## Objective

Freeze Codex app-server archive, restore, and delete availability and semantics
across the maintained executable range.

## Governing Refs

- Research 036
- Contracts 029, 032-034, and 038
- existing Codex `0.80.0..=0.145.0` compatibility corpora
- official tagged Codex app-server sources

## Scope

1. Inspect baseline, latest-qualified, every existing behavior milestone, and
   lifecycle-method introduction boundaries.
2. Freeze exact request, response, notification, already-absent, active-target,
   archived-target, and descendant behavior.
3. Add lifecycle capability segments without shrinking the existing session
   support range.
4. Preserve exact exclusions, prerelease handling, and visible
   unverified-newer execution.
5. Record source commits, dates, schema hashes, and private behavior revisions.

## Acceptance Criteria

- [x] every qualified Codex segment has explicit lifecycle capability truth
- [x] archive, unarchive, and delete introduction points are exact
- [x] hard-delete and descendant semantics have primary-source evidence
- [x] missing files and already-deleted targets retain exact behavior
- [x] versions without lifecycle methods remain supported for existing work
- [x] no current-main behavior is projected backward without tagged evidence

## Validation

- deterministic corpus generation and hash check
- focused compatibility and protocol fixture tests
- `git diff --check`

## Stop Conditions

- tagged source cannot establish a method boundary
- a lifecycle method is experimental or unstable at a required point
- exact deletion semantics conflict across a claimed segment

## Auto-Continuation

No until the range evidence is reviewed. Then make card 050 ready.

## Completion Evidence

- Research 037 freezes tagged method and behavior boundaries:
  - archive at the `0.80.0` baseline
  - restore at `0.92.0`
  - lifecycle notifications at `0.104.0`
  - best-effort descendant archive at `0.123.0`
  - strict descendant hard delete at `0.140.0`
- the separate lifecycle compatibility claim preserves the existing
  `0.80.0..=0.145.0` app-server window and visible unverified-newer posture
- the deterministic fixture records 20 release checkpoints, exact commits,
  npm dates, schema authority, and four aggregate schema hashes per release
- archive guarantees only `TargetOnly`; descendant archive attempts may fail
  without failing the root
- delete qualifies `ProviderHardDeleted` with
  `ProviderDefinedDescendants` from `0.140.0`
- unknown and repeatedly fully deleted targets fail; a missing rollout is
  tolerated only after the target is otherwise known
- focused lifecycle corpus and selection tests pass
- card 050 is ready; no lifecycle provider effect has been implemented
