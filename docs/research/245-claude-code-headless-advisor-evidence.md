# 245 Claude Code Headless Advisor Evidence

Status: promoted
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.088 / 248

## Question

Which exact qualified `claude-code.headless` version, access, advisor-model,
and lifecycle rows, if any, can bind `--advisor` with closed membership,
selection, extra-request/spend, application, terminal, and omission truth?

## Evidence Boundary

Research must use exact official package/native artifacts for the qualified
`2.1.220..=2.1.241` route window plus frozen official documentation. Mutable
current documentation is a lead and cannot backport support. No provider
prompt, login/account work, paid operation, host install/update, or ambient
configuration mutation is authorized.

The record must freeze support membership, parser domain, precedence, model
resolution, access/entitlement, application, extra request/spend, terminal
shape, cleanup, omission, and the separation of requested, parsed, resolved,
dispatched, accepted, effective, returned, observed, and billed truth. It must
classify every candidate row as deliver now, evidence-gated, intentionally
withheld, or not applicable.

## Promotion Gate

Promote a non-empty row only when model membership, access/entitlement,
operation-private precedence, application, extra request/spend, result,
cleanup, and omission close. Otherwise promote an honest empty set with named
gates.

## Method

Evidence was collected on 2026-08-28. No Claude Code installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation was used. Host `claude` at `/Users/tom/.local/bin/claude`
(`2.1.250`) was not invoked, replaced, or updated. No ambient configuration
was written outside disposable probe homes.

Every published official npm package in the qualified window
`2.1.220..=2.1.241` and its `@anthropic-ai/claude-code-darwin-arm64` platform
package were downloaded to disposable `/tmp` paths. Native executables were
inspected through `--version`, `--help`, deterministic local argv probes, and
extracted implementation strings. All probes ran under `env -i` with only
`PATH` and a throwaway `HOME`, in a throwaway working directory.

Two prompt-free probe terminals were used:

- `claude --advisor <model> doctor` reaches full commander parsing and a local
  health action that sends no provider request. It separates *unknown option*
  from *accepted option* without validating advisor pairing or entitlement.
- `claude -p --output-format stream-json --advisor <model>` with `stdin`
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
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--advisor <model>` wording, precedence over `advisorModel`, Fable access note | 2026-08-28 | SHA-256 `cce20fe024733eaa9e4f49f6406d332c90d7958dd9866809f477494a5c10d1f1` |
| [Advisor tool](https://code.claude.com/docs/en/advisor) | enable paths, pairing table, spend, feature-flag gate, silent non-attachment, disable env | 2026-08-28 | SHA-256 `b56b50c009df03ee6933ad0e5f857d8b32e607dbbf07966f5d5f92668f9e7cba` |
| [Claude Code environment variables](https://code.claude.com/docs/en/env-vars) | `CLAUDE_CODE_DISABLE_ADVISOR_TOOL` accepts flag with no effect | 2026-08-28 | SHA-256 `73889e9363feb7b258e819168cacd4c549ef50a23e609385bd7f7e6e02f761ff` |
| [Model configuration](https://code.claude.com/docs/en/model-config) | advisor is distinct from main model / subagent selection | 2026-08-28 | SHA-256 `c30b6a0ae01dc590609d7f2dae93b49f9d72613defc18b297ac2135f13aed29a` |
| `@anthropic-ai/claude-code@2.1.220` wrapper tarball | window baseline identity | 2026-08-28 | SHA-256 `df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3` |
| `@anthropic-ai/claude-code@2.1.241` wrapper tarball | window ceiling identity; matches Research 202 | 2026-08-28 | SHA-256 `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6` |
| `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241` | exact parser, strings, and membership evidence | 2026-08-28 | per-version digests in the fixture below |
| `claude-code-2.1.241/headless-advisor.json` | sanitized deterministic specimen corpus | 2026-08-28 | asserted in this record |

Current documentation is a lead only. It states that `--advisor` enables the
server-side advisor tool for one session, takes precedence over
`advisorModel`, is omitted from `claude --help`, may exit on unsupported
pairing/allowlist/Fable consent, may silently leave the advisor unattached on
capability mismatch, and that consultations consume extra tokens against
subscription usage limits or API rates. None of that closes entitlement,
effective attachment, or billed spend on the selected local-subscription route
without live account or provider work. Every claim below rests on the exact
artifacts.

The wrapper npm packages remain installer wrappers; all implementation evidence
is in the platform native executable. Endpoint platform and help digests match
Research 226 / 241.

## Help And Option Surface

`--advisor <model>` is present in every published package binary in
`2.1.220..=2.1.241` and is **not** advertised in `--help` at any probed
version. That matches current docs ("doesn't list `--advisor` in
`claude --help`"). Official `--help` digests reproduce Research 202 / 226 at
the endpoints: `fcd5b455…` at `2.1.220` and `71ad650f…` at `2.1.241`.

Exact option wording extracted from package strings at both endpoints:

```text
--advisor <model>
Enable the server-side advisor tool with the specified model (alias or full ID).
```

Missing value rejects before doctor/print work:

```text
error: option '--advisor <model>' argument missing
```

`doctor --advisor opus` (option after the doctor subcommand) is an unknown
option. Global `--advisor <model> doctor` is accepted.

## Parser Domain

Doctor probes at `2.1.220` and `2.1.241` accept arbitrary model tokens,
including docs-invalid advisors:

| Input | Disposition |
| --- | --- |
| `opus`, `sonnet`, `fable`, `claude-opus-5` | accepted; doctor exit 0 |
| `haiku`, `not-a-model` | accepted at parse; doctor exit 0 |
| missing value | rejected; exit 1 |

Commander parsing does not close membership, pairing, allowlist, feature-flag,
or Fable-consent entitlement. Those gates live after parse.

Empty-prompt print mode with `--advisor opus` exits `1` with
`Error: Input must be provided either through stdin or as a prompt argument
when using --print` and sends no provider request. The same holds when the
selected headless Plan tools and `--no-session-persistence` flags are present.

## Precedence And Ambient Authority

Docs and package strings agree on three enable paths: `/advisor`,
`advisorModel` settings, and `--advisor`. The flag takes precedence over
`advisorModel` for that session and does not persist the setting.

The selected Swallowtail headless argv already passes
`--setting-sources user,project,local`. Ambient `advisorModel` can therefore
enable the advisor without `--advisor`. `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`
accepts the flag with no effect. Docs also require feature-flag fetching;
variables that disable flag fetching keep the advisor off.

Caller argv alone is therefore not operation-private enablement on this route.
Omission of `--advisor` preserves exact prior argv and does **not** claim
advisor-off.

## Model Resolution And Application

Advisor is a server-side tool (`advisor-tool-2026-03-01` /
`advisor_20260301`), not main-model selection and not a subagent. Package
strings and docs require the advisor to be at least as capable as the main
model; less-capable or unrecognized pairings may leave the advisor unattached
while the session continues. Alias defaults (`fable`, `opus`, `sonnet`) advance
with Claude Code releases, so alias membership is not a frozen closed set for
the window.

Claude decides when to consult. Each consultation is an extra provider request
at the advisor model's rates / subscription usage. Prompt-free probes cannot
observe effective attachment, returned advisor content, or billed spend.

## Access And Extra Spend

| Profile | Advisor semantics | Selected `claude-code.headless` |
| --- | --- | --- |
| Local subscription allowance / plan bars | docs: advisor usage counts toward plan usage limits | route access; live closure forbidden |
| Usage credits | Fable advisor may require consent and bill credits | not closed here |
| Provider-billed API USD | docs: pay advisor model rates | route rejects API-key billing |
| Feature flag / org allowlist | can keep advisor off or reject requested model | live-only |
| Local catalog-priced USD estimate | used by `--max-budget-usd`, not this flag | distinct; see Research 241 |

A non-empty deliver-now row would need closed entitlement and closed
extra-request/spend truth against the selected local-subscription profile.
Those remain live-only under this card's boundary.

## Current Driver Omission

`claude_code_command::arguments` does not emit `--advisor`. Omission preserves
the exact prior argv. No advisor-off claim follows from omission because
ambient settings remain in `--setting-sources`.

## Truth Separation

| Layer | What this lane proves | What remains unproved / rejected |
| --- | --- | --- |
| requested | caller could append `--advisor <model>` | closed alias/ID membership |
| parsed | required string; missing value rejects; no pairing at parse | entitlement / pairing at doctor |
| resolved | aliases and pairing exist in package/docs | frozen alias→ID map for the window |
| dispatched | print mode accepts the flag before empty-prompt reject | live request construction |
| accepted | feature flags, allowlists, consent, pairing | live acceptance under subscription |
| effective | tool is server-side advisor, not main model/subagent | prompt-free attachment observation |
| returned | result block names exist in package strings | live decoder mapping |
| observed | doctor/help/strings/print-empty only | live consultation |
| billed | docs separate subscription usage, credits, API rates | live spend under subscription |

## Claim Strength

| Claim | Strength at the exact evidence boundary |
| --- | --- |
| package declares hidden `--advisor <model>` at every published `2.1.220..=2.1.241` point | observed |
| `--help` omits the flag | observed; matches docs |
| parser accepts model tokens without pairing/entitlement checks | observed at endpoints |
| argv flag documented over `advisorModel` | docs + strings |
| ambient `advisorModel` / setting-sources can enable without flag | docs + current headless argv |
| disable env nullifies flag effect | docs + env-vars + package strings |
| advisor is distinct from main model and subagent | docs + model-config + package strings |
| extra consultation spend closes on selected subscription route | not observed; live-only |
| selected subscription route can bind `--advisor` honestly | not closed |
| current headless argv already passes the flag | not observed; omission preserved |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `--advisor <alias\|id>` on local-subscription `claude-code.headless` | hidden option membership across window; parse-only doctor acceptance; ambient/disable/feature-flag gates; live spend/entitlement open | empty; entitlement, operation-private precedence, application, and extra-spend do not close |
| omission | current argv unchanged; ambient settings can still enable | unchanged; no advisor-off claim |
| missing value | parser rejection | rejected as a binding value |
| doctor-accepted `haiku` / unknown tokens as closed advisors | parse-only | rejected |
| flatten into main-model or subagent vocabulary | contradicted by docs/package | rejected |
| portable advisor capability | product-specific Claude server tool | rejected |
| live consultation / spend / terminal proof | requires provider turns or account inspection | withheld |

Deliver-now rows: **none**.

## Decision

Card 248 is complete as an evidence stop with an honest empty set. Exact
packages declare a hidden `--advisor <model>` across every published
`2.1.220..=2.1.241` point and accept it in print mode before empty-prompt
rejection. That does not close model membership, subscription entitlement,
operation-private precedence, effective attachment, extra request/spend,
terminal mapping, or omission-as-off. Advisor remains distinct from main-model
selection, subagents, and portable vocabulary. No production binding, guide
capability claim, matrix row, or shared closeout follows from this record. The
current headless route and omission behavior stay unchanged.

## Sources

- npm `@anthropic-ai/claude-code` `2.1.220` and `2.1.241`
- official `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241`
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Advisor tool](https://code.claude.com/docs/en/advisor)
- [Claude Code environment variables](https://code.claude.com/docs/en/env-vars)
- [Model configuration](https://code.claude.com/docs/en/model-config)
- [Research 202 Claude Code 2.1.241 Identity](./202-claude-code-2-1-241-identity.md)
- [Research 226 Claude Code Headless Maximum Turns](./226-claude-code-headless-maximum-turns-evidence.md)
- [Research 233 Claude Code Headless Fast Mode](./233-claude-code-headless-fast-mode-evidence.md)
- [Research 237 Claude Code Headless Autocompaction](./237-claude-code-headless-autocompaction-evidence.md)
- [Research 241 Claude Code Headless Spend Cap](./241-claude-code-headless-spend-cap-evidence.md)
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-advisor.json`
