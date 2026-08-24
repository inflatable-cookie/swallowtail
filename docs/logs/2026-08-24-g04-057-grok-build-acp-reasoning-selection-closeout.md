# 2026-08-24 g04.057 Grok Build ACP Reasoning Selection Closeout

Status: complete
Owner: Tom
Milestone: g04.057
Cards: 158 complete; 159-160 blocked

## Result

Research 204 is an honest empty deliver-now set. Exact 1.0.4 and 1.0.5 insert
`x.ai/sessionConfig.options` but do not freeze effort/selected membership.
1.0.5's open-time hint fail-opens. Cards 159-160 stay blocked. No production
code. No public API change. g04 stays open.

## Evidence Table

| Version | Model | Values | Advertised | Open-time hint | Negotiated ACP option | Confirmed | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0.2.114..=0.2.117` | `grok-4.5` | `low`, `medium`, `high` | yes | no | no | no | no |
| `1.0.4` | `grok-4.6` | `low`, `medium`, `high`, `xhigh` | yes | no new-session apply path | no | no | no |
| `1.0.5` | `grok-4.6` | `low`, `medium`, `high`, `xhigh` | yes | dispatchable; ignore if unsupported | no | no | no |
| any qualified | any | `off`/`none`, `minimal`, `max`, aliases | CLI / later enum | ignore on parse failure | no | no | no |

Exact binaries contain no `session/set_config_option`. Both 1.0.4 and 1.0.5
insert `_meta["x.ai/sessionConfig"].options` (`1.0.4` offset `107073191`,
`1.0.5` offset `107433096`). Exact option membership and selected-effort
truth are not in those binaries or in preserved handshakes. Official ACP
`session/new` is `cwd` plus empty `mcpServers`. Current spawn stays
`grok --no-auto-update agent stdio` with empty `SessionOptions`.

The empty set is unfrozen `sessionConfig` effort/selected truth plus 1.0.5
fail-open, not a missing response channel and not later-source-only.

Omission retains current wire. Attachment recovery stays on the empty-options
path. Load/resume stay unqualified. `UnverifiedNewer` has no private mapping to
inherit. No behavior, driver, claim, or configured-instance revision.

## Application State

Unchanged. Interactive sessions reject non-empty `SessionOptions`. Structured
runs expose no reasoning input. `session/new` does not wait on a config
snapshot. Failure after initialize/authenticate/`session/new` still joins owned
work and preserves provider-owned durable-session truth.

## Validation

Card 158 gates passed on this correction: `git diff --check`, `effigy
validate:focused swallowtail-adapter-grok` (30 tests), `effigy qa:northstar`,
`effigy qa:docs:index:research`. Default validation used no install, login,
account inspection, provider prompt, credential capture, external inference
request, or paid work. Doctor was not re-run; inherited baseline remains 378
findings (332 warnings / 46 errors) plus one generated-in-src warning.

## PR

- URL: https://github.com/inflatable-cookie/swallowtail/pull/56
- base: `main`
- head: `t3code/review-acp-reasoning-selection`
- evidence commit: `47e1d8fee35d072082e0d3501922a864d7c1133e`
- previously reviewed heads: `56dd8dd24834f9bf1f9c23c07e793bb42fffa126`,
  `3dff924831c4b9a03e3d67fa5873bc6d0b7decb4`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-70fd631c`
- merge: none; do not merge from this worker

## Shared Closeout

Orchestrator-owned after merge. Do not apply on this worker branch:

- `docs/research/README.md`: 204 reserved → promoted evidence stop; empty set;
  split 1.0.4/1.0.5
- `docs/logs/README.md`: this closeout reserved → complete
- `docs/roadmaps/README.md` Next Task: leave g04.057 until merge, then
  reassess remaining per-route inventory
- `docs/roadmaps/g04/README.md` and generation index: g04.057 planned → stopped
- architecture, Contract 029, route/feature matrix `reasoning_selection`: keep
  No; no claim edit
- programme, changelog: no feature delivery
- `docs/triage/2026-08-21-advanced-route-features.md` `grok-build.acp` block:
  drop the "Changelog: ACP clients can specify reasoning effort when opening
  or resuming (Grok Build 1.0.x)" lead. Frozen changelog pages do not contain
  that sentence. Official ACP docs do not document an effort field. Exact
  1.0.5 has an unconfirmed open-time `_meta.reasoningEffort` hint; 1.0.4 does
  not apply it on `session/new`. Matrix Effort row should not treat "ACP open
  effort" as official 1.0.4+ protocol truth.
- g04 remains open; no rollover

## Next

After review and merge, the orchestrator reconciles g04.057, applies the
shared-triage correction, and reassesses the remaining per-route inventory.
g04 stays open until explicit operator direction.
