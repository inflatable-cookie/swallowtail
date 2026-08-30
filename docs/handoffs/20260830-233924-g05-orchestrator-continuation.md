---
title: g05 orchestrator continuation handoff
kind: northstar-handoff
status: active
owner: Tom
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260830-233924-g05-orchestrator-continuation.md
tags: [coordination, handoff, orchestrator]
---

## What This Thread Was Doing

This thread was Swallowtail's Northstar orchestrator through the end of g04
and the first six g05 roadmaps. The latest work focused on process watchers.
One authorized Claude Code turn failed to create a host watcher, so the first
route claim stayed unpublished and the prototype was withheld. We then planned,
reviewed, repaired, and merged the credential-free lifecycle and proof
infrastructure needed to make any future live attempt meaningful.

PR 126 took three review revisions. It landed by fast-forward at `c8691e84`.
The post-merge closeout is `9a5b7229`. There are no open pull requests, and
the planning checkout was clean and matched `origin/main` before this handoff.

The thread also preserved an operator-requested consumer route-feature and
option projection idea in triage. Its 767-row census is complete enough for a
contract discussion, but it is not runtime authority and has not been promoted.

## Why It Matters

Consumer applications need to see real process activity while an agent is
working, and the agent must not finish a turn while watcher-owned work is still
active. A final success string is not proof of that lifecycle. The merged
repair now observes ordered watcher activity independently of provider stdout
and can distinguish direct gate use from actual Stop-hook re-entry.

That gives the operator a real choice: spend one newly authorized provider
turn on the repaired acceptance oracle, or leave the claim stopped and move to
the next planning question. Neither choice has been made yet.

## Current State

Here is the short version of where things stand:

- **Done:** g05.006 card 019 is merged through PR 126 at `c8691e84`. It adds
  the host-owned lifecycle feed, one provider-neutral activity projection,
  bounded turn-scoped proof facts, the direct-gate counterexample, joined
  cleanup, and panic-safe live workspace cleanup. All five exact-head CI jobs
  were green. The repair did not contact Claude or publish a watcher claim.
- **Still open:** The operator decides whether to authorize one fresh exact
  Claude Code `2.1.251` watcher acceptance turn. If yes, the orchestrator must
  first compile a new bounded live-attempt card and worker handoff; the consumed
  card 011 attempt and its old worker handoff must not be reused.
- **Active spec lane:** None. Contracts 044, 059, and 060 are active authority.
  The consumer route-feature projection remains an open triage note, not a
  promoted spec or contract.
- **Current batch card:** None ready. Card 019 is complete. Card 011 is complete
  as an evidence stop, and g05.003 remains stopped with the first route claim
  withheld.
- **Canonical refs:**
  - `/Users/tom/Dev/projects/swallowtail/AGENTS.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/README.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/README.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/contracts/044-observable-agent-activity-and-disclosure.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/contracts/059-operation-scoped-process-watchers.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/contracts/060-operation-scoped-watcher-http-bridge.md`
- **Remaining continuation envelope:** No implementation card is ready. The
  sole front-door task is the live-authorization decision. If authorization is
  declined or deferred, the consumer route-feature projection census is the
  clearest separate planning candidate, but promotion still needs operator
  discussion.
- **Lane budget / pause signal:** g05 is active with six numbered roadmaps,
  four completed milestones, two honest evidence stops, and zero ready
  milestones. Do not roll the generation merely because this phase paused.
- **Repository state before this handoff:** pushed `main` was `9a5b7229`; no
  pull requests were open. The handoff commit will move `main`, so the next
  thread should trust the tracked file in current `origin/main`.
- **Key files:**
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/batch-cards/011-watcher-acceptance-and-consumer-projection.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/logs/2026-08-30-g05-003-card-011-live-stop-review.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g05/batch-cards/019-watcher-proof-oracle-and-activity-delivery-repair.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/logs/2026-08-30-g05-006-card-019-watcher-proof-repair.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-30-consumer-route-feature-and-option-projection.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`

## Boundaries

Please keep the next pass within these boundaries:

- **In scope:** Own the operator conversation, settle whether a fresh live turn
  is wanted, prepare a new card and manual worker handoff if it is authorized,
  review any returned PR, and keep the g05 runway and logs honest. If the live
  attempt stays parked, discuss whether the route-feature census should be
  promoted into architecture/contracts and a roadmap.
- **Out of scope:** Do not run the live selector from the orchestrator thread,
  reuse the consumed card 011 authorization, rerun the old worker handoff,
  contact a provider without a new exact authorization, merge prototype head
  `49f2692f`, publish a watcher capability before the full oracle passes, or
  silently promote the consumer projection note. Do not widen watchers into
  containers, hostile-process containment, a generic event bus, arbitrary
  process authority, or a consumer UI facade.
- **Repo constraints:** Follow
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md` and the canonical contracts
  above. Use Effigy selectors for validation. Merge only after explicit
  operator authorization and an exact-head green review gate.

## Important Context

- **Planning lineage:** g05.001 promoted Contracts 058 and 059. g05.003 built
  the watcher core, host registry, ordinary host-process supervision, Contract
  060 bridge, and Claude binding. Card 011's single authorized live turn did
  not create a watcher. g05.006 repaired the proof and activity design without
  changing that evidence stop.
- **How the plan fits the system:** Contract 059 owns watcher identity,
  lifecycle, model/operator control, and completion policy. Contract 060 owns
  the operation-private closed bridge. Contract 044 owns consumer-visible
  activity disclosure. Claude-specific MCP, settings, skill, Stop hook, and
  version binding remain adapter-local.
- **Decisions and preferences:** The watcher feature supervises ordinary
  host-managed processes; Docker and hard descendant containment are not the
  product goal. Both model and operator need wait/inspect/stop controls. The
  agent turn should not complete until accepted watcher work is joined. Live
  provider turns are separately authorized. The operator expects the normal
  Northstar manual-handoff loop; do not substitute subagents for handoffs to
  their harness.
- **Open tensions:** The repaired oracle is capable of proving the intended
  sequence, but the model may still choose not to start a watcher. A new live
  authorization therefore needs an exact model, one-turn limit, frozen
  identity check, safe evidence policy, and explicit stop conditions. The
  route-feature projection census still needs decisions on snapshot identity,
  freshness, availability reasons, presentation ownership, and whether it
  amends Contracts 037/047/057 or gets one composing contract. Skill visibility
  remains stopped after Qoder returned an honest empty effective roster.
- **Validation posture:** PR 126 passed Stable, docs/API, pinned Rust `1.95.0`,
  dependency policy, and external source-consumer CI. Independent focused
  validation passed 585 tests. `effigy doctor` remains at the inherited 390
  god-file findings (341 warnings / 49 errors), plus a stale graph-index warning
  and one generated-source warning; this is known structural debt, not a new
  handoff blocker.

## Suggested Next Move

Start by reading `/Users/tom/.agents/skills/northstar/SKILL.md`, then
`/Users/tom/.agents/skills/northstar/references/router.md` and
`/Users/tom/.agents/skills/northstar/references/modes/orchestrator.md`. Fetch
`origin/main`, confirm the checkout is clean and contains this tracked handoff,
then read the roadmap front door and the two watcher logs named above.

The first operator question is simple: do they want to authorize exactly one
fresh provider turn for the repaired Claude watcher acceptance attempt? If yes,
clarify the exact model and compile a new bounded card, log, and worker handoff
before any provider contact. Give the operator only that handoff's absolute
path; let their harness run it. If no, keep g05.003 stopped and ask whether to
begin promotion discussion for the consumer route-feature projection census.

Take a moment to read the named canonical files before changing anything. If
one of the open questions changes the shape of the work, pause and bring that
question back rather than quietly choosing for the user.

## Completion Protocol

This handoff exists because another orchestrator thread genuinely needs to take
over. Before that thread finishes a meaningful batch, please:

1. Keep the single active `## Next Task` pointer only in
   `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/README.md`.
2. If a live turn is authorized, record the exact model, version/digest check,
   one-turn budget, evidence policy, failure stop, and no-rerun rule in a new
   card and log. Commit and push the planning base before issuing a fresh manual
   worker handoff under `/Users/tom/Dev/projects/swallowtail/docs/handoffs/`.
3. Treat the worker report as evidence, review any PR independently, post the
   verdict on GitHub, and merge only after separate operator authorization. A
   failed or inconclusive provider turn returns to an evidence stop; do not
   spend another turn automatically.
4. If live authorization is declined, leave card 011 and g05.003 stopped and
   give the route-feature projection note an explicit disposition before
   compiling implementation work. The census is evidence, not authority.
5. Keep g05 active; six roadmaps are well below the normal 30-50 generation
   range. Name the pause signal instead of manufacturing a ready card.
6. Reconcile the roadmap, generation index, batch-card index, log index, and
   any triage note touched by the next decision. Run the named docs and
   Northstar QA before pushing planning changes.
7. Call out unresolved risks plainly and leave one clear next task for the
   following thread.

If the same thread can continue after compaction, do not create another handoff
just for that reason.
