# g05.034 Release Lane Simplification

Status: ready; v0.4.4 acceptance audit recorded as an honest evidence stop (coordinator closeout)
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-09
Depends on: Contract 036; the v0.4.1, v0.4.2, and v0.4.3 lane records (g05.030, g05.032, g05.033); completed cards 093, 094, 103, 104; the operator direction of 2026-09-06
Vision tags: release, CI, dependability

## Purpose

Two patch releases cost most of two days. The operator's direction: the
lane must be short, simple, and dependable, and CI must be much faster.
This milestone removes the causes measured in those lanes rather than
adding process.

## Measured Causes

| Cost | Cause | Fixed by (folded card) |
| --- | --- | --- |
| ~30 min per local prepare attempt; 4 attempts on `v0.4.2`, 4-5 on `v0.4.1` | gate order runs two full clippy passes and the whole test suite before the cheap checks; every late failure discards them | 109 |
| the same clippy and tests run again in the exact-SHA hosted run | local prepare and hosted CI duplicate each other | 109 |
| 3 of 4 `v0.4.2` attempts failed on docs index drift | closeouts pushed to `main` without the docs check | 111 |
| review rounds on gate scripts pinned to the old version and the baseline directory roles | release mechanics need manual repointing and are undocumented | 110 |
| 370 integration test files, each a linked binary; nextest shards ~3.5 min, local suite far longer | link time dominates | 112 |
| MSRV floor job 5.4 min on macOS is the PR critical path; 3 macOS shards | job placement and shape | 113 |
| flaky fixtures forcing reruns | timing bounds, temp paths, teardown | 104 (in flight), 093, 094 |

## Target

The next release prepares in under 30 minutes wall clock from "gates green
on `main`" to "tag request presented", with one prepare attempt and one
hosted run. PR gate under 3 minutes. No rerun-to-green anywhere.

## Runway

1. Gate order and hosted-gate delegation (Contract 036 amendment) (card 109 complete).
2. Version-derived gate scripts and documented baseline roles (card 110 complete).
3. Docs check on push; status grammar that closeouts cannot break (card 111 complete).
4. Integration test binary consolidation, largest crates first (card 112 complete).
5. CI critical path: MSRV to Linux clippy-only on PRs, shard rebalancing, cache audit (card 113 complete).

Cards 112 and 113 touched no release surface and ran under the freeze in
parallel with the `v0.4.3` candidate. Cards 109-111 ran after the tag.

## Folded card evidence (g05.038)

- 109 Release Gate Order And Hosted Delegation — complete; ran after the `v0.4.3` tag
- 110 Version-Derived Release Scripts And Baseline Roles — complete; PR 252; exact-head re-review
- 111 Docs Check On Push And Closeout Grammar — complete; ran after the `v0.4.3` tag
- 112 Integration Test Binary Consolidation — complete; ran under the freeze
- 113 CI Critical Path — complete; ran under the freeze; operator workflow authority from card 095 carried

## Retired card dispatch (g05.038)

Card-level dispatch is retired: cards 109-113 are all complete, their serial
edges and concurrency notes are now owned by this task, and acceptance is
unchanged — the `v0.4.4` or `v0.5.0` lane wall clock still records against the
target.

## Acceptance

The `v0.4.4` lane ran on the new shape and recorded its wall clock against
the target. It missed the all-green outcome. See Result.

## Acceptance Audit Dispatch

Operator `Continue` on 2026-09-09 authorizes the bounded `v0.4.4` measurement
and closeout. This task does not repair release mechanics or reinterpret a
missed target as success.

| Field | g05.034 acceptance audit |
| --- | --- |
| Readiness | ready; `v0.4.4` is tagged and the candidate, queue, CI, review, merge, and tag records are durable |
| Prerequisites | completed g05.036; Card 154 queue detail and closeout; PR #300 and Actions runs `34345060452`, `34348370780`, `34348374964`, and `34350208617`; tag task `a4962ded-607f-4165-8a4d-a7dc34368f79` |
| Completion conditions | one UTC event ledger defines each measured boundary and source; each target receives an independent pass/fail; exceptional pre-tag Desktop qualification and unrelated planning migrations are separated from core release mechanics without removing them from end-to-end time; this task and one log record the result; current indexes and Next Task are reconciled |
| Owned mutable paths | this task; one new `docs/logs/2026-09-09-g05-034-*.md`; one `docs/logs/README.md` index line; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`; queue coordinator edits these after merge |
| Forbidden paths | Rust, workflows, Effigy configuration, release scripts, contracts, release notes, tags, consumers, providers, historical queue evidence, and every other task body |
| Approved concurrent siblings | g05.031 and g05.029 only when queue serialization protects shared closeout surfaces; no implementation paths overlap |
| Worker capability class | evidence-only release timing audit; Git, GitHub, and queue event correlation; documentation only |
| Acceptance evidence | exact queue event timestamps; Git commit times; GitHub PR/merge and Actions timestamps; the recorded 47-minute Card 154 worker interval; exact run conclusions and job critical paths |
| Review oracle | no clock starts before its stated prerequisite or ends after its stated outcome; queue delay, CI duration, review recovery, merge, Desktop gate, planning migration, operator latency, and tag execution stay distinct; missing exact chat timestamps become bounded intervals, not invented precision |
| Stop conditions | sources disagree on ordering or identity; an exact boundary cannot be established or honestly bounded; completing the audit would require changing code, workflows, release policy, a tag, or a consumer |
| Escalation owner | Chatterbox for clock semantics or follow-up planning; operator for release-policy changes |

The durable evidence already falsifies the all-green acceptance outcome: Card
154 records about 47 minutes from 12:08 to 12:55 BST through candidate
handoff; more than one hosted run occurred; and run `34348374964` repeated the
same candidate and failed its MSRV-floor test. PR run `34348370780` completed
in 2m32s and therefore meets the under-three-minute PR target. The audit must
measure the remaining boundaries exactly or as explicit bounds, then close
this task as an honest evidence stop unless every original target is proved.

## Result

Honest evidence stop. Log:
`docs/logs/2026-09-09-g05-034-release-lane-acceptance-audit.md`.

| Clause | Verdict |
| --- | --- |
| Under 30 minutes from gates green on `main` to tag request presented | fail. Conservative start `2026-09-09T12:21:26Z` (push-to-`main` run `34350208617` 11/11 green at SHA `49c9e3b2`). Tag-request lower bound `14:40:49Z` (PR #302 merge; migrations before request) = 2h 19m 23s. Desktop-only lower bound `13:39:37Z` = 1h 18m 11s. Upper bound tag-auth commit `15:46:22Z` / tag-task submit `15:47:25.593Z`. Exact Chatterbox request time unavailable; the lower bound alone fails the clause |
| One prepare attempt | pass. One authorized prepare at `11:17:50Z`; receipt digest `04a89847e14fc351bbcfdef2b48cb1d8c90282b3ea638db48043d1e8f6ab8feb` |
| One hosted run | fail. Workflow-dispatch `34345060452` green at `1fb5b16c`; repeated workflow-dispatch `34348374964` failed at the same SHA; push-to-`main` `34350208617` also qualifying. PR run `34348370780` is not a hosted gate |
| PR gate under 3 minutes | pass. `34348370780` `11:58:24Z`–`12:00:56Z` = 2m 32s |
| No rerun-to-green | not a clean pass. Green came from the first workflow-dispatch. The second same-SHA run failed Pinned MSRV floor tests (`structured_run::cancellation_and_deadline_stop_the_turn_then_join_operation_cleanup`) and was not repaired into green |

The 47-minute Card 154 interval (`11:08:09.240Z` submit → `11:55:20.297Z`
worker completion_received) is candidate handoff, not the 30-minute clock.

Desktop Card 323 and g05.038/g05.043 migrations sit inside the end-to-end
interval and are separated in the log; they are not subtracted. Coordinator
closeout sets `Status: stopped` and reconciles reserved indexes. No repair
task is opened here.
