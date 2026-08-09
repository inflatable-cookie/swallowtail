# 118 Command Code Session Continuity And Export

Status: promoted
Owner: Tom
Date: 2026-08-09

## Question

After g03.059, which Command Code session surfaces have a machine boundary for
Swallowtail: exact-id resume/continue, session catalogue, and session export?

Provider API stays out of scope for this note.

## Method

Exact npm `command-code@1.15.1` on the qualification host. Isolated temporary
workspaces. Plan-mode `-p --output-format json` with
`--skip-onboarding --no-auto-update --trust --no-skills`, model
`deepseek/deepseek-v4-flash`. Redacted captures only; account names, prompts,
thinking, tool bodies, and private paths are not copied here.

## Resume And Continue

| Invocation | Result |
| --- | --- |
| First retained turn (omit `--no-session`) | Exit `0`; `result.sessionId` is a UUID; project transcript appears under `~/.commandcode/projects/<cwd-component>/` |
| Same cwd + `--resume <exact-uuid>` | Exit `0`; same `sessionId`; second turn completes |
| Same cwd + `--session <exact-uuid-prefix>` | Exit `0`; same session |
| Same cwd + `--session <absolute.jsonl path>` | Exit `0`; same session |
| Different cwd + `--resume <uuid>` | Preflight error on stderr; `result.subtype=error`; no reusable attach |
| `--continue` | Works as ambient latest; **forbidden** for Swallowtail public or private attach (Contract 043) |
| `--resume` + `--fork-session` | Succeeds with a **new** session id; **forbidden** as continuation |
| Missing/unknown id | `Error: No session "…" found to resume.` / `--session` path-or-prefix error; `result.subtype=error` |

`--no-session` still emits a `sessionId` on the result line and may write
`.meta.json` / `.checkpoints.jsonl` under the project component, but does not
create the durable `.jsonl` transcript required for later `--resume`. Structured
runs that must not retain transcripts keep `--no-session`.

Working-resource cwd is part of session identity. Exact-id resume only works
when the approved working directory matches the project that owns the
transcript.

## Catalogue And Export

| Surface | Machine boundary |
| --- | --- |
| List / browse sessions | No non-interactive CLI. `/sessions` and `/resume` require a TUI |
| Import | No 046 import API. Consumer-supplied `--session <path\|id>` is an attach selector, not catalogue→import |
| Export | `/export` and `command-code export` require a TTY; no `--export` flag |
| On-disk layout | `~/.commandcode/projects/<sanitized-cwd>/<sessionId>.jsonl` plus `.meta.json` and `.checkpoints.jsonl` |

Filesystem enumeration of `~/.commandcode/projects` is **not** a Contract 046
catalogue. Meta files carry private `traceIds`; transcript JSONL is private
conversation state. Swallowtail must not scan or ingest those trees into
portable diagnostics or fixtures.

## Route Decision

| Surface | Decision |
| --- | --- |
| Exact-id interactive continuity | Promote under Contract 043 on `command-code.headless` / same package |
| Ambient `--continue` / `--fork-session` | Reject in prepared evidence |
| Public 017 load/resume binding | Not earned; private continuity only |
| 046 catalogue / import / provider export | Unsupported on `1.15.1`; keep matrix `No` / `Not applicable` with this evidence |
| Provider API | Out of scope (operator deferral) |

Recommended interactive shape (same family as Qwen/Antigravity):

- first turn: retained session (no `--no-session`), no resume selector
- later turns: exact private `--resume <sessionId>` observed from the prior
  clean turn
- structured-run path unchanged: `--no-session`,
  `ProviderRetentionPolicy::Prohibited`

## Contract Fit

- Contract 043 covers the proven exact-id private continuation.
- Contracts 017 and 046 do not gain Command Code catalogue, import, export, or
  public resume from this evidence.
- No new provider-neutral contract is required.

## Recommendation

**Promote interactive exact-id continuity** as the next Command Code tranche.
Record catalogue/export as evidence-backed absences until Command Code exposes
a non-TTY list or export surface. Do not invent a home-directory catalogue.
