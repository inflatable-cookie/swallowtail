# 188 Copilot CLI ACP Effort Evidence

Status: promoted
Owner: Tom
Created: 2026-08-22
Updated: 2026-09-01
Card: g04.040 / 110

## Question

Which exact Copilot CLI ACP server-start effort values can Swallowtail bind on
qualified package `1.0.80` as one immutable prepared-session reasoning
selection without model inference, value clamping, or an overstated effective
effort claim?

## Method And Boundary

Official GitHub Copilot CLI documentation and exact `@github/copilot@1.0.80`
package artifacts were inspected on 2026-08-22. No Copilot install, login,
account inspection, initialize, prompt, or provider output was used. The
platform tarball was extracted only far enough to read `package.json` and
`app.js`; the native `copilot` binary was not executed.

The existing route remains `copilot-cli.acp`, driver
`swallowtail.copilot-cli.acp`, axis `copilot-cli.package` `1.0.80`, behavior
`copilot-cli.acp.stdio-v1`. There is no selected model route. Current argv is
exactly `copilot --acp --stdio`.

Current official ACP-server documentation is a lead. Exact `1.0.80` `app.js`
is the package finding.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [ACP server](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server) | `--effort` / `--reasoning-effort` values, server-start scope, `session/new` does not carry reasoning | 2026-08-22 | `ef71569052de6d953a97369ecd9547c87568036d2679528a670165b2a16ab429` |
| [CLI command reference](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-command-reference) | `--effort=LEVEL` alias of `--reasoning-effort`; five named values; `max` described as Anthropic highest-depth | 2026-08-22 | `3c94875ea5afa28e0fe0fd0b4d9b4b66d471a38e25a2526cbbaebbf348cd7dfb` |
| npm `@github/copilot@1.0.80` wrapper tarball | identity; no effort flags in the four-file loader | 2026-08-22 | `799457937f8f87de6fdc95599380de5f5a0f761ab2fdfbba7f8d1c82d2988892` |
| npm `@github/copilot-darwin-arm64@1.0.80` tarball | exact package source | 2026-08-22 | `98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71` |
| extracted `package/app.js` | commander choices, ACP `newSession`, clamp, and `setSessionConfig` | 2026-08-22 | `fa438a4959c3f8b44c123dd95cc0d8b0760faa055a458f81ddd0dcf8a2ae8f58` |
| GitHub tag `v1.0.80` `changelog.md` | tagged public changelog still opens at `1.0.79` | 2026-08-22 | `dc33796e78584aa89e307ed2005af1563ed2663d9e7612d1b14a696b9cb85892` |

Wrapper integrity matches Research 149:
`sha512-6tf93ZF56KOiTTAjK/UhLZkl1W543IzaTQly288kockJZFswpRTnQEI00Yvacpb39DTvTYu3/ha9SeKpo/pgZQ==`.
`buildMetadata.gitCommit` remains `a3a2697`. Platform optional
`@github/copilot-darwin-arm64@1.0.80` integrity remains
`sha512-fzn4PnSx3+O/a3ip72KVsjnzORsEygK+0i21bFAnFBYS+0Wi1Pk+o/CmNsJ7aRbf1enSJrcH8UDVkyc9pMGEBg==`.

The documentation SHA-256 values above identify the retrieved 2026-08-22 HTML
page bodies. Those bodies are Next.js SPA shells; they do not identify
converted documentation text. Research 218 supersedes this retrieval method:
binding corpus digests are `.md` exports, with HTML as corroboration only. The
table above remains historical truth and is not rewritten. Package and
extracted-artifact digests are unaffected. HTML digests are not a
compatibility guarantee. The tagged changelog is not the ACP implementation
source.

## Exact Package Specimens

Commander registers one option with two names; choices come from native
`reasoningEffortLevels()`:

```js
NK=h.reasoningEffortLevels()
// ...
new Zi("--effort, --reasoning-effort <level>","Set the reasoning effort level").choices(NK)
```

Startup reads the parsed flag as `t.reasoningEffort ?? opts.reasoningEffort ?? opts.effort`.
ACP then stores that value as the session-initial effort:

```js
resolveCliReasoningEffort(){let e=this.options.options;return e?.reasoningEffort??e?.effort}
async resolveInitialReasoningEffort(){let e=this.resolveCliReasoningEffort();if(e!==void 0)return e;try{return(await Gt.load(this.options.settings))?.effortLevel}catch{...}}
```

`session/new` in `1.0.80` reads `e.cwd` and `e.mcpServers` only. It does not
read a reasoning field. That matches the official lead.

ACP still exposes a later `reasoning_effort` session config option. Setting it
requires the currently selected model and that model's entitled list. An
unsupported value is rejected. A model-list refresh substitutes an unsupported
startup value with the model default:

```js
g=kN(...)           // model default
p=e.reasoningEffort??g
m=u.includes(p)?p:g // substitute default when the current value is not entitled
```

Explicit non-ACP application of a CLI value fail-closes against the current
model through `$f` (exact membership, no nearest-value map). Omitted CLI effort
uses `QZ`, which falls back to the model default. `kN` default is `"medium"`
except `kimi-k3` `"high"`. `"none"` is rewritten to `NK[0]`; it is not a
commander choice.

Model tables in the same `app.js` are not one list. Examples: some GPT rows
`["low","medium","high"]`; some Claude rows include `xhigh`/`max`; some Gemini
rows include `"minimal"`. `"minimal"` is not in the official CLI flag table.

## Syntax And Lifetime

| Item | Exact 1.0.80 finding | Disposition |
| --- | --- | --- |
| Canonical flag | `--effort`; commander also names `--reasoning-effort` | `--reasoning-effort` is an upstream alias, not a second public option |
| Value syntax | commander `<level>`; official example `--effort=max` | either argv form is the same option |
| CLI choices | `h.reasoningEffortLevels()` | official docs name `low\|medium\|high\|xhigh\|max`; native enum was not executed |
| Default | omit the flag; server/settings/model default apply | omission is not a selected portable value |
| `session/new` | cwd and MCP servers only | no per-session argv substitute through this request |
| ACP config option | `reasoning_effort` after session start | not `session/new`; still model-gated |
| First prompt | inherits server-start `reasoningEffort` | then may be substituted when the model list is entitled |
| Later prompt | same process | same substitution risk; config option can also change it |
| Fresh replacement | new child would re-pass the same argv | Copilot may still substitute after model entitlement |
| Tool filters | separate `--available-tools` / `--excluded-tools` | not applicable |
| `--yolo` / `--allow-all` | separate dangerous permission flags | not applicable |

## Value And Profile Disposition

| Candidate | CLI parse | ACP startup store | Model-entitled apply | Contract 040 |
| --- | --- | --- | --- | --- |
| `low` | documented choice | stored if passed | kept only if the unknown current model lists it | withheld |
| `medium` | documented choice; also the `kN` default | stored if passed | same membership rule; default is not a selected value | withheld |
| `high` | documented choice | stored if passed | same membership rule | withheld |
| `xhigh` | documented choice | stored if passed | absent from some entitled lists | withheld |
| `max` | documented; CLI reference ties it to Anthropic depth | stored if passed | absent from some entitled lists | withheld |
| `minimal` | not in the official CLI table | not a documented server-start value | Gemini-only model table | not applicable |
| `none` | not a commander choice | rewritten to `NK[0]` if it appears internally | default substitution | not applicable |
| omitted | current Swallowtail argv | settings `effortLevel` or model default | not a portable selection | retain current absent path |

No candidate is deliver-now.

## No-Model-Route Contract 040 Decision

The official interface-level session setting is not sufficient.

Exact `1.0.80` applies startup effort only after the current model is known.
Unsupported explicit values are rejected or replaced with that model's default.
The entitled set differs by model. `copilot-cli.acp` does not select a model
and must not infer one from host state, catalogue presence, or later output.

Portable `ReasoningSelection` requires an exact mode qualified for the selected
model route. Mapping any of the five documented values here would need unknown
model capability, accept upstream substitution, or treat a default as a
selected value. All three are forbidden.

Dispatch, acceptance, and effectiveness stay distinct. This record does not
claim that Copilot honors a flag, only that Swallowtail cannot qualify one
without a model route.

The existing one-child/one-session topology could keep a spawn argv fixed
across first prompt, later prompts, and fresh replacement. That lifetime fit
does not rescue the Contract 040 gap.

No facade revision, behavior-segment split, or contract change is proposed.
The current `copilot --acp --stdio` boundary stands.

## Promotion

Research 188 promotes no deliver-now Copilot CLI ACP effort row.

Card 111 must not bind `ReasoningSelection`. Card 112 has no dispatch to prove.
A later lane may reopen this family only with an exact selected-model route or
an upstream interface that accepts one value without model entitlement.

Tool filters, TCP, permissions, login, BYOK, model selection, usage, and
session load/resume remain out of scope.
