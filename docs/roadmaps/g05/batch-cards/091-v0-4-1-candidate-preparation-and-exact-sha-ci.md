# 091 v0.4.1 Candidate Preparation And Exact-SHA CI

Status: ready; Card 093 merged as 4bb00dab; re-prepare on this repaired base under renewed authorization
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
3. Authorization history: the first prepare on 2026-09-05 passed nine gates
   and failed `floor`; Effigy rolled back cleanly and retained no gate
   output. Chatterbox reproduced the floor gate on the identical tree and it
   passed (pinned `1.95.0` clippy clean, 271 test binaries green), and CI's
   pinned-MSRV job passed on card 090's head. The operator then said "do
   whatever we need to do to get this done": authorization is renewed, and
   Chatterbox may renew it again without a further operator round-trip
   whenever a failure is captured and shown to be transient. Each attempt
   still runs exactly one prepare transaction, and each attempt must tee
   every gate's stdout and stderr to `.effigy/reports/release/<gate>.log` in
   the workspace (or an equivalent captured log) so a failure names the test.
   A failure that names a real defect, not a race, stops the lane and returns
   to Chatterbox. Run one prepare transaction; rerun all local gates on the frozen tree; extract the exact
   promoted changelog; regenerate the `0.4.1` semantic baseline and route
   inventory without touching `0.4.0` files.
4. Open the candidate PR; stop for exact-head review; on merge, require
   workflow-dispatch CI at the merged SHA.

## Out Of Scope

Tag creation or push; publication; consumer edits; any feature change.

## Auto-Continuation

No. Stop for exact-head review and exact-SHA CI.
