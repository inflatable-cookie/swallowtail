# 241 Claude Code Headless Spend-Cap Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.087 / 244

## Question

Which exact qualified `claude-code.headless` version, access, positive value,
unit, and lifecycle rows, if any, can bind caller-selected spend capping with
exact enforcement, terminal, billing, and omission truth?

## Evidence Boundary

Research must use exact official package/native artifacts for the qualified
`2.1.220..=2.1.241` route window plus frozen official documentation. Mutable
current documentation is a lead and cannot backport support. No provider
prompt, login/account work, paid operation, host install/update, or ambient
configuration mutation is authorized.

The record must freeze support membership, parser domain, precedence, accrued-
cost source, loop enforcement, limit-reached result shape, access/billing
compatibility with the selected local-subscription route, omission, and the
separation of requested, parsed, applied, accrued, provider-billed, enforced,
returned, and observed truth. It must classify every candidate row as deliver
now, evidence-gated, intentionally withheld, or not applicable.

## Promotion Gate

Promote a non-empty row only when exact official/package evidence closes the
selected access and billing profile, units, parser domain, operation-private
precedence, enforcement point, accrued-cost source, limit-reached result/exit,
cleanup, and omission. Otherwise promote an honest empty set with named gates.

## Method

Evidence was collected on 2026-08-27. No Claude Code installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation was used. Host `claude` at `/Users/tom/.local/bin/claude` was not
invoked, replaced, or updated. No ambient configuration was written.

Every published official npm package in the qualified window
`2.1.220..=2.1.241` and its `@anthropic-ai/claude-code-darwin-arm64` platform
package were downloaded to disposable `/tmp` paths. Native executables were
inspected through `--version`, `--help`, deterministic local argv probes, and
extracted implementation strings. All probes ran under `env -i` with only
`PATH` and a throwaway `HOME`, in a throwaway working directory.

Two prompt-free probe terminals were used:

- `claude [flags] doctor` reaches full commander parsing and a local health
  action that sends no provider request. It separates *unknown option* from
  *accepted option* and exercises the `--max-budget-usd` value parser.
- `claude -p --output-format stream-json --max-budget-usd 1` with `stdin`
  closed rejects the empty prompt before authentication. It confirms print-mode
  acceptance of the flag without any API call.

The route under study is `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis `claude-code.headless-stream-json`,
qualified window `2.1.220..=2.1.241`, behavior
`claude-code.headless.stream-json.v1`. This record does not amend
`claude-code.response-only` or `claude-agent.acp`.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--max-budget-usd` print-mode API-call wording, subagent count, v2.1.217+ enforcement | 2026-08-27 | SHA-256 `9b6f3fa1401983c9cc5512545762d64025fc8f0e17544302dd36053ac3329050` |
| [Claude agent loop](https://code.claude.com/docs/en/agent-sdk/agent-loop) | `max_budget_usd` / `maxBudgetUsd`, `error_max_budget_usd`, `total_cost_usd` | 2026-08-27 | SHA-256 `e1941286969ebaee4ca5129c23cb431dbc4eb7eea66eea6aa7cdd77daec97f03` |
| [Claude Code environment variables](https://code.claude.com/docs/en/env-vars) | no `--max-budget-usd` env equivalent observed | 2026-08-27 | SHA-256 `a6e6fb5f8c5dc25bea4a6e2788ab8774b02a19b0962d823d39e0ffde50538651` |
| [Manage costs effectively](https://docs.anthropic.com/en/docs/claude-code/costs) | subscriber session-cost figure is not billing-relevant | 2026-08-27 | SHA-256 `3f78b3debd355bf855813f1e93bcba26b6ccbd4daa02a26a6ea5e6423298bdb1` |
| `@anthropic-ai/claude-code@2.1.220` wrapper tarball | window baseline identity | 2026-08-27 | SHA-256 `df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3` |
| `@anthropic-ai/claude-code@2.1.241` wrapper tarball | window ceiling identity; matches Research 202 | 2026-08-27 | SHA-256 `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6` |
| `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241` | exact parser, ledger, guard, result, and access evidence | 2026-08-27 | per-version digests in the fixture below |
| `claude-code-2.1.241/headless-spend-cap.json` | sanitized deterministic specimen corpus | 2026-08-27 | asserted in this record |

Current documentation is a lead only. It states that `--max-budget-usd` caps
dollar spend on API calls in print mode, that subagent spend counts, that
cap-enforcement behaviors require Claude Code v2.1.217+, and that
`error_max_budget_usd` is the limit-reached result subtype. Costs docs state
that Max/Pro subscribers have usage included in the subscription and that the
session cost figure is not relevant for billing. None of that backports a
subscription-compatible billed-USD claim onto the selected route. Every claim
below rests on the exact artifacts.

The wrapper npm packages remain installer wrappers; all implementation evidence
is in the platform native executable. Endpoint digests match Research 226.

## Help And Option Surface

`--max-budget-usd <amount>` is advertised in `--help` at every probed version.
Official `--help` digests reproduce Research 202 / 226 at the endpoints:
`fcd5b45507c7c602d54d85a300eab288a8a3c6770c6def696ca19a3100725de4` at
`2.1.220` and `71ad650f59e08ae40ede14c534db4f49d8590ee5a4f92f6da2882d3a5560fea6`
at `2.1.241`. Mid-window help digests differ for `2.1.221` and
`2.1.222`/`2.1.223`, then stabilize on the `2.1.241` digest from `2.1.224`
through `2.1.241`. The budget flag text is present in every case.

Exact option declaration at `2.1.241` (same parser message and help wording at
every probed version):

```js
.addOption(new wp("--max-budget-usd <amount>",
  "Maximum dollar amount to spend on API calls (only works with --print)")
  .argParser((c)=>{
    let u=Number(c);
    if(isNaN(u)||u<=0)
      throw new o6t("--max-budget-usd must be a positive number greater than 0");
    return u;
  }))
```

No `CLAUDE_CODE_MAX_BUDGET`, `MAX_BUDGET_USD`, or settings key for this cap was
observed in package strings or env-var docs. Precedence is argv-only. Repeated
`--max-budget-usd` values last-win at the commander layer.

## Parser Domain

Doctor probes are identical at `2.1.220` and `2.1.241`:

| Input | Disposition |
| --- | --- |
| `1`, `5.00`, `+3`, `" 2 "` | accepted; doctor exit 0 |
| `0`, `-1` | rejected; exit 1; must be positive greater than 0 |
| `abc`, `""` | rejected; exit 1 |
| missing value (`--max-budget-usd doctor`) | next token consumed; rejected |
| `0x1`, `1e-1`, `Infinity` | accepted by `Number` coercion |

Unlike `--max-turns`, zero and negatives are rejected at parse time. Fractional
and exotic `Number` forms still pass. A Swallowtail binding would still need a
closed positive domain; `Infinity` / hex / exponent are not deliver-now values.

Empty-prompt print mode with `--max-budget-usd 1` exits `1` with
`Error: Input must be provided either through stdin or as a prompt argument
when using --print` and sends no provider request.

## Accrued-Cost Source

Enforcement does not read subscription allowance, usage-credit balance, or a
provider invoice. It compares the caller cap to a local session ledger:

```js
function /*guard*/(e){ return e!==void 0 && /*totalCostUSD*/() >= e }
// totalCostUSD reads costLedger.totalCostUSD()
// recordCost(amount, modelUsage, model) accumulates amount into the ledger
```

Cost amounts are computed from baked model-catalog rates and token usage:

```js
function /*xma*/(rates, usage){
  let tokens =
    usage.input_tokens/1e6*rates.inputTokens
    + usage.output_tokens/1e6*rates.outputTokens
    + (usage.cache_read_input_tokens??0)/1e6*rates.promptCacheReadTokens
    + /*cache-write terms*/;
  let web = (usage.server_tool_use?.web_search_requests??0)*rates.webSearchRequests;
  return tokens * /*us-geo multiplier or 1*/ + web;
}
```

At `2.1.241`, `inference_geo === "us"` multiplies the token portion by `1.1`.
That matches the changelog lead about US-only-inference premium in cost
estimates. The meter is a local API-catalog estimate, optionally geo-adjusted.

Official costs docs separate that session figure from subscriber billing:
Max/Pro usage is included in the subscription; the session cost figure is not
relevant for billing. The selected Swallowtail route is local-subscription and
rejects API-key billing as a route choice.

## Enforcement And Terminal Shape

When the guard trips, native code emits result subtype `error_max_budget_usd`
with errors `Reached maximum budget ($<cap>)`, and print-mode text
`Error: Exceeded USD budget (<cap>)`. Subagent launch after the cap fails with
`Budget limit reached ($spent spent of the $cap maximum)`. Docs and strings
agree that subagent spend counts toward the cap.

Live limit-reached exit was not exercised. Hitting the ledger requires provider
turns that accrue estimated cost. That would be paid or credentialed work this
card forbids. Source proves the guard and result subtype; Research 226 already
maps sibling `error_max_turns` through the same print error path to process
exit 1 and Swallowtail `provider_failed`. No distinct budget diagnostic exists
in the current decoder.

## Access Compatibility With The Selected Route

| Profile | Cap semantics | Selected `claude-code.headless` |
| --- | --- | --- |
| Local subscription allowance / plan bars | not what `--max-budget-usd` meters | route access |
| Usage credits | separate product surface; not this flag | not closed here |
| Provider-billed API USD | docs frame the flag as API-call spend; ledger is still a local estimate | route explicitly rejects API-key billing |
| Local catalog-priced USD estimate | exact package meter | not a subscription spend claim |

A non-empty deliver-now row would require closing units against the selected
access/billing profile. The selected profile is local subscription. The flag
meters local API-catalog estimates framed as API-call dollars. Those are not
the same unit. Binding the flag on this route would either invent an API-key
billing profile or equate subscription spend with estimated API USD. Both are
forbidden by the card and triage.

## Current Driver Omission

`claude_code_command::arguments` does not emit `--max-budget-usd`. Omission
preserves the exact prior argv. No spend-cap claim follows from omission.

## Truth Separation

| Layer | What this lane proves | What remains unproved / rejected |
| --- | --- | --- |
| requested | caller could append `--max-budget-usd <amount>` | subscription-compatible billed unit |
| parsed | positive `Number` domain; rejects `<=0` and NaN | closed Swallowtail domain excluding Infinity/hex/exponent |
| applied | argv-only; no env/settings competitor observed | — |
| accrued | local catalog-priced ledger | provider invoice / subscription allowance |
| provider-billed | not observed; docs separate subscriber session cost from billing | requires API-key or Console billing route |
| enforced | source guard + `error_max_budget_usd` shape | live limit-reached without paid turns |
| returned | subtype and error strings in package | distinct Swallowtail diagnostic |
| observed | doctor/help/parser only | live accrued cost under subscription |

## Claim Strength

| Claim | Strength at the exact evidence boundary |
| --- | --- |
| help advertises `--max-budget-usd` for API calls in print mode | observed at every published `2.1.220..=2.1.241` point |
| parser rejects non-positive values before doctor/print work | observed at endpoints |
| no env/settings override of the flag | observed; argv-only |
| accrued cost is local catalog pricing | observed in package source |
| accrued cost is subscription allowance | not observed; contradicted by costs docs |
| accrued cost is provider-billed API USD | not observed; ledger is local estimate |
| selected subscription route can bind the cap honestly | not closed |
| live `error_max_budget_usd` exit under subscription | not observed; would need provider turns |
| current headless argv already passes the flag | not observed; omission preserved |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `--max-budget-usd <positive>` on local-subscription `claude-code.headless` | parser, argv-only precedence, catalog ledger, `error_max_budget_usd` | empty; selected access/billing profile and units do not close |
| omission | current argv unchanged; no spend-cap claim | unchanged; remains exact |
| zero / negative / non-numeric | parser rejection | rejected |
| `Infinity` / hex / exponent | parser accepts via `Number` | rejected as closed binding domain |
| equate estimate with subscription allowance or billed API USD | contradicted by package + costs docs | rejected |
| API-key route creation to host the flag | out of scope for this lane | evidence-gated elsewhere |
| portable spend-cap capability | product-specific Claude print flag | rejected |
| live limit-reached proof | requires provider turns | withheld |

Deliver-now rows: **none**.

## Decision

Card 244 is complete as an evidence stop with an honest empty set. Exact
packages advertise and parse `--max-budget-usd`, enforce against a local
catalog-priced USD ledger, and emit `error_max_budget_usd`. That meter does not
close against the selected local-subscription access and billing profile.
Subscription allowance, usage credits, provider-billed API USD, and local
estimates remain distinct. No production binding, guide capability claim,
matrix row, or shared closeout follows from this record. The current headless
route and omission behavior stay unchanged.

## Sources

- npm `@anthropic-ai/claude-code` `2.1.220` and `2.1.241`
- official `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241`
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Claude agent loop](https://code.claude.com/docs/en/agent-sdk/agent-loop)
- [Claude Code environment variables](https://code.claude.com/docs/en/env-vars)
- [Manage costs effectively](https://docs.anthropic.com/en/docs/claude-code/costs)
- [Research 202 Claude Code 2.1.241 Identity](./202-claude-code-2-1-241-identity.md)
- [Research 226 Claude Code Headless Maximum Turns](./226-claude-code-headless-maximum-turns-evidence.md)
- [Research 233 Claude Code Headless Fast Mode](./233-claude-code-headless-fast-mode-evidence.md)
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-spend-cap.json`
