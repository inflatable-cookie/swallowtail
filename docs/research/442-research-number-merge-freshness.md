# Research 442: Research-number Merge Freshness

Status: investigation and proposal; no enforcement authority changed.
Date: 2026-10-11.
Before adding this record, `effigy qa:docs:research:numbers` passed and the
canonical research index ended at 441; this record is 442.

## Finding

The strict collision check worked on the tree and canonical `main` snapshot it
observed. It did not reserve the number or tie a successful result to the base
that was eventually merged. For 373 and 379, retained PR runs show the
uniqueness check passed before a competing main update, then the stale PR
merged with both records. For 382, the same check-to-merge interval is visible
and the final merge tree contains both records. The historical check runs and
merge parents establish a stale-proof race; the checker itself does not need
an allocation exemption.

## Snapshot and publication

[`check-research-number-collision.py`](../../scripts/check-research-number-collision.py)
first scans the checkout's `docs/research` paths and rejects duplicate number
slugs in that tree. Its normal path then resolves the advertised canonical
`main` commit, fetches that exact SHA into an isolated Git store, and compares
the checked tree with that snapshot. A refresh failure is closed. The checker
does not reserve a number, print the fetched base SHA, or attach it to the
result. `--local-base` remains test-only.

The CI workflow runs on `pull_request` and `push` to `main`; the
`research-numbers` job runs the strict checker and its mutation suite.
`actions/checkout` does not override its ref. GitHub documents that a
`pull_request` workflow's `GITHUB_SHA` is the latest merge commit of that PR's
merge ref, while the Check Run records below identify the PR head SHA. The
checker fetches canonical `main` when the job executes. A later base update
does not revise the already-published result by itself. [GitHub's event
documentation](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)
describes the default pull-request SHA; [the workflow](../../.github/workflows/ci.yml)
and checker establish Swallowtail's use of it.

## Historical merge traces

All times below are UTC. The source PRs selected the same numbers from branch
trees that did not yet contain the competing record. In each case, the
uniqueness check was green before the competing main merge; the PR then merged
without another successful uniqueness run against the updated main.

| Number | Passing check and intervening merge | Actual merged tree | Post-merge evidence |
| --- | --- | --- | --- |
| 373 | PR [#418](https://github.com/inflatable-cookie/swallowtail/pull/418) workflow run [37694057966](https://github.com/inflatable-cookie/swallowtail/actions/runs/37694057966/job/113041144182) passed at 2026-10-07 22:07:13 on head `c161228ef81caeaebdd365222e4c1c2bb1ca7c76`, with base `3b4d6ecd2eea82473e0db78ce404cfc1104e6c5e`. PR [#416](https://github.com/inflatable-cookie/swallowtail/pull/416) merged at 22:36:59, adding `373-claude-agent-acp-0-87-0-identity.md`. | PR #418 merged at 22:46:29 as `d3997d7367a6ae44369bc6be836926b9a67dcf6e`, parents `817e5c09c0480e947ec1d83071a79183aecc290d` and `c161228ef81caeaebdd365222e4c1c2bb1ca7c76`, tree `d9c10de91022f412c635f733399b6eda4d2bae55`. The tree contains that ACP record and `373-claude-code-2-1-293-headless-currentness-stop.md`. | The check on merged commit `d3997d7367a6ae44369bc6be836926b9a67dcf6e` failed at 22:46:40 with both slugs. PR [#420](https://github.com/inflatable-cookie/swallowtail/pull/420) later kept the ACP record at 373 and moved the Claude Code record to 374. |
| 379 | PR [#426](https://github.com/inflatable-cookie/swallowtail/pull/426) workflow run [37715665000](https://github.com/inflatable-cookie/swallowtail/actions/runs/37715665000/job/113111432905) passed at 2026-10-08 02:00:01 on head `ba307022de5cdf74f5453ba48705b7122960a836`, with base `c9ec2f532a1c0ae4bde3f3a52166181416a075dd`. PR [#424](https://github.com/inflatable-cookie/swallowtail/pull/424) merged at 02:12:26, adding `379-antigravity-cli-1-3-1-catalogue-identity.md`. | PR #426 merged at 02:17:14 as `fb7377eb0db6b5f4e11ef925344507dc53195261`, parents `3b390688057f375eae04a39bd682cded42c065db` and `ba307022de5cdf74f5453ba48705b7122960a836`, tree `8a5f3433a02dfe33f9f1989f9d66ceb8db909f88`. The tree contains the Antigravity record and `379-ollama-0-40-0-currentness-stop.md`. | The check on merged commit `fb7377eb0db6b5f4e11ef925344507dc53195261` failed at 02:17:27 with both 379 slugs. PR [#436](https://github.com/inflatable-cookie/swallowtail/pull/436) renumbered Antigravity to 380, moved Grok 382 to 385, and removed the 382 allowlist. |
| 382 | PR [#427](https://github.com/inflatable-cookie/swallowtail/pull/427) workflow run [37724526182](https://github.com/inflatable-cookie/swallowtail/actions/runs/37724526182/job/113139561371) passed at 03:49:29 on head `ba3f3695534b73cc1b7b86b2ebdb6956eaf9458a`, with base `3cd772f0f332f344a2ca70d82f0d45f09c80e853`. PR [#429](https://github.com/inflatable-cookie/swallowtail/pull/429) merged at 04:07:52, adding `382-oh-my-pi-18-8-3-identity-and-qualification.md`. | PR #427 merged at 04:11:58 as `a40eb5040340d081667e185cca2a00c9f2e2e314`, parents `bb4f967c91ed2d629a9eb304db2f2d30ebc4178` and `ba3f3695534b73cc1b7b86b2ebdb6956eaf9458a`, tree `b3677819f369df0e7ba7aa8a196c64fa66800b29`. The tree contains that Oh My Pi record and `382-grok-build-acp-1-0-46-identity.md`. | The post-merge check on `a40eb5040340d081667e185cca2a00c9f2e2e314` failed at 04:13:14 on 379 first. Its log does not isolate 382. The merge parents and tree prove the 382 duplicate; PR #436 moved Grok intact to 385 and removed the 382 allowlist. |

The 373 base before PR #416 was `3b4d6ecd`, which had no 373 record. The 379
check completed before PR #424's merge, and the 382 check completed before PR
#429's merge. The final merge commit parents show each PR branch was integrated
with the newer main. These are more than collisions discovered in an index:
the green check and the later conflicting base are separately timestamped.

## Queue freshness boundary

Read-only Queue source inspection at `e9d328bc7ff5e19ddc9759d84040f1d5b15a4e93`
shows two distinct paths:

- `server/forge.ts` verifies the current PR head and visible check rollup. A
  `BEHIND` but mergeable PR can proceed when the branch rules do not require
  the head to include the current base.
- `server/resolution.ts` calls the prospective-merge gate after review. If
  there is no applicable hook and no repository validation setting, it returns
  without a command. Q-002 says Swallowtail has no separate Queue validation;
  GitHub CI owns merge validation.
- When configured, Queue's prospective-merge hook builds a candidate from the
  exact integration base and reviewed head, then discards the result if the
  base moves. Project validation is keyed to that base and head and also
  rechecks base movement. That optional candidate proof can close this gap;
  it is not active for Swallowtail under Q-002.

At investigation time, the GitHub classic branch-protection endpoint returned
404 and the active rulesets query was empty. This is a current-state
observation, not a reconstruction of the rules at each historical merge.
The retained Check Runs publish `head_sha` values for the PR branch commits;
they do not record the fetched canonical base SHA. Historical branch-protection
snapshots do not establish whether the job was a GitHub-required status at
those merge times. The stale successful check, later base update, and merged
tree are established independently of that missing policy record.
GitHub distinguishes required checks that require an up-to-date branch from
those that do not in its [protected-branch documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches).

## Bounded fake and strict-allocation counterexample

[`research-number-freshness.py`](../../scripts/tests/research-number-freshness.py)
creates two local PR heads from one temporary canonical base. Both independently
add different `900-*` records and pass the production checker against that
base. After alpha advances canonical `main`, beta's successful result remains
associated with its unchanged head in the fake status ledger. The fake records
`BEHIND`, mergeable, and no-up-to-date-rule values as fixture inputs informed
by the Queue source inspection; it does not execute Queue's gate or model
GitHub check publication. The checker then rejects beta against refreshed
main, and rejects the current prospective tree containing both records. The
model uses a fresh temporary directory, local Git repositories, and a
120-second timeout per subprocess; it makes no network request and never uses
`--local-base`.

Reproduce with:

```sh
effigy qa:docs:research:freshness:compile
effigy qa:docs:research:freshness:test
```

This counterexample preserves strict allocation: with a current canonical
snapshot, the second distinct slug for an occupied number fails. The defect is
reusing a prior green result after the base changes.

## Smallest follow-up

Queue should own one base-bound uniqueness proof on the exact prospective
merge candidate, keyed by `(base_sha, reviewed_head)`. Run the existing strict
`effigy qa:docs:research:numbers` selector there; discard and rerun the result
if either input moves. Keep the default canonical-main refresh and strict
collision failure. Do not use `--local-base` for enforcement or add an
allocation exemption.

Enabling that selector through Queue changes Q-002 and requires a separately
approved Queue configuration task. A GitHub strict-up-to-date rule or merge
queue would also require branch-protection/workflow authority and a separate
task. This investigation changes neither path. The Queue owner should create
the follow-up; the worker report records that recommendation.

## Finite unknowns

- The retained pull-request Check Run records identify each PR head SHA. The
  exact temporary pull-request merge-ref SHA used by each successful run is
  not retained in this repository's Git history.
- Repository/Queue records inspected here do not identify which actor or merge
  mechanism performed each historical merge. They do prove check completion,
  intervening main merge, final merge parents/tree, and post-merge check
  results where reported above.
- The #427 post-merge check reported 379 first, so it does not independently
  establish the 382 checker's failure output. The exact #427 merge tree and
  subsequent #436 repair establish the 382 collision itself.
