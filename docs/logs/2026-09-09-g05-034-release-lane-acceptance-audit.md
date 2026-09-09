# 2026-09-09 g05.034 Release Lane Acceptance Audit

Date: 2026-09-09
Task: `../roadmaps/g05/034-release-lane-simplification.md`
Lane: `v0.4.4` at `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3` (tree
`1a9db12742b68e839dfebe42f0633f5d1aa0265b`; tag object `41da6c1a`)

## Result

Honest evidence stop. The `v0.4.4` lane missed the all-green g05.034 target.
Passed: one prepare attempt; PR gate under three minutes. Failed: under 30
minutes from gates green on `main` to tag request presented; one hosted run.
The "no rerun-to-green" clause is not a clean pass: a second hosted run
existed, failed, and was not repaired into green.

This audit does not repair the lane or change release policy. Coordinator
closeout sets this task to `stopped` and reconciles the reserved indexes.

## Clock Semantics

Contract 036 and the release playbook: tag request the moment a qualifying
hosted `CI` run is green — `workflow_dispatch` or a push to `main`, at the SHA
to tag or an identical tree. Pull-request runs do not qualify. If merge
produces a new SHA whose tree matches a green qualifying run, no extra
dispatch is required.

**Start ("gates green on `main`").** Two defensible instants, same verdict:

- `2026-09-09T12:17:52Z` — PR #300 merge puts the candidate tree on `main`
  while identical-tree workflow-dispatch `34345060452` is already green.
- `2026-09-09T12:21:26Z` — push-to-`main` run `34350208617` at the SHA to tag
  completes with all 11 jobs green, including Pinned MSRV floor tests.

The ledger uses `12:21:26Z` as the conservative start (qualifying run at the
exact SHA to tag). Durations from merge are also recorded.

**End ("tag request presented").** Exact Chatterbox chat time is unavailable.
Durable bounds, not invented precision:

- cannot precede events the planning handoff places before the request
  (operator-required Desktop exact-tree qualification, then unrelated
  flattened-task and backlog migrations);
- cannot follow tag-authorization planning commit `a160008e` at `15:46:22Z`
  or tag-task queue submit `15:47:25.593Z`.

The lower bound alone already exceeds 30 minutes.

## UTC Ledger

Times are UTC. Durations are mechanical from the named endpoints.

| UTC | Event | Source | Confidence |
| --- | --- | --- | --- |
| `11:08:09.240Z` | Card 154 queue task `d3390475-778e-4850-bcee-7b2ff91e487a` submitted | northstar-queue `submitted` event; matches planning handoff | exact |
| `11:12:07.277Z` | Desktop Card 323 queue task `e8bca18b-5689-4974-b5a2-503e5aaad73a` submitted (blocked on Card 154) | northstar-queue `createdAt` | exact |
| `11:17:50Z` | One authorized `release prepare` receipt (`prepared_at`); seven cheap gates passed | PR #300 review comment `5601625469` | exact |
| `11:20:09Z` | Candidate commit `1fb5b16c`; tree `1a9db127` | git committer date | exact |
| `11:20:50Z` | Qualifying `CI` workflow-dispatch `34345060452` created/started at `1fb5b16c` | GitHub Actions | exact |
| `11:24:21Z` | Last job of `34345060452` (Pinned MSRV floor tests) green | GitHub Actions | exact |
| `11:24:22Z` | Run `34345060452` completed `success`, 11/11 jobs. Planning handoff rounded this to the Documentation job at `11:23:19Z`; run completion is later | GitHub Actions `updated_at` | exact |
| `11:55:20.297Z` | Card 154 worker `completion_received` (candidate handoff). Closeout's "12:08–12:55 +0100, about 47 minutes" is this window | northstar-queue; Card 154 closeout | exact / rounded |
| `11:58:21Z` | PR #300 created | GitHub | exact |
| `11:58:24Z` | PR `CI` run `34348370780` started (`pull_request`, `1fb5b16c`) | GitHub Actions | exact |
| `11:58:27Z` | Repeated `CI` workflow-dispatch `34348374964` created at the same SHA | GitHub Actions | exact |
| `12:00:56Z` | PR run `34348370780` completed `success`; Pinned MSRV floor tests skipped | GitHub Actions | exact |
| `12:02:46Z` | Repeated run `34348374964` `run_started_at` (queued after create) | GitHub Actions | exact |
| `12:05:06Z` | Repeated run `34348374964` completed `failure`; only Pinned MSRV floor tests failed | GitHub Actions | exact |
| `12:12:21Z` | Exact-head review accepted at comment `5601625469` | GitHub | exact |
| `12:17:52Z` | PR #300 merged as `49c9e3b2`; same tree as `1fb5b16c` | GitHub + git | exact |
| `12:17:55Z` | Push-to-`main` `CI` run `34350208617` started at `49c9e3b2` | GitHub Actions | exact |
| `12:21:26Z` | Run `34350208617` completed `success`, 11/11 jobs including Pinned MSRV floor tests. **Gates green on `main` (conservative start)** | GitHub Actions | exact |
| `12:29:15.670Z` | Card 154 queue closeout done | northstar-queue `finishedAt` | exact |
| `12:29:17Z` | Desktop Card 323 dispatched | northstar-queue | exact |
| `12:37:57Z` | Card 323 blocked: no authorized isolated Grok homes | northstar-queue | exact |
| `13:07:26Z` | Card 323 resumed after operator isolated OAuth | northstar-queue | exact |
| `13:08:36Z`–`13:14:12Z` | Live Grok `1.0.4`/`1.0.5` then Claude controls | Desktop Card 323 log | exact |
| `13:16:59Z` | Desktop PR #186 created | GitHub `acowtancy/bovine-accelerator-desktop` | exact |
| `13:22:26Z` | g05.038 flattened-task planning commit; Card 323 still unfinished | git `734fa8a9` | exact |
| `13:32:02Z` | Desktop PR #186 merged as `32344306` | GitHub | exact |
| `13:39:37.037Z` | Desktop Card 323 closed out (`da9edb8d`) | northstar-queue `finishedAt` | exact |
| `14:00:09Z` | PR #301 (g05.038) created | GitHub | exact |
| `14:08:19Z` | PR #301 merged as `8dc1f161` | GitHub + git | exact |
| `14:29:00Z` | PR #302 (g05.043) created | GitHub | exact |
| `14:40:49Z` | PR #302 merged as `8ae707d5`. **Lower bound for tag request** (migrations before request) | GitHub + git | exact |
| `15:03:29Z` | Last backlog-cutover commit `4e50a7e8` | git | exact |
| `15:46:22Z` | Tag-authorization planning commit `a160008e` | git | exact |
| `15:47:25.593Z` | Tag task `a4962ded-607f-4165-8a4d-a7dc34368f79` submitted. **Upper bound for tag request** | northstar-queue `submitted` | exact |
| `15:48:17Z` | Tag-triggered `CI` run `34372646295` started at `49c9e3b2` | GitHub Actions | exact |
| `15:52:11Z` | Tag-triggered CI completed `success`, 11/11 | GitHub Actions | exact |
| `15:55:36Z` | PR #303 created (tag record) | GitHub | exact |
| `16:01:25Z` | PR #303 merged as `741e33e4` | GitHub | exact |
| `16:03:04.025Z` | Tag task closed out | northstar-queue `finishedAt` | exact |

### Measured Durations

| Interval | Duration | Notes |
| --- | --- | --- |
| Card 154 submit → worker `completion_received` | 47m 11s | the recorded ~47-minute candidate-handoff interval; not the 30-minute target |
| Cheap prepare `prepared_at` → candidate commit | 2m 19s | one prepare transaction |
| Qualifying workflow-dispatch `34345060452` | 3m 32s | last job Pinned MSRV floor tests |
| PR run `34348370780` | 2m 32s | under three minutes |
| Repeated workflow-dispatch `34348374964` create → fail | 6m 39s | 2m 20s after `run_started_at` |
| Merge → main-push CI green | 3m 34s | |
| Main-push CI `34350208617` | 3m 31s | last job Pinned MSRV floor tests |
| Gates green `12:21:26Z` → Desktop closeout `13:39:37Z` | 1h 18m 11s | already > 30 min |
| Gates green → PR #302 merge `14:40:49Z` | 2h 19m 23s | tag-request lower bound |
| Gates green → tag-auth commit `15:46:22Z` | 3h 24m 56s | tag-request upper bound |
| Gates green → tag-task submit `15:47:25.593Z` | 3h 25m 59s | |
| Merge `12:17:52Z` → tag-auth commit | 3h 28m 30s | alternate start; still fail |
| Tag-task submit → tag CI start | 51s | tag execution, after request |
| Tag-triggered CI `34372646295` | 3m 54s | |

## Clause Scores

Target: *The next release prepares in under 30 minutes wall clock from
"gates green on `main`" to "tag request presented", with one prepare
attempt and one hosted run. PR gate under 3 minutes. No rerun-to-green
anywhere.*

| Clause | Verdict | Evidence |
| --- | --- | --- |
| Under 30 minutes, gates green on `main` → tag request presented | **fail** | Lower bound 2h 19m 23s (`12:21:26Z` → `14:40:49Z`). Even the Desktop-only lower bound 1h 18m 11s fails. Upper bound 3h 24m 56s. Missing chat time cannot save the clause |
| One prepare attempt | **pass** | Review `5601625469` and Card 154 closeout: one authorized prepare, receipt digest `04a89847e14fc351bbcfdef2b48cb1d8c90282b3ea638db48043d1e8f6ab8feb`, cheap gates `fmt` 2.2s, `qa` 12.6s, `docs` 28.9s, `metadata` 0.2s, `api` 35.6s, `security` 1.2s, `source` 15.4s. No second prepare |
| One hosted run | **fail** | Two `workflow_dispatch` runs at `1fb5b16c` (`34345060452` green, `34348374964` failed) plus qualifying push-to-`main` `34350208617`. PR run `34348370780` does not qualify as a hosted gate |
| PR gate under 3 minutes | **pass** | `34348370780` `11:58:24Z`–`12:00:56Z` = 2m 32s. Required jobs green; Pinned MSRV floor tests skipped on pull_request as designed |
| No rerun-to-green | **not a clean pass** | Green came from the first workflow-dispatch, not from repairing a failure. A second same-SHA workflow-dispatch then failed and was not rerun to green. Not "no rerun"; not "rerun-to-green" |

## Separated Contributors (Not Subtracted)

End-to-end elapsed time keeps every contributor. These are named so the next
repair can aim at residue, not so the 30-minute clause can be rewritten.

**Core release mechanics (would have met the 30-minute clause if tag request
had fired on green).** Merge `12:17:52Z` to main-push CI green `12:21:26Z` is
3m 34s. Qualifying identical-tree workflow-dispatch was already green at
`11:24:22Z`. Playbook: tag request immediately on that green run.

**Exceptional pre-tag Desktop qualification (operator-required for `v0.4.4`;
playbook puts consumer smoke after the tag).** Card 323 dispatched
`12:29:17Z`, blocked `12:37:57Z` for isolated Grok OAuth, resumed
`13:07:26Z`, live legs `13:08:36Z`–`13:14:12Z`, PR #186 merged `13:32:02Z`,
closed `13:39:37Z`. About 1h 10m after gates green, including ~29 minutes of
operator login wait.

**Unrelated planning migrations.** g05.038 flattened-task planning
`13:22:26Z` (overlaps Desktop), PR #301 `14:00:09Z`–`14:08:19Z`, g05.043
backlog retirement PR #302 `14:29:00Z`–`14:40:49Z`, last cutover commit
`15:03:29Z`.

**Operator / authorization latency after migrations.** `15:03:29Z` to
tag-auth commit `15:46:22Z` (~43 minutes). Tag execution after submit is
separate and short (tag CI 3m 54s).

**Repeated hosted MSRV failure.** Run `34348374964` failed only the Pinned
MSRV floor tests job. The Test step log shows one failed test:
`structured_run::cancellation_and_deadline_stop_the_turn_then_join_operation_cleanup`
(164 passed, 1 failed). Card 154 closeout and queue closeout text say "failed
only the Pinned MSRV floor test twice"; this audit does not reproduce a
second failing test from the hosted log. The run was not repaired into green
and did not consume another prepare.

## Sources

- Contract 036 hosted-gate delegation; [release playbook](../guides/release-playbook.md)
- Card 154 closeout `docs/logs/2026-09-09-g05-card-154-closeout.md`; tag log
  `docs/logs/2026-09-09-v0-4-4-annotated-tag.md`
- Planning handoff `docs/handoffs/20260909-184605-g05-034-release-lane-acceptance-audit.md`
- GitHub PR #300 / #301 / #302 / #303; Actions runs `34345060452`,
  `34348370780`, `34348374964`, `34350208617`, `34372646295`
- Desktop PR #186 and Card 323 log (live capsule times only)
- northstar-queue events for tasks `d3390475-…`, `a4962ded-…`, `e8bca18b-…`
- git committer dates on `1fb5b16c`, `49c9e3b2`, `a160008e`, `734fa8a9`,
  `8dc1f161`, `8ae707d5`, `4e50a7e8`

No source disagreement on identity or ordering. The only loosened figures
are the planning handoff's `~11:23:19Z` run-completion rounding and the
unreproduced "twice" in the MSRV-failure prose.

## Merge

PR [#305](https://github.com/inflatable-cookie/swallowtail/pull/305)
merged 2026-09-09 as `94d8ef0753dd814445544a91cfba952c361487f8` (head
`d0f1b828`). Independent exact-head review `5606467481` recorded ready to
merge at that head with no follow-ups: three owned files only, forbidden
and reserved surfaces untouched, BST→UTC conversions and all durations
rechecked mechanically, clause scores honest, `effigy qa:docs` and `effigy
qa:northstar` passing with a clean `git diff --check`.

## Next

Closed. g05.034 is `stopped`; the generation index entry moves with it and
Next Task advances to the next already-ready canonical task. No repair
scope follows from this audit.
