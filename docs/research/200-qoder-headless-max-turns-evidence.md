# 200 Qoder Headless Maximum-Turn Evidence

Status: promoted; claim reconciliation paused for operator
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.053 / 148

## Question

Can exact Qoder CLI `1.1.25` route `qoder.headless` expose a typed
caller-decreasing positive `--max-turns` selection while preserving current
omission, exact `error_max_turns` terminal truth, and every fixed route
boundary?

## Decision

No deliver-now caller-decreasing binding is admitted.

Exact `@qoder-ai/qodercli@1.1.25` registers `--max-turns <count>` as a raw
string option (no Commander `argParser`) and copies the parsed value onto
Config `maxSessionTurns`. The selected CLI headless QueryEngine factory
(`entrypoint: "cli"`, headless transcript writer) constructs the engine with
literal `maxTurns: kN` where `kN = 1000`. QueryEngine.driveQuery then passes
`maxTurns: this.config.maxTurns ?? kN` into AgentLoop. `getMaxSessionTurns()`
is used only by the text-output `error_max_turns` formatter, not by AgentLoop.

Therefore:

1. Caller argv `--max-turns N` is **not** proven to enforce N AgentLoop turns
   on this route. Binding public `1..=8` would overstate dispatch truth.
2. The same wiring **contradicts** existing qualified-route claims that treat
   flag omission as AgentLoop-unbounded and `--max-turns 8` as a required
   positive headless **loop** bound. Those claims must be reconciled before
   card 148 closes. Correcting them may change the qualified route (adapter
   argv, fixtures, guide), so this lane pauses for operator planning rather
   than preserving the invalidated assumption or silently rewriting production
   surfaces.

Zero, negative, fractional, raised, and overflow stay withheld or invalid. No
shared `Capability`, Contract 040 `OutputTokenLimit`, or Contract 029 change
is admitted from this record alone.

## Contradiction With Existing Qualified Claims

These corpus / route surfaces still assert truths Research 200 falsifies for
the selected stream-json print path:

| Surface | Current claim | Exact `1.1.25` selected-path fact |
| --- | --- | --- |
| Research 151 Authority | `--max-turns` has no bound unless passed; Swallowtail must pass a positive bound | CLI populates `maxSessionTurns` only; AgentLoop bound is factory `kN` (`1000`) whether or not the flag is passed |
| `command.rs` | `Required positive CLI turn bound`; always emits `--max-turns 8` | argv `8` is not proven to set AgentLoop `maxTurns` to 8 |
| Guide | Must pass `--max-turns 8` | Same; documents a historically required argv, not a proven loop cap of 8 |
| Fixtures | `require_max_turns: true`; `omit-max-turns-unbounded`; `omit_max_turns_forbidden` | Upstream flag omission leaves `maxSessionTurns` at settings/`-1`, but the selected factory still hardcodes AgentLoop `maxTurns: 1000`. Omission is not AgentLoop-unbounded on this path |
| `limit.jsonl` | `error_max_turns` with `num_turns: 1` | Synthetic decoder fixture. It proves the adapter maps that envelope to `swallowtail.qoder.headless.max_turns`. It does **not** prove argv `8` produces a limit at turn 8. The selected factory AgentLoop ceiling is 1000 |

Distinct truths that remain:

- Decoder maps a synthetically supplied `error_max_turns` / `Maximum turns
  exceeded` envelope to provider-failed (fixture + unit tests).
- Production still emits fixed argv `--max-turns 8` today.
- Official docs still describe `--max-turns` as a conversation-turn limit.

Do not treat the guide, Research 151 Authority paragraph, or fixture
`require_max_turns` / omit-unbounded story as settled until the operator picks
a reconciliation plan.

## Frozen Official Evidence

Fetched without credentials on 2026-08-24. Bodies are Next.js HTML; digests
identify the retrieved bodies and are not a compatibility guarantee. Alias
`https://docs.qoder.com/cli/headless` returned 404.

| Surface | URL | Date | Complete-body SHA-256 |
| --- | --- | --- | --- |
| CLI overview | <https://docs.qoder.com/cli/overview> | `Mon, 24 Aug 2026 07:50:52 GMT` | `6089c90e6196a159e70ed3c40b5f58783d288cd63ba469a5b7df37406b2b4d71` |
| Run in scripts | <https://docs.qoder.com/cli/run-in-scripts> | `Mon, 24 Aug 2026 07:50:53 GMT` | `b38f9c4832a19ee8da28dd9ca4b242a486878a21e6b7a718aa03209847d44430` |
| CLI reference | <https://docs.qoder.com/cli/cli-reference> | `Mon, 24 Aug 2026 07:50:55 GMT` | `7b32577a5378d52e1db3f96b69b22a95b9042368770318ad4861a7e733e7e2ae` |
| How it works | <https://docs.qoder.com/cli/how-it-works> | `Mon, 24 Aug 2026 07:50:57 GMT` | `8c34dac161e623d65f30d8e9eb5165e2088c3da4de8e298b9b19161ac5cb9fbd` |

Official text:

- `--max-turns`: limit Conversation Turns per query / single run; automation
  anti-loop
- run-in-scripts example: `--max-turns 20` with `--output-format json` and
  `accept_edits` (not this route's stream-json / `dont_ask` wire)
- sibling `--max-output-tokens` is unmapped

Official docs do not define the counter, off-by-one boundary, parser domain, or
exit mapping. Exact package source owns those facts.

## Frozen Exact Package Evidence

Downloaded official npm `@qoder-ai/qodercli@1.1.25` on 2026-08-24 without
install, login, credential, or live `--print`.

| Surface | Value |
| --- | --- |
| Registry GET | `https://registry.npmjs.org/@qoder-ai/qodercli/1.1.25` at `Mon, 24 Aug 2026 07:51:37 GMT` |
| Metadata body SHA-256 | `eb1a541837f3b5062c6160bec3e694d266ff67a776d1bac560ddf152638771c4` |
| Tarball | `https://registry.npmjs.org/@qoder-ai/qodercli/-/qodercli-1.1.25.tgz` |
| Tarball SHA-256 | `627749221c609bfb5514f4486fb42f464597cf49472ed52c087c36a1d2fbb4ab` |
| Integrity | `sha512-Z1U7W+RBtnHVxiqt8eCySMwxkGXaGzkFQNFbI5QPpkGSAY4Mz2WTLrI+1l2x5tnK4ftru/PkS1jEDnogJV8Tpw==` |
| Shasum | `16374dc8b576e263a74f934c10a00a07d03fcd63` |
| File count / unpacked | 33 / 62086670 |

Tagged headless source digests match Research 151 / fixture `identity.json`:

| Path | SHA-256 |
| --- | --- |
| `package/package.json` | `459d820e451a6bdfd34c9799a841f2bcb66eaae155316e497cb1b12d44b53310` |
| `package/bundle/qodercli.js` | `77f7387974d5df79c7127bb41c9c7be8aad82aa567512ca2d9f780b2e3f73d52` |
| `package/bundle/qoder-npm-dispatcher.cjs` | `f8aabc49f26ec4a8c98302cf239d5e1a6e4c1efaccea18cdf8b2ad147de6cb6a` |
| `package/postinstall.cjs` | `5b9995a17600678f17b4226582ed45dce097c6f79249d9a546c7453fc5f8f220` |

Existing route fixtures:
`crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.25/`.

## Current Swallowtail Mapping

Every structured run emits:

```
qodercli --print --output-format stream-json --permission-mode dont_ask
  --max-turns 8 --no-session-persistence --cwd <cwd> <prompt>
```

`MAXIMUM_TURNS` is fixed `"8"` in `command.rs`. Prepared input carries no turn
selection. Driver joins one child; stream-json `error_max_turns` with
`is_error: true` maps to `ProviderFailed` /
`swallowtail.qoder.headless.max_turns`.

Fixture `limit.jsonl` freezes a synthetic envelope
`errors: ["Maximum turns exceeded"]` and `num_turns: 1`. That freezes decoder
classification only. Cancellation and host deadline stay distinct.
`stream_event` remains ignored.

## Parser Domains Versus Swallowtail Domains

Commander registers:

```js
.addOption(new ni("--max-turns <count>", "Maximum turns per query").hideHelp())
```

No `.argParser`, choices, min, or max. The CLI value is therefore a **raw
string**. After parse:

```js
void 0 !== t.maxTurns && (QA.maxSessionTurns = t.maxTurns)
```

Config construction defaults `this.maxSessionTurns = A.maxSessionTurns ?? -1`.
Settings schema labels `model.maxSessionTurns` default `-1` (telemetry
snapshot treats that as `"unlimited"`). That domain is Config /
text-formatter only on the selected path.

| Input class | Upstream Config `maxSessionTurns` | Selected AgentLoop `maxTurns` | Swallowtail disposition |
| --- | --- | --- | --- |
| caller omission of public selector | N/A (adapter still emits argv `8` today) | factory `kN` (`1000`) | not a deliver-now selector; claim about "required loop bound 8" is unsettled |
| upstream flag omission | settings / `-1` | factory `kN` (`1000`) | not AgentLoop-unbounded; fixture/guide "unbounded omit" claim is unsettled |
| `1..=8` as argv | raw string copied to `maxSessionTurns` | still factory `kN` unless engine `config.maxTurns` is set — it is not on this factory | **withhold**; not wired into AgentLoop |
| `9..` / raised | parser-accepted into `maxSessionTurns` | same factory `kN` | withheld; caller-increasing and unwired |
| `0` / negative | raw string accepted into `maxSessionTurns`; settings `-1` means unlimited in telemetry | factory `kN` | withheld/invalid |
| fractional / non-number | raw string stored; no Commander coercion | factory `kN` | invalid for public domain |
| overflow / huge | not proven | factory `kN` | withheld |
| cron/goal/agent `maxTurns` | separate surfaces | separate | not this route |

`--max-output-tokens` stays unmapped.

## Counter Definitions And Lifetime

AgentLoop (`fBl` / `cM`) takes `maxTurns: C`. At loop top:

```js
if (null != C && EA.turnCount >= C)
  return await z({ reason: "max_turns", numTurns: EA.turnCount, ... })
```

`Iir` / `mir` initialize `turnCount: 0`. After a completed model iteration the
state copies `turnCount: _` where `_ = EA.turnCount + 1`. So with bound `C = N`,
the loop allows N increments then stops when `turnCount >= N` before the next
request. Compaction and tool continuations are part of the same loop; they are
not a separate Swallowtail counter.

QueryEngine.driveQuery passes:

```js
maxTurns: this.config.maxTurns ?? kN
```

with package constant `kN = 1e3` (1000).

**Decisive selected-path site:** the CLI headless session QueryEngine
construction with `entrypoint: "cli"` (`Kio`) and headless transcript writer
(`"headless_writer_created"`) sets literal `maxTurns: kN`. It does not read
`config.getMaxSessionTurns()`.

Three other literal `maxTurns: kN` sites exist in the same bundle (ACP
`entrypoint: "acp"`, remote-control `entrypoint: "remote-control"`, and a
TUI/input path). They are not this route's selected print factory and are not
cited as route evidence beyond noting they also hardcode `kN`.

`getMaxSessionTurns()` appears only as:

1. Config getter returning `this.maxSessionTurns`
2. text-output formatter for `error_max_turns`:
   `Error: Reached max turns (${getMaxSessionTurns()})`

Therefore CLI `--max-turns` affects the text diagnostic string's reported
bound, not the AgentLoop cap used by the selected stream-json print path.

Lifetime is one print child. Continue/resume/teleport/ACP/SDK/TUI/goal/cron
paths are unselected.

## Terminal Truth

When AgentLoop returns `reason: "max_turns"`, QueryEngine emits:

```js
buildResultError("error_max_turns", {
  errors: ["Maximum turns exceeded"],
  numTurns: F.numTurns,
  ...
})
```

Result schema admits subtype `error_max_turns` with `is_error: true` and
`num_turns`. Stream-json exit helper `oHn` sets `process.exitCode` from
`is_error` (1 on error). Text formatter `sHn` writes a max-turns line and also
exits 1.

Swallowtail already maps that subtype to provider-failed, not Completed, via
deterministic fixtures. That decoder truth is separate from proving argv `8`
is the AgentLoop ceiling.

Earlier assistant `OutputDelta` events may exist before the terminal result.
They must not flip the native limit into success. Current finalize attaches
output only on Completed.

Cancellation (`error_during_execution` / abort → Cancelled) and host deadline
(TimedOut) stay distinct. Partial `stream_event` envelopes stay ignored.

## Plan And Evidence Representation

No typed adapter-local carrier is admitted. A future binding would need exact
wiring of caller value into AgentLoop `maxTurns` (or a package revision that
does so), then immutable plan/evidence agreement. That is outside this card.

Feature-local revision: none until claim reconciliation settles whether fixed
argv `8`, `require_max_turns`, and omit-forbidden remain part of the qualified
route.

## Deliver-Now Table

| Release | Profile | Turns | Disposition | Reason |
| --- | --- | --- | --- | --- |
| `1.1.25` | ordinary structured run | omit public selector | not deliver-now; claim unsettled | adapter still emits argv `8`; not proven as AgentLoop bound 8 |
| `1.1.25` | ordinary structured run | `1..=8` | **withhold** | CLI value not wired into AgentLoop `maxTurns` on selected factory |
| `1.1.25` | ordinary structured run | raised / zero / negative / fraction / overflow | withhold/invalid | see parser table |
| any | any | upstream flag omission | claim unsettled | Config `-1` ≠ AgentLoop unbounded; selected factory still `kN` |
| any other release | any | selected | reject before start | route is exact `1.1.25` only |
| any | ACP / SDK / TUI / continue / resume / goal / cron | — | not applicable | not this route |
| any | `--max-output-tokens` | — | not applicable | unmapped sibling |

Deliver-now rows: **none**.

## Behavior Revision And Compatibility

Do not close card 148 or declare private behavior / guide / fixture claims
unchanged. Research 200 falsifies the AgentLoop reading of those claims for
exact `1.1.25`.

Operator planning must choose how to reconcile the qualified route, for
example:

- keep emitting `--max-turns 8` as historical argv while rewriting corpus
  claims to state the real AgentLoop ceiling is factory `1000`, and narrow
  `error_max_turns` proofs to decoder-only; or
- change adapter/fixtures/guide (`require_max_turns`, omit handling, comments)
  to match exact wiring — which is a qualified-route change, not docs-only.

This worker lane does not pick that plan. No Contract 029 point, matrix row,
or public API change follows from this record alone.

## Validation

Evidence-only inspection on 2026-08-24. No install, login, credential,
catalogue, or provider prompt.
