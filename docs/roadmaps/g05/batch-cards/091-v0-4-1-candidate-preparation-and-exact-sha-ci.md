# 091 v0.4.1 Candidate Preparation And Exact-SHA CI

Status: complete; `v0.4.1` tagged at `c3cce750`
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

## Result

Prepared the `0.4.1` source candidate on rebased base
`201996d9a67926ae24de8b61dc4651aef3591118`. The worker head before Effigy
preparation was `f9a7714095de477f2420fa9375b46939a23036d9`, preserving the
release note and index. The required environment and toolchain evidence is in
`.effigy/reports/release/environment-attempt-6.log`; it records pinned Cargo
`1.95.0` and stable Cargo `1.97.1`. Read-only status was ready for patch
`0.4.1`, the plan contained exactly three mutations, locked metadata reported
40 workspace packages, and Cargo.lock was synchronized before preparation.

Exactly one JSON-captured prepare transaction ran. Its report is
`.effigy/reports/release/prepare-attempt-6.json` with SHA-256
`1f8b93b3361a3e87431a3b6c698167a0ff4f62134b1d18bdb7d93f8077846092`; all 11
prepare gates passed. The frozen-tree set also passed all 11 gates; its
complete per-gate outputs and summary are under
`.effigy/reports/release/frozen-gates-0.4.1-attempt-6-*.log`, with
`frozen-gates-0.4.1-attempt-6-summary.log` recording the green result. The
prepared floor was green, so no compression-rule floor rerun was needed.

The promoted changelog was extracted to
`.effigy/reports/release/changelog-0.4.1.txt` (SHA-256
`fc4aa3a696386c69f50e508efc90a3e590033d557dd0abe3b99458cf22b838d0`). The
distinct `0.4.1` evidence contains 40 public API package files plus its
manifest, 49 production routes (SHA-256
`ee184c09c57409ed8dcac7789d1c87cf9b97b8015f5892be81febafff99b82a2`), and 88
internal dependency edges, all requiring `^0.4.1` (SHA-256
`6d35bf0f02e13f572d36f4a3b63e992616a04e7e5c44552d164a26634fc860be`). The API
manifest SHA-256 is
`42b64192145c35aa1523167ec6c99d8b0d09cc66a05b7561aeb08fd04a5531eb`.
No `0.4.0` baseline was modified. Candidate PR `#229` was accepted at exact
head `cb732009` and merged at `c3cce750`. The merged-SHA workflow-dispatch run
`33969131592` passed all six jobs. The source consumer passed from a clean
detached checkout at the merged SHA. Operator authorization created and pushed
annotated tag `v0.4.1`; no application had driven the candidate before the
tag. No publication or consumer mutation occurred.
