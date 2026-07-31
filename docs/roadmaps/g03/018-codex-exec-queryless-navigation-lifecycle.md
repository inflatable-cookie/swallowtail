# 018 Codex Exec Queryless Navigation Lifecycle

Status: completed
Owner: Tom
Created: 2026-08-01
Depends on: g03.017
Vision tags: consumer stability, observable activity, exact lifecycle
Contract refs: 009, 029, 044
Planning state: cards 047-048 completed

## Problem

Soundcheck reproduced Codex CLI `0.146.0` completing a valid `web_search`
item with an empty query and `action.type == "other"`. Codex then emitted a
valid schema-conforming final `agent_message` and `turn.completed`, but
Swallowtail rejected the lifecycle-only item during activity projection and
aborted the whole structured run as malformed.

Contract 044 already requires exact activity identity, no invented display
content, and fail-closed malformed lifecycle. The adapter's query rule is
narrower than the observed qualified event.

## Goals

- [x] freeze the exact queryless navigation lifecycle
- [x] accept only completed queryless `action.type == "other"` observations
- [x] preserve ordinary query-bearing search content
- [x] retain one activity identity across start and completion
- [x] prove malformed completed search actions still fail closed
- [x] prove final structured output and terminal completion survive the event

## Execution Plan

### Batch 18.1 — Exact Activity Fixture And Projection

- [x] Execute card 047.
- [x] add the exact `0.146.0` lifecycle to the exec activity corpus
- [x] narrow the content rule to the observed lifecycle-only action
- [x] assert identity, phases, and absent content
- [x] retain negative coverage for missing actual-search queries

### Batch 18.2 — Whole-Stream Regression And Closeout

- [x] Execute card 048.
- [x] feed the frozen lifecycle through `ExecEventParser`
- [x] prove the later structured proposal and `turn.completed` remain intact
- [x] run focused Codex and affected-package validation
- [x] record the accepted shape and Soundcheck retest handoff

## Boundaries

- no Soundcheck edit or model/reasoning change
- no broad acceptance of queryless completed searches
- no invented query, label, summary, or display content
- no structured-output schema or final-output validation change
- no diagnostic-code change
- no version-range, search-authority, provider, or transport change
- no live provider, authentication, installation, publication, or broad suite

## Acceptance Criteria

- [x] a started queryless navigation observation remains valid and content-free
- [x] a completed queryless `action.type == "other"` observation is valid and
  content-free
- [x] both observations retain one provider and runtime activity identity
- [x] query-bearing search lifecycle retains its exact display query
- [x] completed actual-search actions without a query remain malformed
- [x] the later valid structured proposal becomes final output
- [x] focused, package, docs, Northstar, format, and diff checks pass
- [x] Soundcheck's ignored live review remains the sole next integration task

## Next Planning Checkpoint

After card 048, return Swallowtail to the g03 evidence gate. Soundcheck owns
the authenticated Luna/medium rerun.
