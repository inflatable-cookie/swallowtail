# 2026-08-27 g04.082b Gemini CLI Headless Thinking Evidence

Status: complete
Card: 229
Research: 230

## Boundary

Evidence only. The worker may update this file, card 229, Research 230, and new
Gemini-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Research 230 promoted with an empty deliver-now set on `gemini-cli.headless`
through exact `0.56.0`.

Thinking configuration is ambient and settings-backed:
`settings.modelConfigs` resolves into `generateContentConfig.thinkingConfig`
with built-in alias defaults (`includeThoughts`, `thinkingBudget: 8192` on
Gemini 2.5 chat aliases; `thinkingLevel: HIGH` on Gemini 3 chat aliases).
There is no qualified argv or env seam. `loadSettings()` always reads user and
workspace files even when `GEMINI_CLI_SYSTEM_SETTINGS_PATH` redirects the
system layer. Swallowtail's prepared route keeps ambient posture, passes no
thinking setting, and rejects portable `reasoning_mode` before process work.

Stream-json `init` exposes only resolved `model` and `session_id`. Assistant
messages exclude thought content. Reasoning output is not selected-value
confirmation.

Frozen evidence:
`crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-headless-0.56.0-thinking/thinking-evidence.json`.

Validation:
`effigy validate:focused swallowtail-adapter-gemini`, `effigy qa:northstar`, and
`git diff --check` passed on 2026-08-27.

## Validation

| Command | Result |
| --- | --- |
| `effigy validate:focused swallowtail-adapter-gemini` | pass |
| `effigy qa:northstar` | pass |
| `git diff --check` | pass |

## Unresolved

Production reasoning binding on headless stays blocked until Gemini CLI exposes
a headless-local confirmation transport or Swallowtail gains a fail-closed child
settings contract without ambient shadowing.
