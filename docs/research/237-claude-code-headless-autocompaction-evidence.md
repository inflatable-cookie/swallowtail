# 237 Claude Code Headless Autocompaction Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.085 / 238

## Question

Which exact qualified `claude-code.headless` version, value, and operation rows
can bind caller-selected `--autocompact` with operation-private precedence,
pre-effect rejection, and honest application or effective-state truth?

## Evidence Boundary

Research must use exact official package/native artifacts for the qualified
`2.1.220..=2.1.241` route window plus frozen official documentation. Mutable
current documentation is a lead and cannot backport support. No provider
prompt, login/account work, paid operation, host install/update, or ambient
configuration mutation is authorized.

The record must freeze support membership, native value meaning, parser domain,
argv/settings/environment precedence, enablement gates, omission, and the
separation of requested, argv-encoded, accepted, effective, compaction-observed,
and usage-observed truth. It must classify every candidate row as deliver now,
evidence-gated, intentionally withheld, or not applicable. Autocompaction is
not context size or an output-token limit.

## Promotion Gate

A non-empty deliver-now row requires operation-private precedence over ambient
user, project, managed, and environment settings, plus pre-effect rejection and
prompt-free application or effective-state confirmation. An empty set is an
acceptable result.

## Method

Evidence was collected on 2026-08-27. No Claude Code installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation was used. Host `claude` was not on `PATH` and was not installed,
replaced, or updated. No ambient configuration was written.

Every published official npm package in the qualified window
`2.1.220..=2.1.241` and its `@anthropic-ai/claude-code-darwin-arm64` platform
package were downloaded to disposable cache paths. Native executables were
inspected through `--version`, `--help`, deterministic local argv probes, and
extracted implementation source. All probes ran under a throwaway `HOME` with
only `PATH` and, where required, one disposable ambient variable.

Two prompt-free probe terminals were used:

- `claude [flags] doctor` reaches full commander parsing and a local health
  action that sends no provider request. It separates *unknown option* from
  *accepted option* and exercises the `--autocompact` value parser.
- `claude [flags] -p --output-format stream-json` with closed stdin reaches the
  print-mode entry and rejects the empty prompt before authenticated provider
  work. It does not emit an autocompact effective-state field comparable to
  Fast mode's `fast_mode_state`.

The route under study is `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis `claude-code.headless-stream-json`,
qualified window `2.1.220..=2.1.241`, behavior
`claude-code.headless.stream-json.v1`. This record does not amend
`claude-code.response-only` or `claude-agent.acp`.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--autocompact` description and v2.1.221+ membership lead | 2026-08-27 | SHA-256 `9840dd3f4a81d581ffe61fa2223c146109e28b5d958283bdbbbc5dfb54e50ad5` |
| [Claude Code model config](https://code.claude.com/docs/en/model-config) | auto-compact window meaning, `/autocompact`, settings, env precedence | 2026-08-27 | SHA-256 `357dd3bd9e616f6e330c3e365ea04039ce8af7be77a8801bfb8cf684694ca8a5` |
| [Claude Code environment variables](https://code.claude.com/docs/en/env-vars) | `CLAUDE_CODE_AUTO_COMPACT_WINDOW`, `DISABLE_AUTO_COMPACT`, `DISABLE_COMPACT`, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` | 2026-08-27 | SHA-256 `0a1434823835c346217b468d27367f10b80ff609c68fcafa2a293e633375a427` |
| `@anthropic-ai/claude-code@2.1.220` wrapper tarball | window baseline identity | 2026-08-27 | SHA-256 `df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3` |
| `@anthropic-ai/claude-code@2.1.241` wrapper tarball | window ceiling identity; matches Research 202 | 2026-08-27 | SHA-256 `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6` |
| `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241` | exact option, parser, merge, resolution, and enablement evidence | 2026-08-27 | per-version digests in the support table below |
| `claude-code-2.1.241/headless-autocompaction.json` | sanitized deterministic specimen corpus | 2026-08-27 | asserted in `claude_code_headless_autocompaction_identity.rs` |

Current documentation is a lead only. It states that `--autocompact <auto|tokens>`
sets the auto-compact window for one launch without changing saved settings,
accepts the same values as `/autocompact`, requires Claude Code v2.1.221 or
later, and that `CLAUDE_CODE_AUTO_COMPACT_WINDOW` takes precedence over the
command, the flag, and the `autoCompactWindow` setting. None of that backports
onto the qualified window without exact artifacts. Every claim below rests on
the exact packages.

The wrapper npm packages carry no `--autocompact` option registration. They
remain installer wrappers; all evidence is in the platform native executable.

## Version Membership

`--autocompact` is absent from `2.1.220` and present at every published point
from `2.1.221` through `2.1.241`. That matches the current docs lead and the
exact option declaration.

| Probe | `2.1.220` | `2.1.221` and every later published point through `2.1.241` |
| --- | --- | --- |
| option declaration `--autocompact <auto\|tokens>` in binary | absent | present |
| `--help` advertises `--autocompact` | no | yes |
| `claude --autocompact auto doctor` | exit 1, `unknown option '--autocompact'` | exit 0 |
| `claude --autocompact 500k doctor` | exit 1, unknown option | exit 0 |
| `claude --nope 3 doctor` | exit 1, unknown option | exit 1, unknown option |

Official `--help` digests reproduce Research 202 at the window endpoints:
`fcd5b45507c7c602d54d85a300eab288a8a3c6770c6def696ca19a3100725de4` at
`2.1.220` and `71ad650f59e08ae40ede14c534db4f49d8590ee5a4f92f6da2882d3a5560fea6`
at `2.1.241`. Unlike `--max-turns`, `--autocompact` is not hidden; from
`2.1.221` onward help advertises it.

`2.1.230` remains unpublished. It is inside the semantic qualified window but
has no artifact, so selection must reject it.

## Native Value Meaning

The option selects an **auto-compact window**: how full the model context may
get before Claude Code summarizes the conversation. Exact declaration at every
supporting version:

```js
.addOption(new /*Option*/("--autocompact <auto|tokens>",
  "Auto-compact window size (auto, or 100k–1M tokens)")
  .argParser((value) => {
    let parsed = /*Xin|n6n*/(value);
    if (parsed === undefined)
      throw new /*InvalidArgumentError*/(
        "It must be 'auto', or between 100k and 1M (e.g. 500k, 200000, or 200 as shorthand)");
    return parsed;
  }))
```

Exact constants are `uyi = 1e5` and `B2a = 1e6` (100_000..=1_000_000 tokens).

This is not:

- a portable context-size claim
- an output-token limit
- a session-continuity / resume control
- enablement of compaction itself (`autoCompactEnabled`, `DISABLE_AUTO_COMPACT`,
  and `DISABLE_COMPACT` are separate)

`auto` means "use the model-tuned default path", not a token count. A numeric
value is a token threshold later capped by the model's context window.

## Parser Domain

The exact parser at every supporting version:

```js
function /*Xin|n6n*/(e){
  let t = e.trim().toLowerCase();
  if (t === "auto") return "auto";
  let r;
  if (t.endsWith("m")) r = parseFloat(t) * 1e6;
  else if (t.endsWith("k")) r = parseFloat(t) * 1000;
  else {
    let n = /*number coerce*/(t);
    r = n >= 100 && n <= 1000 ? n * 1000 : n; // 100..1000 are thousand-token shorthand
  }
  if (!Number.isFinite(r) || r < 1e5 || r > 1e6) return;
  return Math.round(r);
}
```

Observed dispositions are identical at `2.1.221` and `2.1.241`, and the option
declaration is byte-stable across every supporting published version:

| Input | Parser | Notes |
| --- | --- | --- |
| `auto`, `AUTO`, `Auto` | accepted as `"auto"` | case-insensitive after trim |
| `100k`, `100000`, `100` | accepted as `100000` | floor |
| `500k`, `500000`, `500`, `500K`, `" 500k "` | accepted as `500000` | trim + k/shorthand |
| `1m`, `1M`, `1000000`, `1000` | accepted as `1000000` | ceiling |
| `200`, `200k`, `200000` | accepted as `200000` | shorthand and explicit forms |
| `1e5` | accepted as `100000` | `Number("1e5")` |
| `200 as shorthand` | accepted as `200000` | leading-number coerce + 100..1000 shorthand; trailing junk ignored |
| `99k`, `99`, `99999` | rejected | below 100k |
| `1000001`, `1.5m` | rejected | above 1M or non-finite path |
| `0`, `-1`, `""`, `Infinity`, `NaN` | rejected | outside domain |
| `true`, `false`, `off`, `on`, `disable`, `none` | rejected | not enablement vocabulary |
| flag with no value | rejected | `argument missing`, exit 1 |

Invalid values exit `1` before doctor or print work. That is pre-effect
rejection for the argv parser only. It does not prove the selected window is
effective once ambient environment or enablement gates intervene.

## Flag Merge And Resolution Precedence

Print-mode startup merges the flag with saved settings, then resolves the
effective window:

```js
// merge: flag wins over saved autoCompactWindow; "auto" clears to undefined
function /*XJu|mNp*/(flag, saved){
  if (flag === undefined) return saved;
  return flag === "auto" ? undefined : flag;
}

// resolve: environment beats the merged settings/flag value
function /*EX|_V*/(model, mergedSettingsValue){
  if (process.env.CLAUDE_CODE_AUTO_COMPACT_WINDOW) {
    let parsed = /*clamp plain integer*/(
      "CLAUDE_CODE_AUTO_COMPACT_WINDOW",
      process.env.CLAUDE_CODE_AUTO_COMPACT_WINDOW,
      1e5, 1e6);
    if (parsed.status !== "invalid") {
      let configured = Math.max(1e5, parsed.effective);
      return { window: Math.min(modelLimit, configured), configured, source: "env" };
    }
  }
  if (mergedSettingsValue !== undefined)
    return { window: Math.min(modelLimit, mergedSettingsValue),
             configured: mergedSettingsValue, source: "settings" };
  // then clientdata / experiment / model-default / auto
}
```

Consequences that are exact at every supporting version:

1. Explicit `--autocompact <tokens>` overrides saved `autoCompactWindow` for the
   launch and does not write settings.
2. Explicit `--autocompact auto` clears the saved numeric window for the launch
   so resolution falls through to model-default/auto paths.
3. **`CLAUDE_CODE_AUTO_COMPACT_WINDOW` overrides the flag.** When the env var is
   present and not invalid, resolution returns `source: "env"` and never reads
   the merged flag/settings value.
4. Docs and exact strings agree: `/autocompact` reports
   `CLAUDE_CODE_AUTO_COMPACT_WINDOW is set and takes precedence` when env wins.
5. The env parser is a plain-integer clamp, not the flag's `auto|k|m|shorthand`
   parser. Current docs state a value like `500k` reads as `500` and clamps to
   the 100k minimum. That is a separate ambient encoding, not the flag domain.

Doctor probes confirm the ambient override is invisible at parse time:
`CLAUDE_CODE_AUTO_COMPACT_WINDOW=300000` plus `--autocompact 500k doctor` still
exits `0`. The flag is accepted; precedence is applied later in resolution.
There is no argv-level rejection that would let Swallowtail fail closed without
inspecting the approved environment.

## Enablement And Sibling Ambient Gates

Window selection is not enablement. Exact enablement gate:

```js
function /*nNp*/(){ return Boolean(DISABLE_COMPACT || DISABLE_AUTO_COMPACT); }
function /*mO|kO*/(){
  if (/*nNp*/()) return false;
  return /*settings*/("autoCompactEnabled", true).value;
}
```

`DISABLE_AUTO_COMPACT=1` or `DISABLE_COMPACT=1` disables automatic compaction
even when `--autocompact` is accepted. `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`
additionally changes the percentage of the window at which compaction triggers.
Those ambient controls sit beside the window selector and can make a selected
window inert or earlier without changing argv.

## Application And Effective-State Observability

No prompt-free headless confirmation of the effective auto-compact window was
observed. Empty-prompt print-mode exits with
`Error: Input must be provided either through stdin or as a prompt argument when
using --print` and emits no autocompact state fields analogous to Fast mode's
`fast_mode_state` / `fast_mode_disabled_reason`.

Compaction itself fires only after context usage approaches the resolved window
during a real conversation. Observing compaction or usage effects requires
provider work this card does not authorize. Interactive `/autocompact` and
settings UI strings exist in the package but are not the headless route's
confirmation seam.

## Operation-Private Precedence Against The Route

`claude-code.headless` inherits an opaque approved environment reference and
passes it through unchanged. Swallowtail does not inspect, scrub, or rewrite
that environment. Because `CLAUDE_CODE_AUTO_COMPACT_WINDOW`,
`DISABLE_AUTO_COMPACT`, `DISABLE_COMPACT`, and `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`
can override or nullify caller selection, an explicit `--autocompact` argv is
**not** operation-private under the route's current authority.

This is the opposite of Research 226's `--max-turns` result, where explicit argv
unconditionally short-circuits `CLAUDE_CODE_MAX_TURNS`. Here the environment
short-circuits the flag.

Saved `autoCompactWindow` and `autoCompactEnabled` settings remain readable by
the child through the existing `--setting-sources user,project,local` argv.
The flag overrides the saved window for one launch, but that does not defeat
environment or disable gates, and this lane did not mutate ambient settings to
prove isolation.

## Omission

Omission emits no `--autocompact` argument and leaves the exact current command
byte-identical to Research 226 / the current headless driver:

```text
claude -p --input-format text --output-format stream-json --verbose
  --no-session-persistence --model <selected> [--effort <selected>]
  [--max-turns <selected>] --permission-mode plan --tools Read,Glob,Grep
  --setting-sources user,project,local --mcp-config {"mcpServers":{}}
  --strict-mcp-config
```

Omission makes no compaction, threshold, context-size, token-limit, or default
claim. Ambient env and settings remain whatever the approved environment and
setting sources already provide.

## Truth Separation

| Layer | What this lane proves | What remains unproved / blocked |
| --- | --- | --- |
| requested | caller could append `--autocompact <auto\|tokens>` from `2.1.221` | operation-private effective selection |
| argv-encoded | parser domain and pre-effect rejection of invalid values | env still accepted beside valid argv |
| accepted | commander accepts valid values at doctor | acceptance ≠ effective window |
| effective | env/`DISABLE_*` can override or disable after acceptance | cannot confirm without inspecting opaque env or prompting |
| compaction-observed | not observed | requires provider conversation growth |
| usage-observed | not observed | requires provider work |

Autocompaction is not context size, output-token limit, Fast mode, maximum
turns, or provider session continuity.

## Claim Strength

| Claim | Strength at the exact evidence boundary |
| --- | --- |
| `--autocompact` exists at `2.1.220` | not observed; unknown option |
| `--autocompact` exists at every published `2.1.221..=2.1.241` | observed |
| value selects auto-compact window tokens or `auto` | observed from exact option + resolver |
| invalid values reject before process work | observed at doctor |
| explicit argv has operation-private precedence over ambient env | **not observed; env wins** |
| effective window confirmable without provider prompt | not observed |
| compaction firing confirmable without provider prompt | not observed |
| selection is context size or output limit | rejected |
| current headless argv already passes `--autocompact` | not observed; omission preserved |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `--autocompact <auto\|tokens>` at published `2.1.221..=2.1.241` | registered, parsed, docs-aligned window selector | **empty / withheld**; ambient env and disable gates defeat operation-private precedence; effective state unconfirmed without provider work |
| `--autocompact` at `2.1.220` | unknown option | rejected; outside flag membership |
| omission | current argv unchanged; no compaction claim | unchanged; remains exact |
| invalid values | pre-effect parser rejection | rejection evidence only; not a deliverable binding |
| `CLAUDE_CODE_AUTO_COMPACT_WINDOW` scrub or inspection | would be required for private precedence | rejected; out of scope |
| `DISABLE_AUTO_COMPACT` / `DISABLE_COMPACT` scrub | can nullify selected window | rejected; out of scope |
| portable context-size / token-limit control | product-specific compact window | rejected |
| `claude-code.response-only`, `claude-agent.acp` | separate routes | not applicable |
| `2.1.230` | unpublished | evidence-gated; reject |
| `2.1.242`+ | not probed | evidence-gated; reject |

Deliver-now rows: **none**.

## Support Table

Every published version in `2.1.220..=2.1.241` was probed on darwin-arm64.

| Version | Option present | Help advertises | Parser accepts `auto`/`500k` | Parser rejects `bogus` | Env overrides flag in source | Binary SHA-256 |
| --- | --- | --- | --- | --- | --- | --- |
| 2.1.220 | no | no | n/a (unknown option) | n/a | settings/env exist; no CLI flag | `8addc857f3fe64d5a0368af9ee50321b50afb4a6918ba3ef018ab84f5dbbe081` |
| 2.1.221 | yes | yes | yes | yes | yes | `7a181f36ed0fc4fbac6cee4ecf2b615eff93d8b434221fff5d7c878dc5ebf380` |
| 2.1.222 | yes | yes | yes | yes | yes | `c66a6cc6fa2e8145bb1a6e77831f2caf4b83690ff04650500dfa6e2c05ca997c` |
| 2.1.223 | yes | yes | yes | yes | yes | `fcbe0b8d47570c501302dd1ad31cc26ac2810f022c45fa253936a6961dee32bf` |
| 2.1.224 | yes | yes | yes | yes | yes | `391df9d2ab04e4cf32199335720ac7715a582e91eaecfd4d2198a16f57ea59b3` |
| 2.1.225 | yes | yes | yes | yes | yes | `08d6e85dd2b80883bb8da93cbeae3dc79b4704d6b84a05d614bf1ff4a5155b69` |
| 2.1.226 | yes | yes | yes | yes | yes | `013a1cf17df5ff1dcc189d5d6fd3fdd5f097ddc3cd41aa9992e99805574febbe` |
| 2.1.227 | yes | yes | yes | yes | yes | `7432511ba3be818e01f23f6eef8630d214a8b618451e188c3c7d61a987eef6c7` |
| 2.1.228 | yes | yes | yes | yes | yes | `43484b1352cef03a08346f36ef0437755b1aad646ab9313ce187857b794b7247` |
| 2.1.229 | yes | yes | yes | yes | yes | `d732f0ba0a539c58c2ffcaef06ed03b4e523726f0cb6cc27b3a5b7e7ae0a7a21` |
| 2.1.231 | yes | yes | yes | yes | yes | `ba790279cab6ef77b713864d4bf5f764fcea87d3a3eb7591a41f741e45212b5c` |
| 2.1.232 | yes | yes | yes | yes | yes | `7b39c1588df919d001dea3ffd5651adb682f2451b5a0e18d42d4233296b53cc7` |
| 2.1.233 | yes | yes | yes | yes | yes | `bc466b6cde63edafc773f471a1fb98787fabb31f52240c8616ce7e1f587b212d` |
| 2.1.234 | yes | yes | yes | yes | yes | `08d8700313697cbe730a25420c908a299ce52d56f0eb2cf4fac94cab5109bc57` |
| 2.1.235 | yes | yes | yes | yes | yes | `83b8f806f6f2eea316cfe246628e6c23374711d868f1fd0409db551b877b7748` |
| 2.1.236 | yes | yes | yes | yes | yes | `6bc4ba992d2786cbf0237c4453ca53c1fdf0c3b3d83ffa0025c0d8190ed27848` |
| 2.1.237 | yes | yes | yes | yes | yes | `338901351d4ff17495738c67fc3e12a32c1b506738ac5e012eb782d3d8b5be43` |
| 2.1.238 | yes | yes | yes | yes | yes | `1c196c456373b57818ae87df84aecee96cb659448c0d6a6bbb401ac5758431b2` |
| 2.1.239 | yes | yes | yes | yes | yes | `2b4f7aafdaa65bcc2335f56a4b276317837203f2c5587b1f2a17ca78ad14e36f` |
| 2.1.240 | yes | yes | yes | yes | yes | `8917e01c99ea0ce6ed887a1729a4cda693c758fe542747be71756987b145c772` |
| 2.1.241 | yes | yes | yes | yes | yes | `1495eb7c42d3b4451f5f1cd38b6d498d22a4a38c802bc2be5c1cf1795e64820d` |

Resolution and enablement source shapes were extracted in full at `2.1.221` and
`2.1.241` and matched. Supporting versions share the same option declaration
string and doctor parser dispositions; the lane treats env-override and
disable-gate authority as window-wide for every supporting published point.

## Decision

Card 238 is complete as an evidence stop with an honest empty set. Exact
packages expose a real `--autocompact` window selector from `2.1.221` through
`2.1.241`, with a closed parser domain and pre-effect rejection of invalid
values. That is not enough. Ambient `CLAUDE_CODE_AUTO_COMPACT_WINDOW` overrides
the flag, disable env vars can nullify compaction, the approved environment is
opaque, and effective compaction cannot be confirmed without a provider prompt.

No production binding, guide capability claim, matrix row, or shared closeout
follows from this record. The current headless route, reasoning, maximum-turns,
and omission behavior stay unchanged.

## Sources

- npm `@anthropic-ai/claude-code` `2.1.220` and `2.1.241`
- official `@anthropic-ai/claude-code-darwin-arm64` every published `2.1.220..=2.1.241`
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Claude Code model config](https://code.claude.com/docs/en/model-config)
- [Claude Code environment variables](https://code.claude.com/docs/en/env-vars)
- [Research 202 Claude Code 2.1.241 Identity](./202-claude-code-2-1-241-identity.md)
- [Research 226 Claude Code Headless Maximum Turns](./226-claude-code-headless-maximum-turns-evidence.md)
- [Research 233 Claude Code Headless Fast Mode](./233-claude-code-headless-fast-mode-evidence.md)
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-autocompaction.json`
