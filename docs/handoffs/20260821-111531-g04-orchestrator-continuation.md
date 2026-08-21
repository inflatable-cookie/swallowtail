---
title: g04 orchestrator continuation
kind: northstar-handoff
status: active
owner: Tom
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260821-111531-g04-orchestrator-continuation.md
tags: [coordination, handoff, orchestrator]
---

## What This Thread Was Doing

This was Swallowtail’s Northstar orchestrator for g04: route readiness and
connection admission. The thread closed g03, opened g04, shipped the 057
lifecycle through first and second proofs, parked hosted OAuth, moved Contract
029 currentness to a standing lane, inventoried remaining addable routes, and
dispatched g04.023.

Near the end it also reviewed standing-currentness PRs (Codex, then Qwen and
Ollama) without letting them steal the generation pointer. The last concrete
work was restacking three open PRs onto current `main` so they can merge in
order.

This note is for a fresh orchestrator thread. It is not a worker dispatch.

## Why It Matters

g04 is still the active generation and should stay open toward 30-50 numbered
roadmaps. The 047 presentation-metadata seam is sitting in a reviewable PR.
Two official-stable qualifications are stacked behind it. Landing them cleanly
keeps the generation pointer honest and keeps currentness from rewriting the
front door.

## Current State

Here is the short version of where things stand:

- **Done:** g04.001–022 are on `main`. `v0.3.3` still peels to `51d18620`.
  Six addable routes are realized. Research 171 classified the remaining 47
  production routes (6 reuse, 26 later descriptor work, 15 gated) and named
  g04.024 as hosted API-key `kimi-platform.chat`. Hosted URL-open OAuth is
  parked. Contract 029 currentness is a standing lane. Codex CLI `0.149.0` is
  qualified (g04.025, PR 19, `25fc3e35`; closeout `d8e3471a`).
- **Still open:** merge the restacked stack, then close out 023 and the two
  currentness families. After that, compile g04.024 implementation cards.
  Do not start Kimi wiring until those cards exist.
- **Active spec lane:** none. Contracts 020, 047, and 057 govern 023.
  Contract 029 governs the currentness PRs.
- **Current batch card:** 023 cards 065–067 are complete on the worker
  branch, awaiting merge. 024 implementation cards are not compiled.
- **Canonical refs:**
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/README.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/README.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/standing-lanes.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/047-configured-provider-instance-catalogue.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/057-route-readiness-and-connection-admission.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/029-interface-version-qualification-and-compatibility.md`
- **Remaining continuation envelope:** after the merges, compile 024 cards;
  hosted OAuth stays parked; Gemini stays deferred.
- **Lane budget / pause signal:** g04 stays active toward 30-50. Do not roll
  over. Currentness does not keep the generation open.
- **Pushed `main`:** `d8e3471a` —
  `docs(g04): record merged Codex 0.149.0 currentness`
- **Key files:**
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/023-047-presentation-metadata.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/024-hosted-api-key-kimi-platform-chat.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/research/171-further-addable-route-inventory.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-19-route-readiness-facade.md`

### First task: merge this stack

Operator authorized these merges as the new thread’s first task. Recheck CI
on the restacked heads, then fast-forward in this order. Previous merges used
local `git merge --ff-only` plus `git push origin main` because `gh pr merge
--ff` is not available.

| Order | PR | Head | Base | What it is |
| --- | --- | --- | --- | --- |
| 1 | [23](https://github.com/inflatable-cookie/swallowtail/pull/23) | `75daf124` | `main` | g04.023: project 057 `instance_label` into 047 |
| 2 | [21](https://github.com/inflatable-cookie/swallowtail/pull/21) | `dc06292f` | PR 23 branch | Qwen headless `0.21.15` (Research 173, g04.026, cards 072–073) |
| 3 | [22](https://github.com/inflatable-cookie/swallowtail/pull/22) | `ddd5c766` | PR 21 branch | Ollama attached `0.32.15` (Research 174, g04.027, cards 074–075) |

At handoff time all three were MERGEABLE and UNSTABLE (CI still running after
the restack). Do not merge a red head. Independent review already landed as
comments; restack addressed the requested changes. Re-read the diffs against
current `main` if the heads moved.

After each merge, record it. After 23, close g04.023 and leave Next Task on
compiling 024 cards, not on starting Kimi. After 21 and 22, record standing
currentness without rewriting the generation pointer.

Shared GitHub identity is `betterthanclay`. Post orchestrator verdicts as
comments; do not use formal self-approve.

## Boundaries

Please keep the next pass within these boundaries:

- **In scope:** merge 23 → 21 → 22 when green; close out 023 and the two
  currentness families; compile g04.024 cards; keep g04 pointed at 30-50;
  review later worker PRs in the same orchestrator loop.
- **Out of scope:** starting Kimi implementation before 024 cards exist;
  hosted URL-open OAuth; OpenHands production wiring; advertising
  `claude-code.headless`, `claude-code.response-only`, or `llama-cpp.owned`
  from sibling addable rows; marking every leftover production route
  addable; Gemini requalification; crates.io or GitHub Release; rewriting
  `release-baselines/public-api-0.3.3`.
- **Repo constraints:** Follow `/Users/tom/Dev/projects/swallowtail/AGENTS.md`.
  Prefer `effigy` for supported work. Validation for ordinary batches is
  explicit package scope, one to four names. Glue-light writing from
  `/Users/tom/Dev/projects/northstar/docs/policy/internal-writing-style.md`.

## Important Context

- **Planning lineage:** g04 is the route-readiness generation. 047 is the
  selection snapshot; 057 is the lifecycle facade after admission. Overlay
  markers stay overlay. Accent color stays consumer-owned.
- **How the plan fits the system:** 023 adds one optional already-stored
  field, `instance_label`, onto the 047 snapshot. It must not change
  `Ready` / `NotReady`. Currentness is Contract 029, one family per run,
  compiled into the then-active generation without keeping it open.
- **Decisions and preferences:** merge only with operator authorization —
  this handoff *is* that authorization for 23, 21, and 22. Hosted OAuth
  stays parked until a named production route needs URL-open plus
  loopback. Do not invent catalogue `provider_id`. Next Task lives only in
  `docs/roadmaps/README.md`. Standing currentness must not rewind it to an
  old g04 card.
- **Open tensions:** `qa:docs:next-action:roadmaps` wants a verb from
  `docs/policy/vision-next-task-verbs.txt`. “Worker in flight” is what
  `main` currently uses; do not “fix” that by restoring an old card. After
  023 merges, pick an allowed verb for compiling 024 cards. The g04
  generation-runway row “close remaining 057/047 seams and expand addable
  coverage” should stay planned until 024 actually expands addable
  coverage. Cards 070–071 are unused numbers; that is a gap, not a task.

## Suggested Next Move

Start by reading the `northstar` skill, then `references/router.md` and
`references/modes/orchestrator.md`. Load Swallowtail `AGENTS.md` and the
canonical refs above. Fetch `origin/main` and confirm it is still
`d8e3471a` or a later closeout you understand.

Then take the merges. Recheck CI on 23, 21, and 22. If all five jobs are
green and the stack is still MERGEABLE, fast-forward 23 onto `main`, push,
close out 023, then 21, then 22. If CI is red or a head moved, stop and
review that PR again instead of merging.

Take a moment to read the named canonical files before changing anything. If
one of the open questions changes the shape of the work, pause and bring that
question back rather than quietly choosing for the user.

## Completion Protocol

This handoff exists because another orchestrator thread is taking over.
Before you finish a meaningful batch, please:

1. Confirm `docs/roadmaps/g04/023-047-presentation-metadata.md` and cards
   065–067 reflect merge reality, not “complete on worker branch”.
2. Confirm `docs/roadmaps/README.md` Next Task, `docs/roadmaps/g04/README.md`,
   `docs/logs/README.md`, and `docs/roadmaps/standing-lanes.md` match what
   actually landed.
3. After the stack is on `main`, the in-bounds next card work is compiling
   g04.024 implementation cards. Named addable implementations wait until
   those cards are ready. Hosted OAuth stays parked.
4. Keep g04 open toward 30-50. Do not roll over because a phase ended.
5. Call out CI failures, restack drift, or a Next Task fight with currentness
   rather than rewriting the front door to make a docs gate pass.
6. Leave one clear next task in `docs/roadmaps/README.md` only.

If the same thread can continue after compaction, do not create another
handoff just for that reason.
