# 2026-08-24 g04.057 Grok Build ACP Reasoning Selection Closeout

Status: complete
Owner: Tom
Milestone: g04.057
Cards: 158 complete; 159-160 blocked

## Result

Research 204 is an honest empty deliver-now set. Exact Grok Build ACP
advertises model-catalog `reasoning_efforts` and has no Contract 034 selection
channel. Cards 159-160 stay blocked. No production code. No public API change.
g04 stays open.

## Evidence Table

| Version | Model | Values | Advertised | Selectable ACP option | Confirmed | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- |
| `0.2.114..=0.2.117` | `grok-4.5` | `low`, `medium`, `high` | yes | no | no | no |
| `1.0.4..=1.0.5` | `grok-4.6` | `low`, `medium`, `high`, `xhigh` | yes | no | no | no |
| any qualified | any | `off`, `minimal`, `max`, aliases | CLI only | no | no | no |

Exact `0.2.114`, `1.0.4`, and `1.0.5` binaries contain no
`session/set_config_option`. Official ACP `session/new` is `cwd` plus empty
`mcpServers`. Current spawn stays `grok --no-auto-update agent stdio` with empty
`SessionOptions`.

Omission retains current wire. Attachment recovery stays on the empty-options
path. Load/resume stay unqualified. `UnverifiedNewer` has no private mapping to
inherit. No behavior, driver, claim, or configured-instance revision.

CLI `--effort`, `GROK_CONFIG` defaults, and later-source `_meta.reasoningEffort`
are not ACP confirmation. The last of those also substitutes defaults and
ignores unsupported values.

## Application State

Unchanged. Interactive sessions reject non-empty `SessionOptions`. Structured
runs expose no reasoning input. `session/new` does not wait on a config
snapshot. Failure after initialize/authenticate/`session/new` still joins owned
work and preserves provider-owned durable-session truth.

## Validation

Card 158 gates passed: `git diff --check`, `effigy validate:focused
swallowtail-adapter-grok` (30 tests), `effigy qa:northstar`, `effigy
qa:docs:index:research`. Doctor was not re-run; inherited baseline remains 378
findings (332 warnings / 46 errors) plus one generated-in-src warning. Default
validation used no install, login, account inspection, provider prompt,
credential capture, external inference request, or paid work.

## PR

- URL: https://github.com/inflatable-cookie/swallowtail/pull/56
- base: `main`
- head: `t3code/review-acp-reasoning-selection`
- reviewed worker head: `47e1d8fee35d072082e0d3501922a864d7c1133e`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-70fd631c`
- merge: none; do not merge from this worker

## Shared Closeout

Orchestrator-owned after merge. Do not apply on this worker branch:

- `docs/research/README.md`: 204 reserved → promoted evidence stop; empty set
- `docs/logs/README.md`: this closeout reserved → complete
- `docs/roadmaps/README.md` Next Task: leave g04.057 until merge, then
  reassess remaining per-route inventory
- `docs/roadmaps/g04/README.md` and generation index: g04.057 planned → stopped
- architecture, Contract 029, route/feature matrix `reasoning_selection`: keep
  No; no claim edit
- programme, changelog: no feature delivery
- g04 remains open; no rollover

## Next

After review and merge, the orchestrator reconciles g04.057 and reassesses the
remaining per-route inventory. g04 stays open until explicit operator
direction.
