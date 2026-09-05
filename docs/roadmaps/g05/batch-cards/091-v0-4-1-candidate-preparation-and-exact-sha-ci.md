# 091 v0.4.1 Candidate Preparation And Exact-SHA CI

Status: ready; Card 090 accepted and merged; operator prepare authorization granted 2026-09-05 for exactly one Effigy prepare transaction
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../030-v0-4-1-release-readiness.md`
Depends on: accepted card 090 and Research 286; Contract 036; card 051 as precedent

## Goal

Prepare the frozen `0.4.1` source candidate through one authorized Effigy
prepare transaction, rerun all local gates on the frozen tree, land the
accepted candidate on canonical `main`, and require CI at that exact SHA.

## Scope

1. Write the `docs/releases/0.4.1.md` note and index entry from card 090's
   ledgers: carrier content, Contract 061 coverage, OpenCode qualification,
   known limits, install, upgrade, and rollback text. No version mutation
   yet.
2. Run read-only release status; confirm it infers patch `0.4.1` and the
   three-mutation prepare plan (coordinated `Cargo.toml` versions, changelog
   promotion, workspace-only `Cargo.lock` sync).
3. The operator granted prepare authorization on 2026-09-05, contingent on
   card 090's acceptance and a clean canonical base; a failed or rolled-back
   prepare consumes it and needs a fresh grant. Run exactly one prepare
   transaction; rerun all local gates on the frozen tree; extract the exact
   promoted changelog; regenerate the `0.4.1` semantic baseline and route
   inventory without touching `0.4.0` files.
4. Open the candidate PR; stop for exact-head review; on merge, require
   workflow-dispatch CI at the merged SHA.

## Out Of Scope

Tag creation or push; publication; consumer edits; any feature change.

## Auto-Continuation

No. Stop for exact-head review and exact-SHA CI.
