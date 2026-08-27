# 233 Claude Code Headless Fast-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.083 / 232

## Question

Which exact qualified `claude-code.headless` version, model, subscription,
value, and operation rows can bind caller-selected Fast mode without ambient
settings authority, mutable credit entitlement, or unconfirmed activation?

## Evidence Boundary

Research must use exact official package/native artifacts for the qualified
`2.1.220..=2.1.241` route window plus frozen official documentation. Mutable
current documentation is a lead and cannot backport support. No provider
prompt, login/account work, paid operation, host install/update, or ambient
configuration mutation is authorized.

The record must freeze support membership, settings schema, print-mode
activation, stream observability, omission, access and billing leads, and the
separation of requested, settings-encoded, provider-accepted, effective,
returned, billed, and latency-observed truth. It must classify every candidate
row as deliver now, evidence-gated, intentionally withheld, or not applicable.

## Method

Evidence was collected on 2026-08-27. No Claude Code installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation was used. Host `claude` was not on `PATH` and was not installed,
replaced, or updated. No ambient configuration was written except disposable
user settings files inside throwaway `HOME` directories for precedence probes.

Every probe used official npm packages downloaded to disposable `/tmp` paths.
Native executables were inspected through `--help`, `--version`, deterministic
local argv probes, extracted implementation strings, and prompt-free print-mode
terminals that reject empty input before any authenticated provider work.

Two prompt-free probe terminals were used:

- `claude [flags] doctor` reaches full commander parsing and a local health
  action that sends no provider request. It separates invalid `--settings`
  JSON from schema-valid settings and exercises the `fastMode` boolean parser.
- `claude [flags] -p <prompt>` with a one-token prompt reaches the main print
  action, emits stream-json init/result frames, and exposes
  `fast_mode_state` / `fast_mode_disabled_reason` before authentication fails
  closed with `Not logged in · Please run /login`.

The route under study is `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis `claude-code.headless-stream-json`,
qualified window `2.1.220..=2.1.241`, behavior
`claude-code.headless.stream-json.v1`. This record does not amend
`claude-code.response-only` or `claude-agent.acp`.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [Claude Code Fast mode](https://code.claude.com/docs/en/fast-mode) | print-mode activation, model/access/billing requirements, research preview status | 2026-08-27 | SHA-256 `ce14865e78f2c054948927ad8626b86a116d245f0404c5498010f8c8735b32bc` |
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--settings` option description | 2026-08-27 | SHA-256 `5e9c7e929f5189593ecccbd7a9dd62a903ac7e21b9e2add5c574445bd9740a9d` |
| [Claude Code environment variables](https://code.claude.com/docs/en/env-vars) | `CLAUDE_CODE_DISABLE_FAST_MODE` and network/org skip variables | 2026-08-27 | SHA-256 `a542edc27c2b0e91bb770d5999f4863a70ee77743e4a46e5339bbf095c34db2f` |
| `@anthropic-ai/claude-code@2.1.220` wrapper tarball | window baseline identity | 2026-08-27 | SHA-256 `df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3` |
| `@anthropic-ai/claude-code@2.1.241` wrapper tarball | window ceiling identity; matches Research 202 | 2026-08-27 | SHA-256 `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6` |
| `@anthropic-ai/claude-code-darwin-arm64@2.1.220..=2.1.241` | exact settings schema, print-mode activation, stream fields, and disable reasons | 2026-08-27 | endpoint digests in the support table below |
| `claude-code-2.1.241/headless-fast-mode.json` | sanitized deterministic specimen corpus | 2026-08-27 | asserted in this record |

Current documentation is a lead only. It states that Fast mode is a research
preview, uses Opus 5 or Opus 4.8, requires Anthropic API or Claude
subscription access with usage credits, is unavailable on Bedrock/Vertex/Foundry,
activates in print mode only through launch-time
`--settings '{"fastMode": true}'`, and exposes interactive `/fast` toggles.
None of that backports effective activation or billing truth onto the qualified
window without exact artifacts.

## Help And Settings Surface

`--settings <file-or-json>` is advertised in `--help` at both probed endpoints.
`fastMode` itself is not a standalone CLI flag and does not appear in help.
Official `--help` digests reproduce Research 202 exactly at `2.1.220` and
`2.1.241`.

Invalid JSON to `--settings` exits `1` with `Error: Invalid JSON provided to
--settings`. Schema-invalid `fastMode` values are reported as invalid settings
at doctor time:

| Input | Disposition |
| --- | --- |
| `{"fastMode": true}` | accepted |
| `{"fastMode": false}` | accepted |
| `{"fastModePerSessionOptIn": true}` | accepted |
| `{"fastMode": "true"}` | rejected; `Expected boolean, but received string` |
| `{"fastMode": 1}` | rejected; `Expected boolean, but received number` |
| `{"fastMode": null}` | rejected; `Expected boolean, but received null` |
| `{fastMode: true}` | rejected; invalid JSON |

Schema warnings do not block `doctor` from completing locally. They were not
observed to abort print-mode startup before the empty-prompt or auth-failure
terminals used in this lane.

## Print-Mode Activation Seam

Official docs and exact runtime probes agree on the print-mode rule:

- Without launch-time `--settings '{"fastMode": true}'`, stream init reports
  `fast_mode_state: "off"` and `fast_mode_disabled_reason:
  "sdk_opt_in_required"`.
- With launch-time `--settings '{"fastMode": true}'`, stream init reports
  `fast_mode_disabled_reason: "preference"` instead, but still
  `fast_mode_state: "off"` under unauthenticated probes.
- With launch-time `--settings '{"fastMode": false}'`, the disabled reason
  returns to `sdk_opt_in_required`.
- A disposable user settings file containing `"fastMode": true` does **not**
  satisfy print-mode activation when argv omits `--settings`; the disabled
  reason remains `sdk_opt_in_required`.

That proves a caller-bound **settings-encoding** seam exists, but not effective
Fast mode. Encoded preference and effective mode diverge before authentication.

`CLAUDE_CODE_DISABLE_FAST_MODE=1` forces `fast_mode_disabled_reason:
"disabled_by_env"` even when argv passes `"fastMode": true`.

## Stream Observability

At `2.1.220` and `2.1.241`, unauthenticated print-mode probes emit:

- init system frame fields `fast_mode_state` and `fast_mode_disabled_reason`
- result frame fields `fast_mode_state`, `fast_mode_disabled_reason`, and
  `usage.speed: "standard"`
- no `Fast mode ON` / `Fast mode OFF` confirmation text in stream-json output

Interactive confirmation strings exist in the exact package (`Fast mode ON`,
`Fast mode OFF`) but are not the headless route's confirmation seam. Swallowtail
must not treat `/fast` or marketing names as selected-state confirmation.

Extracted implementation literals for disabled reasons include at least:
`sdk_opt_in_required`, `preference`, `disabled_by_env`, `disabled_by_config`,
`disabled_by_default`, `disabled_by_org`, `disabled_by_setting`, and
`disabled_by_flag`.

## Model And Access Probes

Without authentication, `--settings '{"fastMode": true}'` reaches print-mode
init for every probed model id:

| Model | Init `fast_mode_state` | Init `fast_mode_disabled_reason` |
| --- | --- | --- |
| `claude-opus-5` | `off` | `preference` |
| `claude-opus-4-8` | `off` | `preference` |
| `claude-opus-4-7` | `off` | `preference` |
| `claude-sonnet-4` | `off` | `preference` |
| `claude-haiku-4` | `off` | `preference` |

No pre-effect rejection for unsupported models was observed. Official docs say
Fast mode is Opus 5 / Opus 4.8 only, but exact package behavior under
unauthenticated print mode does not fail closed on Sonnet or Haiku before init.
Model eligibility therefore cannot be closed without account-backed checks.

Access, usage-credit, organization-enablement, allowlist, Bedrock/Vertex
exclusion, and Console provisioning requirements documented on the Fast mode
page are mutable account and org facts. This lane did not inspect them and does
not freeze them as static capability.

## Setting-Sources And Process Privacy

Research 226 freezes the current headless argv, including
`--setting-sources user,project,local`. That composition remains unchanged.

Precedence probes show:

- argv `--settings '{"fastMode": true}'` changes the disabled reason from
  `sdk_opt_in_required` to `preference` even when `--setting-sources ""`
- ambient user `fastMode` alone never activates print mode
- argv `--settings '{"fastMode": true}'` still wins over ambient user
  `fastMode: false` by the same disabled-reason signal

Those facts prove a bounded activation-encoding seam, but not full process-
private precedence over ambient project/user/managed settings. Managed org
settings, other Fast-related keys such as `fastModePerSessionOptIn`, and account
backed availability checks can still influence effective mode once authenticated.
Swallowtail did not read or mutate ambient configuration to prove isolation for
every settings source.

## Omission

Omission emits no `--settings` argument and leaves the exact current command
byte-identical to Research 226:

```text
claude -p --input-format text --output-format stream-json --verbose
  --no-session-persistence --model <selected> [--effort <selected>]
  [--max-turns <selected>] --permission-mode plan --tools Read,Glob,Grep
  --setting-sources user,project,local --mcp-config {"mcpServers":{}}
  --strict-mcp-config
```

Omission makes no Fast, latency, credit, cost, or default-speed claim. Stream
init under omission reports `fast_mode_disabled_reason: "sdk_opt_in_required"`.

## Truth Separation

| Layer | What this lane proves | What remains unproved |
| --- | --- | --- |
| requested | caller could add `--settings '{"fastMode": true}'`; current route omits it | authenticated acceptance of a future binding |
| settings-encoded | boolean schema and print-mode opt-in encoding | org-managed overrides after auth |
| provider-accepted | not observed | requires login and successful request |
| effective | always `off` in unauthenticated probes | requires account-eligible successful request |
| returned | `usage.speed: "standard"` while off | `fast`/`priority` returned speed while on |
| billed | not observed | usage credits and Console Speed grouping require account work |
| latency-observed | not observed | docs claim up to 2.5x faster; no probe sent |

Claude Fast mode is not effort, Codex service tier, Cursor bracket syntax, or
Anthropic Messages `output_config.effort`.

## Claim Strength

| Claim | Strength at the exact evidence boundary |
| --- | --- |
| help advertises `fastMode` | not observed; only `--settings` is advertised |
| `--settings` accepts boolean `fastMode` | observed at `2.1.220` and `2.1.241` |
| print mode requires launch-time `--settings` opt-in | observed; ambient user settings alone insufficient |
| effective Fast mode can be confirmed without auth | not observed; init stays `off` |
| unsupported models reject before init | not observed for Sonnet or Haiku |
| usage credits / org enablement are static capability | not observed; mutable account facts |
| process-private precedence over all ambient/managed settings | not proved |
| current headless argv passes Fast settings | not observed; omission preserved |
| `/fast` confirms selected state in headless route | rejected; interactive only |
| latency or billing truth | unproved without provider/account work |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `--settings '{"fastMode": true}'` as print-mode encoding seam | schema, opt-in reason transition, stream fields | withheld; effective activation, model/access closure, and billing unproved |
| omission | current argv unchanged; no Fast claim | unchanged; remains exact |
| `--settings '{"fastMode": false}'` | restores `sdk_opt_in_required` | rejection/omission evidence only |
| non-boolean `fastMode` values | schema rejection | rejected |
| unsupported-model pre-effect rejection | not observed without auth | rejected as static adapter gate |
| usage-credit or org-enablement binding | mutable account state | rejected as static capability |
| portable speed / service-tier control | product-specific Fast mode | rejected |
| `CLAUDE_CODE_DISABLE_FAST_MODE` scrub or host mutation | not required for evidence | out of scope |
| `claude-code.response-only`, `claude-agent.acp` | separate routes | not applicable |

Deliver-now rows: **none**.

## Decision

Card 232 is complete as an evidence stop with an honest empty set. The exact
package supports a launch-time `--settings` encoding seam and stream-json
observability, but effective Fast mode, eligible model membership, access and
credit entitlement, billing, returned speed, and latency cannot be closed
without login, account inspection, or successful provider work that this card
does not authorize.

No production binding, guide capability claim, matrix row, or shared closeout
follows from this record. The current headless route and omission behavior stay
unchanged.

## Sources

- npm `@anthropic-ai/claude-code` `2.1.220` and `2.1.241`
- official `@anthropic-ai/claude-code-darwin-arm64` `2.1.220` and `2.1.241`
- [Claude Code Fast mode](https://code.claude.com/docs/en/fast-mode)
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Claude Code environment variables](https://code.claude.com/docs/en/env-vars)
- [Research 202 Claude Code 2.1.241 Identity](./202-claude-code-2-1-241-identity.md)
- [Research 226 Claude Code Headless Maximum Turns](./226-claude-code-headless-maximum-turns-evidence.md)
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-fast-mode.json`
