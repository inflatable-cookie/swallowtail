---
title: g05 orchestrator continuation handoff
kind: northstar-handoff
status: active
owner: Tom
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260831-130011-g05-orchestrator-continuation.md
tags: [coordination, handoff, orchestrator]
---

## What This Thread Was Doing

This thread carried Swallowtail's Northstar orchestration through the Contract
061 two-route proof and the planning-only Batch 9.4 package expansion. PR 131
merged the runtime composer, portable conformance, all 36
`codex.app-server` rows, and all 15 `openai.realtime` rows at `fdd2b018`.
Batch 9.4 then partitioned the remaining 716 census rows into 12 bounded
complete-package candidates. Only candidate A passed the readiness rubric and
became card 023.

The herdr dispatch machine is now leaving the floor. No card 023 worker was
started, no worker handoff was published, and no implementation has begun.
Tom is continuing locally from `/Users/tom/Dev/projects/swallowtail` with a
fresh Sol/orchestrator thread.

## Why It Matters

Contract 061 gives consumers exact route-feature and lifecycle-control truth
without adapter downcasts, route enumeration, or inferred provider state. The
next tranche must complete the two adapter packages already started by PR 131
while preserving exact route, operation, source, access, lifecycle, and
negative evidence boundaries.

The separate Sweep currentness PR remains useful but cannot displace this
runway. Its implementation widening is sound; its Contract 029 planning and
standing-lane bookkeeping are not yet honest.

## Current State

Here is the short version of where things stand:

- **Done:** The clean planning base before this handoff is
  `9622ca8d27355d5e2e6798af50a42ef1ca922b5b`. This checkout and its
  `origin/main` ref matched that exact commit. It contains the Batch 9.4
  package-expansion planning, card 023, and the unchanged card 023 Next Task.
- **Done:** Card 022 and PR 131 merged at `fdd2b018`. The exact 51-row Contract
  061 two-route vertical is complete.
- **Done:** Papercut PR 132 merged at `811db499`. That closeout grants no
  execution authority and did not move the Next Task.
- **Still open:** Ready g05.009 card 023 is the sole Next Task: one
  provider-free two-package PR for the remaining 35 `codex.exec` and 24
  `openai.background` census rows. It is ready but not dispatched. No worker
  is in flight and no worker-pr-loop handoff exists for it.
- **Still open:** Sweep PR 130,
  `https://github.com/inflatable-cookie/swallowtail/pull/130`, remains
  extracurricular and merge-withheld. Its reviewed head is
  `93d893874dbaff20a4cda4084020c7db88663ebd`. The useful-newer widening to
  Codex `0.151.0` is sound, but the Contract 029 one-family milestone/cards,
  g05 indexes, and `standing-lanes.md` are stale; Claude Code still appears to
  be the latest completed family. Helm posted the withheld verdict as comment
  `5477482678`. Leave PR 130 open for the local thread.
- **Active spec lane:** None. Contract 061 is active; Batch 9.4 planning
  evidence owns the candidate partition.
- **Current batch card:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/batch-cards/023-contract-061-codex-openai-package-completion.md`
- **Canonical refs:**
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/001-working-rules.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/037-prepared-consumer-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/047-configured-provider-instance-catalogue.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/057-route-readiness-and-connection-admission.md`,
  and
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/061-consumer-route-feature-and-control-projection.md`
- **Remaining continuation envelope:** Card 023 only. Candidates B-L remain
  unnumbered planning rows. Batch 9.5 is not compiled.
- **Lane budget / pause signal:** One ready card, one provider-free PR, then an
  exact-head orchestrator checkpoint. No automatic continuation.
- **Dispatch state:** The herdr machine is shutting down this floor. No further
  workers will be dispatched from herdr.
- **Repository state before this handoff:** clean `main` and `origin/main`
  both resolved to `9622ca8d27355d5e2e6798af50a42ef1ca922b5b`. The handoff
  commit will advance `main`; trust the tracked handoff in current
  `origin/main` and require local `HEAD == origin/main` before continuing.
- **Key files:**
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/README.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/batch-cards/023-contract-061-codex-openai-package-completion.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-31-contract-061-batch-9-4-package-expansion.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/logs/2026-08-31-g05-009-batch-9-4-package-expansion-compiled.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/standing-lanes.md`

## Boundaries

Please keep the next pass within these boundaries:

- **In scope:** Resume orchestration on Tom's Mac from the current pushed
  `main`; keep card 023 as the sole Northstar task; coordinate its exact
  provider-free 35/24-row implementation and later review only when Tom chooses
  the local execution path. Keep PR 130 separate and open until its stale
  Contract 029 planning surfaces are repaired and independently re-reviewed.
- **Out of scope:** Do not treat PR 130 or PR 132 as the Next Task. Do not
  contact a provider, promote candidates B-L, compile Batch 9.5, restart a
  watcher or skill-visibility lane, reopen PR 127, close the generation, or
  merge PR 130 without a new exact-head merge-authorized verdict.
- **Repo constraints:** Follow
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md` and the canonical contracts
  above. Preserve the existing runtime/testkit/core public baseline. Use the
  exact validation tier in card 023. Keep the sole active Next Task pointer in
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/README.md`.

## Important Context

- **Planning lineage:** Contract 061 promoted the reviewed 767-row census. PR
  131 proved the first 51 rows. The Batch 9.4 checkpoint assigned all 716
  remaining rows exactly once and promoted only the Codex/OpenAI remainder as
  card 023.
- **How the plan fits the system:** Contract 047 remains the immutable
  configured-instance selection snapshot, Contract 057 owns admission and
  readiness before it, Contract 037 owns exact preparation, and Contract 061
  composes descriptive truth only. Adapter contributions cannot create
  execution, mutation, acknowledgement, routing, or provider authority.
- **Decisions and preferences:** Card 023 may add the established contribution
  method only to `CodexPreparedExec` and `OpenAiPreparedBackgroundRun`. It must
  withhold documentation-only, incompatible-operation, route-wide, lifecycle,
  and unobserved rows at construction. OpenAI Models and background
  reconciliation do not become background structured-run evidence or a new
  composer input.
- **Open tensions:** Exact 35/24 ledgers may expose a row that the two named
  prepared facades cannot prove. That row stays withheld. If completion would
  require a new runtime/testkit/core type, source kind, composer rule, bound,
  active-observation seam, provider operation, or contract amendment, stop and
  return to planning.
- **PR 130 posture:** The extracurricular code widening was accepted in
  principle, but merge authority remains withheld until the one-family
  milestone/cards, g05 indexes, and standing-lane latest-family pointer match
  the Codex `0.151.0` result. Do not fold that repair into card 023.
- **Validation posture:** Batch 9.4 planning passed `effigy qa:docs`, `effigy
  qa:northstar`, the exact 716-row partition assertion, and `git diff --check`.
  The inherited doctor baseline remains 391 god-file findings, one stale graph
  warning, and one generated-source warning.

## Suggested Next Move

Start on Tom's Mac by reading `/Users/tom/.agents/skills/northstar/SKILL.md`,
then its router and orchestrator mode. In
`/Users/tom/Dev/projects/swallowtail`, fetch and fast-forward `main`, confirm
the checkout is clean, confirm
`9622ca8d27355d5e2e6798af50a42ef1ca922b5b` is an ancestor, and confirm local
`HEAD` equals `origin/main` and contains this tracked handoff.

Then read the roadmap front door, g05.009, card 023, Contract 061, the Batch
9.4 planning checkpoint, and its closeout log. Card 023 remains ready but
undispatched. Let Tom choose the local execution arrangement before any code
work. Keep PR 130 as a separate review-withheld item; do not let it move the
Northstar pointer.

Take a moment to read the named canonical files before changing anything. If
implementation exposes a new public or contract decision, pause and bring it
back to planning rather than choosing it locally.

## Completion Protocol

This handoff exists because a fresh local Sol/orchestrator thread is taking
over. Before that thread finishes a meaningful batch, please:

1. Keep card 023 as the sole active Next Task until its one provider-free
   two-package PR passes exact-head review and the operator separately
   authorizes merge.
2. Treat card 023's 35 `codex.exec` and 24 `openai.background` ledgers as
   independent exact totals. Withhold unsupported rows at construction; do not
   emit then filter, borrow another route's truth, or infer provider-effective
   state.
3. Run every validation command named by card 023. Do not run a live probe or
   contact a provider.
4. Review any card 023 PR independently against Contract 061, the card's review
   oracle, the exact census rows, changed files, and settled checks. Post the
   verdict on GitHub. Merge remains separately operator-authorized.
5. After card 023 closes, return to the Batch 9.4 planning checkpoint before
   promoting any candidate B-L. Do not precompile Batch 9.5.
6. Keep PR 130 extracurricular and merge-withheld until its Contract 029
   milestone/cards, g05 indexes, and `standing-lanes.md` state are repaired,
   re-reviewed at the exact head, and checks are settled. Do not merge it as
   part of card 023.
7. Reconcile the card, milestone, batch-card index, g05/generation indexes,
   log index, and sole roadmap Next Task after any accepted merge. Leave one
   honest next move and call out unresolved risks plainly.

If the same local thread can continue after compaction, do not create another
handoff just for that reason.
