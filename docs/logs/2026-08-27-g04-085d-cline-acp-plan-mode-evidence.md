# 2026-08-27 g04.085d Cline ACP Plan-Mode Evidence

Status: done
Card: 241
Research: 240

## Boundary

Evidence only. The worker may update this file, card 241, Research 240, and new
Cline-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Research 240 promotes one deliver-now row on exact `cline.acp` `3.0.55`:

- snapshot: `session/new` advertises `plan` / `act`, default `act`
- request: `session/set_config_option` `{ configId: "mode", value: "plan" }`
- confirmation: response `mode.currentValue == plan`
- application: first `ensureSessionManager` / `buildConfig` before prompt
- lifecycle: new-session only; post-start changes and load restore withheld
- contrast: root `--plan` discarded on ACP; not promoted from Research 220

Frozen evidence:
`crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.55/plan-mode-evidence.json`.

No production, shared index, matrix, or Next Task edits.

## Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy qa:northstar
git diff --check
```
