---
title: g05.029 Card 089 bounded-profile tool exclusion worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260904-211500-g05-card089-core-preflight-tool-exclusion.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g05, contract-013, preflight]
---

# g05.029 Card 089 Core Preflight Tool Exclusion

## Objective

Narrow the Contract 013 preflight rejection to bounded profiles: preserve the
bounded read-write plus `ToolCalls` failure while admitting ambient read-write
plus `ToolCalls` for mediated provider tools.

## Scope

Own only `crates/swallowtail-core/src/preflight/**`, the manifest-named
`swallowtail-testkit` assertions and any explicitly named additive API
baseline, Card 089 result, and allowed closeout/documentation surfaces.
Adapters, runtime, contracts, and unrelated packages are forbidden.

## Execution

Fetch origin before preflight. Preserve the existing bounded-profile message
and behavior; make ambient read-write with `ToolCalls` pass; prove every
existing adapter plan still preflights unchanged. Follow the Card 089 oracle
and Contract 013 clarification exactly, without changing Card 080 in this
lane.

## Validation and review

Run the manifest-named focused, affected-package, API, docs, Northstar,
god-file, formatting, and diff gates. Push one reviewable PR against `main`
and request an independent cross-model exact-head review in this same worker
workspace. Do not merge.

## Handoff

Report the exact PR head, owned-path diff, baseline additivity, validation,
and any deviation from the card. Coordinator owns the merge gate, reserved
closeout, and Card 080 sequencing.
