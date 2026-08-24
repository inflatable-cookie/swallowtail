# 200 Qoder Headless Maximum-Turn Evidence

Status: promoted; claim correction complete; empty deliver-now
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

No. Research 200 admits an empty deliver-now set. No caller-selectable
maximum-turn feature is admitted.

Exact `@qoder-ai/qodercli@1.1.25` registers `--max-turns <count>` as a raw
string option (no Commander `argParser`) and copies the parsed value onto
Config `maxSessionTurns`. The selected CLI headless QueryEngine factory
(`entrypoint: "cli"`, headless transcript writer) constructs the engine with
literal `maxTurns: kN` where `kN = 1000`. QueryEngine.driveQuery then passes
`maxTurns: this.config.maxTurns ?? kN` into AgentLoop. `getMaxSessionTurns()`
is used only by the text-output `error_max_turns` formatter, not by AgentLoop.

Operator disposition (2026-08-24):

- Retain exact current argv `--max-turns 8` as historical inert compatibility
  state. Do not claim it sets the AgentLoop ceiling.
- The AgentLoop ceiling on this route is factory `1000`.
- Synthetic `error_max_turns` evidence proves decoder mapping only.
- Do not remove the flag. Do not add a caller-selectable max-turn control.
- Cards 149–150 stay blocked.

Corpus, guide, architecture, matrices, fixtures, and comments are reconciled
to that truth. Runtime argv bytes are unchanged.

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

`MAXIMUM_TURNS` is fixed `"8"` in `command.rs` as historical inert argv.
Prepared input carries no turn selection. Driver joins one child.

Stream-json `error_max_turns` with `is_error: true` maps to `ProviderFailed` /
`swallowtail.qoder.headless.max_turns`. Fixture `limit.jsonl` freezes a
synthetic envelope (`errors: ["Maximum turns exceeded"]`, `num_turns: 1`) for
decoder classification only. It does not prove argv `8` stops at turn 8.
Cancellation and host deadline stay distinct. `stream_event` remains ignored.

## Parser Domains Versus Swallowtail Domains

Commander registers:

```js
.addOption(new ni("--max-turns <count>", "Maximum turns per query").hideHelp())
```

No `.argParser`, choices, min, or max. The CLI value is a **raw string**. After
parse:

```js
void 0 !== t.maxTurns && (QA.maxSessionTurns = t.maxTurns)
```

Config construction defaults `this.maxSessionTurns = A.maxSessionTurns ?? -1`.
Settings schema labels `model.maxSessionTurns` default `-1` (telemetry
snapshot treats that as `"unlimited"`). That domain is Config /
text-formatter only on the selected path.

| Input class | Upstream Config `maxSessionTurns` | Selected AgentLoop `maxTurns` | Swallowtail disposition |
| --- | --- | --- | --- |
| route argv `--max-turns 8` | raw string `"8"` | factory `kN` (`1000`) | retained historical inert argv |
| upstream flag omission | settings / `-1` | factory `kN` (`1000`) | not route argv; forbidden as `omit-max-turns-not-route-argv` (historical inert requirement, not AgentLoop-unbounded) |
| caller `1..=8` selector | would copy to `maxSessionTurns` | still factory `kN` | **withhold**; no deliver-now feature |
| raised / zero / negative / fraction / overflow | raw string into `maxSessionTurns` | factory `kN` | withhold/invalid |
| cron/goal/agent `maxTurns` | separate | separate | not this route |

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
TUI/input path). They are not this route's selected print factory.

`getMaxSessionTurns()` appears only as:

1. Config getter returning `this.maxSessionTurns`
2. text-output formatter for `error_max_turns`:
   `Error: Reached max turns (${getMaxSessionTurns()})`

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
`num_turns`. Stream-json exit helper sets `process.exitCode` from `is_error`.

Swallowtail maps that subtype to provider-failed via deterministic fixtures.
That decoder truth is separate from proving argv `8` is the AgentLoop ceiling.

Cancellation and host deadline stay distinct. Partial `stream_event` envelopes
stay ignored.

## Plan And Evidence Representation

No typed adapter-local carrier is admitted. Feature-local revision: none.
Historical inert argv `8` remains the only route max-turns byte.

## Deliver-Now Table

| Release | Profile | Turns | Disposition | Reason |
| --- | --- | --- | --- | --- |
| `1.1.25` | ordinary structured run | fixed argv `8` | retain historical inert | does not set AgentLoop; factory `1000` |
| `1.1.25` | ordinary structured run | caller `1..=8` | **withhold** | CLI value not wired into AgentLoop |
| `1.1.25` | ordinary structured run | raised / zero / negative / fraction / overflow | withhold/invalid | see parser table |
| any | any | omit route `--max-turns` | forbidden | historical inert route argv; not AgentLoop-unbounded |
| any other release | any | selected | reject before start | route is exact `1.1.25` only |
| any | ACP / SDK / TUI / continue / resume / goal / cron | — | not applicable | not this route |
| any | `--max-output-tokens` | — | not applicable | unmapped sibling |

Deliver-now caller-selection rows: **none**.

## Behavior Revision And Compatibility

Claim/corpus correction only. Runtime argv bytes unchanged. Exact `1.1.25`
qualified-only membership unchanged. No Contract 029 point, no new capability
row, no public API change, no caller-selectable max-turns feature.

## Validation

Evidence-only inspection on 2026-08-24. No install, login, credential,
catalogue, or provider prompt. Claim surfaces reconciled under operator
direction on 2026-08-24.
