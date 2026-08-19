# 147 Cline Headless 3.0.55 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 304

## Question

Is official Cline CLI `3.0.55` `--json` a distinct bounded print wire that
can freeze one JSON run without flattening onto `--acp`, TUI, hub, `--id`
resume, the docs `ask`/`say` schema, or `--auto-approve true`?

## Method

Reconciled Research 144/145 and Research 146 with official docs, npm
`cline@3.0.55` identity already frozen for ACP, GitHub annotated tag
`cli-v3.0.55`, and the tagged CLI sources that emit `--json` output.

Did not install Cline. Did not download a platform binary. Did not log in
or send a `--json` prompt. Host PATH has no `cline`.

Observed versions are not qualified claims. No headless compatibility
claim in this record.

## Identity

Same executable and axis as `cline.acp`. Distinct selected flags.

| Surface | Value |
| --- | --- |
| Route | `cline.headless` |
| Axis | `cline.package` |
| Package | existing `swallowtail-adapter-cline` |
| npm | `cline@3.0.55`, published 2026-08-14T07:55:21.353Z, `latest` on 2026-08-18 |
| Wrapper tarball SHA-256 | `7eec2ad80d8dfa27b9baaa22c7340ebe861850f6057b9e2e80a5dd9d2ef2f5ef` |
| GitHub tag | `cli-v3.0.55` annotated `c238103e631d492b97bf9e63b060390f1bb8a8a6` |
| GitHub commit | `ad442cbb6a81d21773ceabc1398ea5eb58170718` |
| Host | absent |

`cline` on npm is a Node wrapper that resolves an optional platform
package. The headless child is that compiled binary with `--json`.
Swallowtail binds the host-approved `cline` executable; it does not wrap
Node, Bun, or `@cline/core`.

## Selected wire

Entrypoint: `cline --json --auto-approve false` plus one prompt argv
operand. Optional `-c/--cwd` is the working resource. Piped stdin is
documented as an alternate prompt channel and stays unselected.

`--json` forces plain-text/headless mode and requires a prompt or piped
stdin; interactive TUI is unsupported. `--acp` is mutually exclusive in
`apps/cli/src/main.ts`: ACP returns before the print path. `cline --json
--acp` is still ACP, not this route.

Stdout is one JSON object per line from `emitJsonLine`: ISO-8601 `ts`
plus a `type` envelope. Tagged encoder types:

- `run_start`
- `agent_event` wrapping nested AgentEvent (`content_start` /
  `content_end` / `done` / `error` / `notice` / iteration markers)
- `run_result` (`finishReason`, `text`, usage, duration)
- `run_abort_requested` / `run_aborted`

JSON-mode errors on stderr are `{type:"error", message}` envelopes.
`team_event` / `team_restored` are emitted and stay unmapped.

docs.cline.bot still documents `ask`/`say` NDJSON with millisecond `ts`.
That matches task `ui_messages.json`, not the tagged CLI encoder. Treat
docs `ask`/`say` as the wrong wire.

First useful op:

1. spawn `cline --json --auto-approve false` with cwd and one prompt
2. drain stdout envelopes
3. map `run_result.finishReason` `completed` → `end_turn`, `aborted` →
   `cancelled`; other finish reasons and timeout exit 1
4. join or kill the child

## Authority

CLI `--auto-approve` help-default is `true`. Headless startup uses
`defaultToolAutoApprove = true` and
`resolveStartupToolAutoApprove(flag → persisted → default)`. Omitting
the flag inherits true. Swallowtail must pass `--auto-approve false`.
`--yolo` is not a substitute.

Auth is host-owned `CLINE_API_KEY` or persisted settings. JSON/headless
mode skips browser OAuth. Swallowtail does not log in or pass `-k/--key`.

Working resource is `-c/--cwd`. Isolation is one owned stdio child.
Cleanup is SIGINT/SIGTERM (child emits `run_abort_requested`), then join
or kill. Host process deadline is required; CLI `--timeout` stays
optional until card 305 selects it.

`--id` resume, TUI, hub/zen, kanban, schedule, teams, worktree, and
plan/act as a Swallowtail harness mode stay unmapped.

## Decision

Admit `cline.headless` as a first-party print route on the existing
Cline package. Freeze identity and named fixtures under
`crates/swallowtail-adapter-cline/tests/fixtures/cline-headless-3.0.55/`.
Card 305 may add the decoder and claim. No production matrix row in this
card.

## Non-goals

- installing Cline or extracting the platform binary
- live `--json` prompt, catalogue, or OAuth
- ACP execution, version-range claims, README crate-count repair
