# 2026-08-27 g04.086 Cline ACP Plan Mode

Status: complete
Generation: g04
Cards: 242-243
Research: 240
Worker branch: `t3code/cline-acp-plan-mode`
Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-bc6d8f5a`

## Boundary

Bind one exact `cline.acp` `3.0.55` `HarnessMode::Plan` row through negotiated
ACP session configuration. Require exact advertisement and set-config
confirmation before readiness. Preserve omission, authority, replacement, and
joined cleanup.

## What Changed

- optional `ClineSessionProfileInput::with_harness_mode(Plan)` binds
  `HarnessModeSelection(Plan)`, plan requirements, evidence, and
  `SessionOptions`
- after `session/new`, Plan requires unique modes/`configOptions` plan
  membership, one `session/set_config_option` `{configId: mode, value: plan}`,
  and response `mode.currentValue = plan` before a usable handle
- omission keeps initialize/`session/new`/prompt frames with no mode request
- fresh context-losing replacement renegotiates the immutable Plan selection
- deterministic `plan_mode` coverage for positive, omission, rejection,
  permission non-widening, and replacement
- guide, route/feature matrices, example, changelog, Research 240, cards,
  milestone, and package API baseline updated route-locally

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy package:api
git diff --check
```

Inherited doctor baseline unchanged: `scan.god-files` 380 findings;
`scan.generated-in-src` one warning.

## Shared Closeout Deltas (orchestrator after merge)

- inventory item 41 moved to delivered; the ledger now has 53 closed items,
  22 active qualification candidates, no active delivery, and ten without an
  active lane
- programme, triage, roadmap/card/log indexes, and sole Next Task reconciled
- g04 remains open; Contract 029 currentness remains standing

## Review Repair

Addressed PR review on `7aa34afa`:

1. snapshot gate now requires `modes.currentModeId` and
   `configOptions[mode].currentValue` to agree, and both to be the frozen Act
   row before selection
2. `SessionRef` is validated before `set_session_id` / set-config; blank session
   ID emits zero set-config requests and joins cleanup
3. independent deterministic cases for missing/ambiguous/malformed modes vs
   config, current-truth contradiction, confirmation missing/ambiguous/
   malformed, drift, reject, disconnect-during-negotiation, and same-session
   second turn with no reselection
4. split fixture/tests under god-file thresholds; `effigy doctor` restored to
   inherited `380 / 334 / 46`

## Merge

PR 95 was reviewed, all five required hosted checks passed on exact head
`3f56aeb419296c1f539b09daa939f39fd70e10b3`, and the operator authorised the
merge. `main` fast-forwarded from `a88a612b` to that exact head. No merge commit
or rewritten head was created.

## Next

Compile the next bounded parallel qualification wave from the 22 active
inventory candidates. Keep g04 open.
