---
name: version-currentness
description: >-
  Runs Swallowtail's recurring Contract 029 version-currentness lane: the
  all-route checkpoint and one-family useful-newer qualification. Use when
  the user names version-currentness, useful-newer, currentness sweep,
  qualify official stable, raise a compatibility bound, or asks to bring
  qualified version support up to date. One family per run. Do not
  bulk-bump from latest.
disable-model-invocation: true
---

# Version Currentness

Recurring operator-triggered lane. Contract 029 owns the rules. This skill
is the execution loop that the 2026-08 useful-newer sweep actually ran.

Read [reference.md](reference.md) before writing cards, fixtures, or claims.

## Authority

Read, in order, then follow the latest of these over memory:

1. this skill and `reference.md`
2. `docs/roadmaps/standing-lanes.md`
3. `docs/roadmaps/README.md` Next Task
4. Contract 029 Upgrade Workflow and Recurring Currentness Checkpoint
5. `docs/guides/version-currentness-checkpoint.md`
6. the latest currentness research (127 or its successor)
7. the family's `selection.rs`, frozen corpus, and prepared guide

Do not invent product policy. Ask when authority surfaces disagree.

## Hard rules

These are the operator corrections that made the sweep useful. Do not
rediscover them.

- **Qualify the current official stable.** The point of this lane is to
  bring qualified support up to date. Do not leave the current host or
  official stable `UnverifiedNewer` just because `AllowUnverified` already
  admits it. That posture is only for stables **above** the latest
  qualified point (g03.068).
- **One family per run.** No bulk bump from registry `latest`.
- **Identity before claim.** Freeze evidence and name the segment shape
  before editing production claims.
- **Changelog is discovery, not completeness evidence.** Release notes
  and changelogs find candidate changes. When shipped files feeding
  mapped behavior are non-byte-identical or package trees refactor,
  derive a deterministic artifact tree inventory.
- **Do not flatten families.** Adjacent products, auth paths, packages, or
  transports stay separate even when they share a vendor.
- **No provider work.** No prompt, login, live catalogue, live session,
  install, or host update unless the accepting card names that evidence.
- **Do not map unused surfaces.** New flags, nested fields, sibling
  sign-in, or changelog extras stay unmapped unless a later card owns them.
- **Keep gaps.** Unpublished stables and independently unqualified tags
  stay incompatible even when they share a git commit with a qualified
  point.
- **Decoder specimens stay** unless adapter mapping changed. Frozen
  historical corpora stay.
- **Do not reopen closed families** from this sweep.
- **Gemini stays deferred** until the operator lifts that gate. Completing
  every other family does not lift it.
- **Recheck official latest before final push.** Re-probe official latest
  after evidence repairs and immediately before push; if it moved, stop
  and ask.
- **Number from canonical pushed main.** Authority is
  `https://github.com/inflatable-cookie/swallowtail.git` `main`, not
  `origin/main`. Refresh it immediately before allocating ids and again
  immediately before push with `effigy qa:docs:roadmaps:numbers`. Fetch
  failure is a stop. Renumber onto a new unused id; do not reuse a
  canonical number on another path.
- **Ask** on major-line resets, exact-pin widening, new public operations,
  or anything Contract 029 does not settle.

## Modes

Name of the skill with no extra words → **family** mode.

| User says | Mode |
| --- | --- |
| skill name, `useful-newer`, `continue` while Next Task names a family | **family** |
| `checkpoint`, `currentness inventory`, `version sweep` without a family | **checkpoint** |
| an explicit family name | that family only |
| Next Task is Define/evidence-gate and no family remains | **stop** and ask |

**checkpoint:** follow the runbook. Write research. Index it. Do not edit
claims, matrices, or fixtures.

**family:** execute the Upgrade Workflow for exactly one production route
family. Identity card then claim card. One validation round at the claim.

**stop:** do not start a deferred family. Do not invent a generation to
house currentness. If a claim change is needed and no generation is
active, ask.

## Rank the family

If Next Task already names a family, use it.

Otherwise rank from current claims, not from a frozen research table:

1. Read every production adapter `selection.rs` claim and the feature-matrix
   version column.
2. Re-probe official latest on the family's documented channel. The last
   currentness row may already be stale.
3. Record host `--version` when the tool is on `PATH`. Missing install is
   not a gap. Do not install.
4. Prefer `AllowUnverified` families whose official and/or host stable is
   newer than the qualified ceiling, and whose host already sits on a
   qualified bound.
5. Skip current deferrals recorded in `docs/roadmaps/standing-lanes.md`.
6. Skip exact-pin / qualified-only families unless the operator asked to
   reopen them.
7. Pick one. Do not start a second family in the same run.

If official latest already equals the qualified ceiling, the family is
unchanged. Pick the next rank or stop.

## Probe

Compare the **selected mapped** CLI/protocol subset to the frozen corpus.
Help dumps, argv0, and unselected flags are not protocol changes.

Changelog is discovery, never completeness evidence. For each published hop
from the previous ceiling through intermediates to official latest:

- If all executable files feeding mapped behavior are byte-identical (with
  only package metadata or version bumps), record a compact exact
  byte-identity ledger.
- If any shipped file containing or feeding mapped behavior is
  non-byte-identical, or package trees refactor, derive a deterministic
  artifact tree inventory and classify every changed file affecting mapped
  wire shape, lifecycle, failure, permissions, usage, capability
  advertisement, session updates, config/mode, or session operations.
- Explicitly bound provider-internal and unmapped changes with why they
  cannot affect selected behavior.
- Freeze the ledger with mutation-sensitive assertions on exact file and
  key sets.
- See [reference.md](reference.md) for recursive tree extraction, fixture
  schema, and proof mechanics.

Record:

- host identity when present (version, digest, size, signature/publisher,
  source tag/commit)
- official identity (version, published time, asset digest, extracted
  binary digest when the public repo is not executable source)
- published stables between the previous ceiling and official latest
- independently unqualified older points
- the first unpublished later stable (synthetic `UnverifiedNewer` fixture)
- tree inventory diffs or byte-identity hashes, classified deltas, and bounded unmapped keys

Official artifacts live in `/tmp`, not the repo. Freeze secret-free JSON
plus a short README under the adapter `tests/fixtures/`. No credentials,
host paths, account ids, payloads, or conversation ids.

Public git may be changelog-only. Then evidence is host binary + official
release binary + changelog + artifact inventory, not an unverified source diff.

Re-probe official latest after evidence repairs and immediately before final
push; if it moved, obey the stop-and-ask rule.

Always `cd` to the Swallowtail repo root before `cargo` / `effigy`.

## Segment shape

Name one of:

- **compatible-extension** — selected mapped subset unchanged; keep
  baseline, claim ids, and behavior revisions; raise latest qualified
  through official latest; qualify published intermediates; keep gaps
- **private-milestone** — adapter-private mapping changed; new behavior
  revision on a new or split segment
- **new-driver-or-facade** — public lifecycle changed materially; stop
  and ask
- **stop** — identity needs a prompt/live session; mapped protocol
  differs; official point moved mid-run or before push; would flatten onto
  another family; would require mapping a new public operation to close
  the pin

Fixes that make already-mapped flags actually apply are compatible
extension, not a milestone.

## Edit set (claim card only)

Typical claim shape for compatible-extension:

- keep baseline and `AllowUnverified`
- replace exact `N` or extend `N..=old` to `N..=official`
- `LATEST_QUALIFIED` = official latest
- synthetic later-stable `UnverifiedNewer` = first unpublished after official
- raise every claim on that axis, or name why one stays behind

Then update tests, the prepared guide, route + lifecycle matrix rows,
feature-matrix CSV, architecture if it names the ceiling, `CHANGELOG.md`
Unreleased, research/log/roadmap indexes, the then-active generation
runway + checkpoint + numbered milestone, batch-card index, and Next
Task. Do not keep a generation open for currentness.

Do **not** edit historical research, immutable release notes, or
`provider-wide-harness-activity.json` unless a named gate fails.

Copy the nearest prior family in the then-active generation for card,
research, log, and fixture shape. Numbers: next unused research, that
generation's next roadmap, and batch card **on refreshed canonical
pushed main**. Do not take ids from this worktree or from `origin/main`.
Do not roll a generation to house currentness. If none is active, stop
and ask.

## Validation

`cargo fmt -p <adapter-package>` then the claim card's named gates:

```sh
effigy validate:focused <adapter-package>
effigy package:verify-affected <adapter-package>
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:<active-generation>
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:roadmaps:numbers
effigy qa:docs:next-action:roadmaps
```

Identity-only stops may run focused + `qa:northstar` without package
verify. Do not run workspace `qa`, broad `qa:docs` (child-index papercut),
live probes, MSRV, or consumer checks unless the card names them.

## Closeout

Glue-light: what changed, current state, failed or material validation,
next move.

Next Task lives only in `docs/roadmaps/README.md`. Verb from
`docs/policy/vision-next-task-verbs.txt`.

- more official-newer families remain → Implement the next named family
- sweep complete except current deferrals → stop currentness; resume the
  generation's actual Next Task, or ask if none
- identity named a stop → say so and ask
- no active generation and a claim change is needed → stop and ask

Auto-continuation: identity → claim in the same run. Claim → no. A later
bare `continue` resumes whatever Next Task now names.
