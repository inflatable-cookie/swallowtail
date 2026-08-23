# 198 Qwen Headless Turn And Tool Budget Evidence

Status: complete; deliver-now subset admitted
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.051 / 142-144

## Question

Can exact production route `qwen.headless` on Qwen Code `0.21.15` expose
typed caller-decreasing session-turn and tool-call budgets across structured
runs and every turn child while preserving exact terminal and lifecycle truth?

## Decision

Yes, for exact package `0.21.15` only. Admit caller-decreasing
`--max-session-turns` values `1..=24` and `--max-tool-calls` values `0..=16`
as adapter-local typed input. Independent omission of either flag keeps the
current argv byte `24` or `16`. Both omitted keeps current
`--max-wall-time 60s --max-tool-calls 16 --max-session-turns 24`.

These are per-child process-local Qwen counters. They do not cap
Swallowtail's separate interactive session bound of 24 host turns. They are
not Contract 040 portable generation controls and do not prove that the
provider completed less work.

No shared `Capability`, no OperationPolicy/SessionOptions field, and no
Contract 029 currentness-range change is required to bind the subset. Gate
selected values on exact package `0.21.15`. Other qualified package points
keep omission-only current argv. Name
`qwen-code.headless.v0.21.15-turn-tool-budgets` only as a post-merge
orchestrator delta if the claim should record the exact-version feature gate;
the worker must not edit Contract 029.

## Frozen Official Evidence

The current official headless page was fetched without credentials on
2026-08-23 at 20:56:45 GMT.

| Surface | URL | Last-Modified | ETag | Complete-body SHA-256 |
| --- | --- | --- | --- | --- |
| Qwen Headless Mode | <https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/> | `Mon, 17 Aug 2026 22:22:30 GMT` | `"6a8389a6-9bbb9"` | `3710cc904831e4591acc34395dd85e298ddd4013913ea57e7384e88a7b7ea2fb` |

The HTML body reports `Last updated` `2026-08-12T21:19:39.000Z`. The matching
exact-tag markdown `docs/users/features/headless.md` SHA-256 is
`6686955703a0de42ee721cb2e4777e8deaa3add56f1f1a3247475fd61bb1faaa`.

Official text:

- `--max-session-turns` caps user/model/tool turns; overrun is exit **53**
- `--max-tool-calls` is a cumulative top-level tool-call budget; overrun is
  exit **55** (`FatalBudgetExceededError`), shared with wall-time overrun
- default for each budget is `-1` (unlimited) when unset; Swallowtail already
  pins `24` / `16` / `60s` and must keep those bytes on omission
- `--max-tool-calls` counts top-level dispatches only; `agent` inner calls
  are not counted; `structured_output` under `--json-schema` is tool-budget
  exempt and turn-budget not exempt
- stream-json input resets budget counters at the start of every user
  message (per-message, not per-process)
- `qwen serve` / ACP does not consult these flags; not this route

Subagent inner-call accounting and JSON-schema `structured_output` exemptions
are not applicable: the current route excludes `agent` and selects no JSON
schema. Record them as withheld/not-applicable. Do not widen the route to
exercise them.

## Frozen Exact Package Evidence

Inspected from the disposable official npm tarball and matching official
GitHub source at tag `v0.21.15`. No package was installed. No live Qwen
process, account, credential, catalogue, or prompt was used.

- Package: `@qwen-code/qwen-code@0.21.15`
- npm tarball SHA-256: `8d405b065888b7000a6989d99c2d79257cd8f9f5b68e9078fb76484527351b9a`
  (re-fetched 2026-08-23; matches Research 173 / fixture `identity.json`)
- GitHub source commit: `5dce2515a778f9cf2013168962b4fbc3454636e3`
- Existing route fixture: `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.15/`

| Path | SHA-256 |
| --- | --- |
| `packages/cli/src/config/config.ts` | `a195b3a8782eab208559620c0c24649a6e685c48559aafdcb725738fb3f27042` |
| `packages/cli/src/utils/runBudget.ts` | `e7a4a0c98583fe417f02430f081d1408f875711dabe51b3dabe61d81882dd592` |
| `packages/cli/src/utils/runBudget.test.ts` | `e82b35a29e0fdeff55d003c8ed2d17e7f801b85d03cb31f842141d0af493b885` |
| `packages/cli/src/utils/errors.ts` | `5650e05413ef8d64089b0a0cf14dbabde076fcbb99dc2546d22f5b5f3bcaddce` |
| `packages/cli/src/nonInteractiveCli.ts` | `29505afe63601deeca70ad29ea2c1f6b310cc14c835d0bbf60b979ccad67b3a6` |
| `packages/cli/src/nonInteractive/session.ts` | `25991c35f9b5b3aae1d14b4678411be363332d687e1f602fa12eb3a45c17b7f1` |
| `packages/cli/src/nonInteractive/types.ts` | `6ad4da59e11d6a84d2b9bd03376b834d1737edfc7a8dc9af1e90783ccab2acdf` |
| `packages/cli/src/nonInteractive/io/BaseJsonOutputAdapter.ts` | `d555f489d55ac33fd3ae3dc05e12242b77105e5d962168f4fa49e802498777a5` |
| `packages/core/src/config/config.ts` | `d68623bfa6a032edf8f0d516e3fa71f5c3016ada0e641e5d6027c2371a8104d1` |
| `packages/core/src/utils/errors.ts` | `ec86ce1b15a12703fb86c7a3110f3118bddb4a9ebb87c3450f42f8239b106393` |

## Parser Domains Versus Swallowtail Domains

### `--max-session-turns`

Exact `config.ts` yargs option is `type: 'number'`. Config stores
`argv.maxSessionTurns ?? settings.model?.maxSessionTurns ?? -1`.
`validateMaxSessionTurns` requires an integer and otherwise throws
`FatalConfigError`; it accepts `0`, `-1`, and other negatives.

Enforcement in `runNonInteractive`: local `limitedTurnCount` starts at `0`,
increments at the start of every non-runtime-Goal loop iteration (user query,
tool-result continuation, teammate, drain). When
`maxSessionTurns >= 0 && limitedTurnCount > maxSessionTurns`, the process
calls `handleMaxTurnsExceededError` and exits 53. First allowed iteration is
count `1`. Value `0` therefore aborts before `sendMessageStream`. Value `-1`
and other negatives skip the `>= 0` gate (unlimited).

| Input class | Upstream | Swallowtail |
| --- | --- | --- |
| omission | unlimited `-1` unless settings supply a value | retain current `24` |
| `1..=24` | integer accepted; N loop iterations per `runNonInteractive` | deliver-now |
| `0` | aborts before the first model call | withheld; not useful for a one-prompt child |
| `25..` / raised | accepted by parser | withheld; caller-increasing |
| `-1` | unlimited | withheld |
| other negatives | integer accepted; treated as unlimited | withheld/invalid |
| fractional / NaN / non-integer | `FatalConfigError` | invalid; never a public constructor |
| aliases | none | invalid |

### `--max-tool-calls`

`validateMaxToolCalls` accepts `-1` (unlimited), `0` (no tools; first tick
aborts), and integers `1..=1_000_000`. It rejects NaN, Infinity, other
negatives, fractions, and values above `1_000_000`. Flag wins over
`model.maxToolCalls`; otherwise unlimited.

`RunBudgetEnforcer.tickToolCall` increments then compares. Tick happens
**before** `executeToolCall`. Budget `N` allows N executions; the `(N+1)`th
tick aborts without launching that tool. Budget `0` aborts on the first
tick; the model may still produce assistant text. Source tests assert
`executeToolCall` is not called.

`structured_output` is exempt only when `getJsonSchema()` is set. This route
does not set it. `agent` inner calls are uncounted; this route excludes
`agent`.

| Input class | Upstream | Swallowtail |
| --- | --- | --- |
| omission | unlimited `-1` unless settings supply a value | retain current `16` |
| `0` | first tool tick aborts before dispatch | deliver-now |
| `1..=16` | integer accepted; N top-level launches per `runNonInteractive` | deliver-now |
| `17..` / raised | accepted up to `1_000_000` | withheld; caller-increasing |
| `-1` | unlimited | withheld |
| other negatives / fraction / NaN / overflow | rejected at resolve | invalid; never a public constructor |
| aliases | none | invalid |

Wall time stays fixed `60s`. `--max-wall-time 0` is a parser fatal in
upstream and is out of scope here.

## Counter Definitions And Lifetime

Turn counter: process-local `limitedTurnCount` in one `runNonInteractive`
call. One increment per main-loop or drain-loop iteration, including
tool-result continuations. Runtime Goal continuations skip the increment.
Not persisted in session JSONL. `--resume` starts a new process, so the
counter starts at `0` again even when chat history is restored.

Tool counter: process-local `RunBudgetEnforcer.toolCallCount`. Created with
each `runNonInteractive`. Stream-json input (`nonInteractive/session.ts`)
calls `runNonInteractive` once per user message, which is the documented
per-message reset.

Swallowtail child mapping:

| Child | Transport | Counter lifetime |
| --- | --- | --- |
| structured run | one process; ordinary text-stdin or reasoning stream-json with one user record | one `runNonInteractive`; counters start at 0 |
| first session turn | new process, no `--resume` | new counters |
| resumed session turn | new process, private `--resume <id>` | new counters; not an operation-wide remainder |
| fresh replacement | new process, no recovered session id | new counters |

Do not describe these flags as Swallowtail session-turn accounting. The
route still rejects a 25th host turn with
`swallowtail.qwen.headless.turn_limit` independently of `--max-session-turns`.

Reasoning-selected children use `--input-format stream-json` and one user
record after initialize/`set_effort`. That is one `runNonInteractive`, so
the same per-child budgets apply. Omitted reasoning keeps text-stdin. Budget
selection must not change handshake, prompt timing, model qualification,
resume, or replacement.

## Terminal Truth

`FatalTurnLimitedError` is exit 53. `FatalBudgetExceededError` is exit 55
and covers both wall-time and tool-call overrun. `handleMaxTurnsExceededError`
and `handleBudgetExceededError` write **plain stderr** for
`OutputFormat.STREAM_JSON` (this route) and structured JSON only for
`OutputFormat.JSON`. They then `process.exit`. Source comment: emitting a
stream-json semantic envelope is an acknowledged gap shared with cancel and
max-turns; it is not budget-specific.

The protocol type union includes `result.subtype = error_max_turns`, but
this exit path does not emit that record. Swallowtail already classifies:

- exit 53 → `swallowtail.qwen.headless.native_turn_limit` /
  `InputLimitExceeded` (harness)
- exit 55 → `swallowtail.qwen.headless.native_budget` /
  `InputLimitExceeded` (harness)
- corpus `terminal-observations.json` uses `stream_fixture: null` for both

Keep that classification. Do not promote stderr text or an absent
`error_max_turns` record into a stronger stream event. Exit 55 remains
ambiguous between wall time and tool-call overrun; Swallowtail already names
it a native run budget and must not split those causes without a new
semantic stream field.

Partial assistant and tool-request events may already have been streamed
before the aborting tick or the next-turn check. Cleanup stays the existing
join/reap path. Cancellation (130) and host deadline remain distinct: the
enforcer does not claim a budget event if the abort already happened.

Zero-tool usefulness: a text-only completion can succeed; a tool request
aborts before dispatch with exit 55 and the existing native-budget terminal.
That is truthful route behavior for a caller who wants no tool launches.

## Plan And Evidence Representation

Carry an adapter-local optional pair on `QwenRunProfileInput` /
`QwenSessionProfileInput` and `QwenPreparedEvidence`. Copy it onto
`QwenHeadlessDriver` / session handle for argv construction. Do not add a
shared `Capability` or portable policy field. Reject selected values unless
the installed package is exact `0.21.15`. Reject unconstructable numbers at
the typed constructor. Preserve current constructors and omission argv.

## Deliver-Now Table

| Package | Profile | Turns | Tools | Disposition | Reason |
| --- | --- | --- | --- | --- | --- |
| `0.21.15` | ordinary structured run | omit | omit | retain current | argv `24` / `16` |
| `0.21.15` | ordinary structured run | `1..=24` | omit | deliver-now | per-child decreasing turns |
| `0.21.15` | ordinary structured run | omit | `0..=16` | deliver-now | per-child decreasing tools, including zero |
| `0.21.15` | ordinary structured run | `1..=24` | `0..=16` | deliver-now | independent pair |
| `0.21.15` | reasoning-selected structured run | same as ordinary | same as ordinary | deliver-now | budgets compose with initialize/`set_effort`; one user record |
| `0.21.15` | first session turn | same pair | same pair | deliver-now | new process, no `--resume` |
| `0.21.15` | resumed session turn | same pair | same pair | deliver-now | new process; counters reset; private `--resume` unchanged |
| `0.21.15` | fresh replacement | same pair | same pair | deliver-now | new process; no recovered session id |
| any other qualified package | any | selected | selected | reject before start | exact-version gate |
| any | any | `0`, raised, `-1`, negative, fraction | — | withheld/invalid | see parser table |
| any | any | — | `-1`, raised, negative, fraction | withheld/invalid | see parser table |
| any | JSON-schema / `agent` inner calls | — | — | not applicable | route selects neither |
| any | `qwen serve` / ACP | — | — | not applicable | not this route |

## Behavior Revision And Compatibility

Keep `qwen-code.headless.v0.21.0-catalogue-filter` through `0.21.14` and
`qwen-code.headless.v0.21.15-reasoning-control` at `0.21.15`. Do not change
the published currentness range. Selected budgets are a feature-local
exact-version gate inside `swallowtail-adapter-qwen`. Prior fixture evidence
for fixed `24` / `16` remains historical proof of omission.

## Validation

Evidence-only inspection on 2026-08-23. No install, login, credential,
catalogue, or provider prompt.
