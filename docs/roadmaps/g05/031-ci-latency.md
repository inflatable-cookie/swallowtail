# g05.031 CI Latency

Status: stopped; honest evidence stop — PR-gate median pass, literal macOS pinned floor fail, every-PR test population pass, required-check set unproven; audit PR 306 merged as `3b4f5378` after independent exact-head review `5606858468` with no follow-ups
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-09
Depends on: Contract 036 (workflow edits need explicit operator request; granted); the `v0.4.1` tag
Vision tags: delivery, validation, CI

## Purpose

Cut the pull-request gate from ten to fifteen minutes to four or five
without weakening the release floor. Today both slow jobs run on
`macos-latest`: the stable job runs format, two full clippy passes, the whole
nextest suite, examples, metadata, and route guides serially, and the pinned
MSRV job repeats a full clippy and a full `cargo test`. Everything else
finishes in three minutes.

## Operator Authority

Contract 036 keeps workflow edits out of ordinary lanes. On 2026-09-05 the
operator explicitly requested this optimisation ("All sounds good, go for
it"). That grant covers `.github/workflows/ci.yml` and any nextest
configuration it needs, and nothing else.

## Runway

1. Card 095 restructured CI: split the stable job into parallel jobs sharing
   the rust-cache; made the pinned MSRV job clippy-only on pull requests and
   full-test on `main` pushes and workflow-dispatch (release candidates);
   sharded nextest across runners; isolated the process-spawning sidecar suites
   into their own shard; moved every check that is not Apple Silicon
   verified-target evidence to Linux runners.

Folded evidence (g05.038): card 095 work complete through PR 230 merged as `ba8275eb`.

## Boundary

The release floor keeps its full macOS pinned clippy-plus-test run on
`main` pushes and release-candidate dispatches. Required-check names used by
the merge gate change only with the coordinator's agreement. No test is
removed or weakened; no gate command in `config/release.toml` changes.

## Folded card evidence (g05.038)

Card 095 completed against `v0.4.1` tagged at `c3cce750` under the operator
workflow authority of 2026-09-05, through PR 230 merged as `ba8275eb`.
Delivered: parallel stable jobs sharing the rust-cache; nextest sharded with
the process-spawning suites isolated; MSRV clippy-only on pull requests with
the full pinned run on `main` pushes and workflow-dispatch; non-target checks
on Linux; every prior step still running on every PR; before-and-after timings
recorded on one representative PR; required-check names documented and agreed
with the coordinator before any branch-protection change. Card 094's remaining
sweep (PR 227 and follow-ups) ran as the approved concurrent sibling. No step
removed; release floor intact on `main` pushes and dispatches.

## Acceptance

The Card 095 PR-gate work ran. The original four-clause target is not fully
proven. See Result.

## Dispatch

| Field | g05.031 acceptance audit |
| --- | --- |
| Readiness | ready; Card 095 implementation merged through PR 230 at `ba8275eb`; current evidence is sufficient to score the original target without another workflow mutation |
| Prerequisites | Contract 036; PR 230 and its exact-head review; current `.github/workflows/ci.yml` and `.config/nextest.toml`; PR and Actions history through PR 305; g05.034 audit evidence for workflow-dispatch and push-to-`main` runs |
| Completion conditions | one source-linked audit scores each Acceptance clause independently; latency uses the latest 20 merged PRs through PR 305 with a completed pull-request `CI` run at the accepted head, extending backward only when a named exclusion leaves fewer than 10 observations; report median, p75, range, and code/docs strata; compare the pre-Card-095 command/test inventory with current PR coverage; verify current push-to-`main` and workflow-dispatch floor execution; verify required-check names and strength from durable repository/GitHub evidence; record contradictions rather than rewriting the target; update this task and one log |
| Owned mutable paths | this task; one new `docs/logs/2026-09-09-g05-031-*.md`; one `docs/logs/README.md` index line; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`; queue coordinator edits these after merge |
| Forbidden paths | `.github/workflows/**`; `.config/nextest.toml`; Rust; Cargo manifests and lockfile; Effigy configuration; release configuration and scripts; contracts; release notes and tags; every other task body |
| Approved concurrent siblings | g05.029 only when queue serialization protects shared closeout surfaces; implementation paths do not overlap |
| Worker capability class | evidence-only CI acceptance audit; GitHub Actions/PR and Git history correlation; documentation only |
| Acceptance evidence | exact Actions run/job timestamps and conclusions; event and accepted-head identity; pre/post workflow command mapping; current trigger and runner conditions; PR 230 review/merge evidence; branch-protection or equivalent durable required-check evidence |
| Review oracle | no cherry-picked timing sample or hidden exclusion; “typical” is scored from the median against about five minutes while p75 and outliers remain visible; every pre-Card-095 PR command/test has one current PR execution path; the literal macOS floor clause is not weakened to platform-neutral after the fact; missing required-check visibility is reported as unproven, not assumed |
| Stop conditions | evidence identity or ordering conflicts; GitHub cannot expose enough accepted-head runs to score latency; an Acceptance clause needs a workflow, policy, branch-protection, release, or runtime mutation |
| Escalation owner | Chatterbox for clock/sample semantics or follow-up planning; operator for workflow, branch-protection, release-policy, or target changes |

## Result

Honest evidence stop. Log:
`docs/logs/2026-09-09-g05-031-ci-latency-acceptance-audit.md`.

| Clause | Verdict |
| --- | --- |
| Typical PR all-green in about five minutes | pass. Median 2m 43s over the latest 20 merged PRs through #305 with a completed pull-request `CI` run at the accepted head. p75 2m 46s. Range 2m 30s–4m 49s. No merged-PR exclusions |
| Full macOS pinned floor on `main` pushes and release-candidate dispatches | fail. Current pinned clippy and pinned tests both run on `ubuntu-latest`. Reused g05.034 dispatch/main/tag runs execute that floor on Linux. Process-spawning nextest on `macos-latest` is not the pinned floor |
| Every test that ran before still runs somewhere on every PR | pass. Pre-Card-095 nextest default suite maps to current `ci` shards ∪ `ci-process`. Skipped PR `cargo +MSRV test` is a pinned duplicate of that population |
| Required-check set documented and unchanged in strength | unproven. `GET /branches/main/protection` 404; rulesets empty. Card 095 documented names; GitHub does not enforce them; names drifted (3 shards → 2) |

Coordinator closeout 2026-09-09 sets `Status: stopped`: PR 306 merged as `3b4f5378`
after independent exact-head review `5606858468` (ready to merge, no follow-ups). No workflow, branch-protection, or target repair is opened here.

### Card 104 held gate (task-owned)

Card 104 stopped as a task-owned held gate; it was ready concurrent with the
g05.029 runway. Recorded basis: `v0.4.2` tagged with cards 094 and 103 merged.
Scope: four surfaces classified and fixed at the class; 20+ loaded runs per
binary clean; card 094's list annotated; loaded-run logs and classification
table with anchors as evidence. `swallowtail-adapter-claude-agent` stays with
the g05.029 runway; every other crate, baselines, and contracts stay out. A
real leak needing a shared runtime change stops the gate.

### Card 139 held gate (task-owned)

Card 139 stopped as a task-owned held gate. Recorded basis: the 2026-09-08
papercut on current `main`. Scope: reproduction under load or an anchored
explanation; bounded process output and exit evidence on fixture setup failure;
deterministic startup and listener readiness signals; 20+ loaded runs clean;
papercut retired; the loaded-run log naming the failure's own cause as
evidence. Every other crate, baselines, contracts, and the live gate stay out.
A cause in shared runtime stops the gate.

### Card 141 completed (folded)

Card 141 completed through PR 292 merged at
`b68a1ccc757eb6adf768fb7ae3d330c5daef1547`, on card 139 merged and current
`main`. Delivered: close no longer pays a fixed `IO_TIMEOUT`;
before/after measurements recorded; watcher behaviour unchanged; every teardown
guarantee preserved; papercut retired. Evidence: before/after close
measurements on registered and watcher cases.
