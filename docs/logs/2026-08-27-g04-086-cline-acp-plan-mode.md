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

- inventory item 41 → delivered; active-delivery count
- programme, triage, roadmap/card/log indexes, sole Next Task
- keep g04 open; Contract 029 currentness remains standing

## Next

Open one reviewable worker PR against current pushed `main`. Do not merge.
