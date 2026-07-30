# 044 Installed Harness Range Maintenance

Status: completed
Owner: Tom
Created: 2026-07-30
Depends on: g02.043
Vision tags: compatibility windows, installed harnesses, consumer longevity
Contract refs: 029, 032, 036-040, 044
Planning state: cards 146-149 completed

## Problem

Codex `0.146.0` and OpenCode `1.18.5..=1.18.10` are stable releases above
Swallowtail's guaranteed upper bounds. They can run as visible unverified
newer points, but current exact evidence now supports a batched qualification
tranche.

Routine upstream movement must remain cheap. It must not force immediate
Swallowtail publication, erase older support, or turn one provider patch into
a common API change.

## Generation Runway Goal

Prove the normal Contract 029 maintenance loop across two materially different
installed transports while retaining baseline support and visible
unverified-newer execution.

## Goals

- [x] select only ranges with current exact evidence
- [x] qualify Codex exec and app-server through `0.146.0`
- [x] qualify OpenCode HTTP/SSE through `1.18.10`
- [x] retain every existing baseline, milestone, exclusion, and lifecycle rule
- [x] keep later stable releases visible unverified newer
- [x] close through focused package and public-truth evidence

## Non-Goals

- automatic network currentness checks in default QA
- automatic installation or updates
- Swallowtail or crates.io publication
- moving any supported baseline
- qualifying prereleases or syntactically possible unpublished points
- Grok alpha qualification
- provider, model, endpoint, credential, or route fallback
- consumer repository edits

## Execution Plan

### Batch 44.1 — Currentness And Corpus Selection

- [x] Execute card 146.
- [x] Promote Research 071.
- [x] freeze exact package, tag, source, and selected-surface evidence
- [x] confirm existing contracts cover both extensions

### Batch 44.2 — Codex `0.146.0`

- [x] Execute card 147.
- [x] extend exec, app-server, lifecycle, continuity, activity, discovery, and
  prepared claim evidence
- [x] retain exact later-stable and prerelease classification

### Batch 44.3 — OpenCode `1.18.10`

- [x] Execute card 148.
- [x] extend HTTP/SSE, lifecycle, deletion, continuity, callback, usage,
  generation-control, activity, discovery, and prepared evidence
- [x] preserve the unrelated `1.18.8` full-artifact delta without inventing a
  selected-surface milestone

### Batch 44.4 — Acceptance And Closeout

- [x] Execute card 149.
- [x] run focused cross-host and extracted-package proof
- [x] refresh route, release-note, roadmap, and front-door truth

## Acceptance Criteria

- [x] Codex `0.146.0` is qualified for both transports
- [x] OpenCode `1.18.5..=1.18.10` is qualified
- [x] older guaranteed versions remain unchanged
- [x] exact later stable points remain unverified newer, not hard-denied
- [x] no selected access, lifecycle, cleanup, activity, or operation claim is
  widened without exact evidence
- [x] no broad workspace suite is required before closeout
- [x] affected package archives assemble and compile
- [x] one clear next task remains

## Decision Gates

- Stop if exact source changes a selected lifecycle or authority boundary.
- Stop if a range point needs prompt replay, fallback, hidden cleanup, or a new
  public operation.
- Stop before implementation if a new shared contract is required.
- Do not run a live provider prompt or mutate an attached OpenCode server.

## Next Planning Checkpoint

Compatibility maintenance pauses until another installed stable point
accumulates. Reassess the next g02 stabilization lane with the operator.
