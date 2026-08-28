# 249 Claude Code Headless Permission-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.089 / 252

## Question

Which exact qualified `claude-code.headless` version, permission mode,
resource, tool, and lifecycle rows, if any, can safely replace fixed Plan
without widening authority or claiming host containment?

## Evidence Boundary

Use exact official package/native artifacts for every published qualified
`2.1.220..=2.1.241` point plus frozen official documentation. No provider
prompt, login/account work, paid operation, install/update, or ambient host
mutation is authorized. `bypassPermissions` is excluded.

## Promotion Gate

Promote a non-empty row only when membership, precedence, application,
resource/tool authority, terminal behavior, cleanup, and omission close before
provider effects. Otherwise promote an honest empty set with named gates.

## Method

Evidence was collected on 2026-08-28. No Claude Code installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation was used. Host `claude` was not invoked, replaced, or updated. No
ambient configuration was written outside disposable probe homes.

Every published official npm package in the qualified window
`2.1.220..=2.1.241` and its `@anthropic-ai/claude-code-darwin-arm64` platform
package were downloaded to disposable `/tmp` paths. Native executables were
inspected through `--version`, `--help`, deterministic local argv probes, and
extracted implementation strings. All probes ran under `env -i` with only
`PATH` and a throwaway `HOME`, in a throwaway working directory.
`2.1.230` was never published; the semantic range still contains it but it
cannot be observed.

Membership used two prompt-free terminals:

- Invalid `--permission-mode <token>` rejects before any session work and
  prints the closed commander choice list.
- `claude -p --output-format stream-json --permission-mode <mode> --tools
  Read,Glob,Grep --no-session-persistence` with `stdin` closed rejects the
  empty prompt before authentication. That separates *accepted option* from
  *rejected option* without an API call.

`claude --permission-mode <accepted> doctor` can hang under an empty
disposable `HOME`, so doctor was not used as the acceptance terminal.

The route under study is `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis `claude-code.headless-stream-json`,
qualified window `2.1.220..=2.1.241`, behavior
`claude-code.headless.stream-json.v1`. This record does not amend
`claude-code.response-only` or `claude-agent.acp`.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [Permission modes](https://code.claude.com/docs/en/permission-modes) | mode table, Manual/`default`, Plan, auto, dontAsk, bypass exclusion, `-p` start rules | 2026-08-28 | SHA-256 `7962876f9c4697b315ea58f5104444f983cc318acc9423c1359b86644493a6fd` |
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--permission-mode`, `--dangerously-skip-permissions`, `-p` start note | 2026-08-28 | SHA-256 `cce20fe024733eaa9e4f49f6406d332c90d7958dd9866809f477494a5c10d1f1` |
| [Settings reference](https://code.claude.com/docs/en/settings-reference) | `permissions.defaultMode` lead | 2026-08-28 | SHA-256 `0d9d326a73501879c1ef7964ca64b2cd01ffe0920708b50d0db7c36dd50c8b2a` |
| `@anthropic-ai/claude-code@2.1.220` wrapper tarball | window baseline identity | 2026-08-28 | SHA-256 `df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3` |
| `@anthropic-ai/claude-code@2.1.241` wrapper tarball | window ceiling identity; matches Research 202 | 2026-08-28 | SHA-256 `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6` |
| `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241` | exact parser, strings, and membership evidence | 2026-08-28 | per-version digests in the fixture below |
| `claude-code-2.1.241/headless-permission-mode.json` | sanitized deterministic specimen corpus | 2026-08-28 | asserted in this record |

Current documentation is a lead only. It states Manual/`default` reads only,
`acceptEdits` auto-approves working-directory edits and common filesystem
commands, Plan researches without editing, `auto` runs with a classifier,
`dontAsk` auto-denies prompting paths, and `bypassPermissions` skips checks.
None of that alone closes a Plan replacement on the selected route. Every claim
below rests on the exact artifacts.

The wrapper npm packages remain installer wrappers; implementation evidence is
in the platform native executable. Endpoint binary digests match Research 226.

## Help And Option Surface

`--permission-mode <mode>` is advertised in `--help` at every published window
point. Exact help wording at both endpoints:

```text
--permission-mode <mode>              Permission mode to use for the session
                                      (choices: "acceptEdits", "auto",
                                      "bypassPermissions", "manual",
                                      "dontAsk", "plan")
```

Help digests reproduce Research 202 / 226 at the endpoints: `fcd5b455…` at
`2.1.220` and `71ad650f…` at `2.1.241`. Intermediate help digests vary only as
already recorded by sibling Claude evidence lanes; the permission-mode choice
list is identical at all 21 published points.

Invalid tokens reject before prompt effects:

```text
error: option '--permission-mode <mode>' argument 'not-a-mode' is invalid.
Allowed choices are acceptEdits, auto, bypassPermissions, manual, dontAsk, plan.
```

A missing value consumes the next argv token and rejects the same way.
`Default`, `ACCEPT_EDITS`, `dont_ask`, `dont-ask`, `yolo`, and `unknown` all
reject.

## Parser Domain

Empty-prompt print probes at every published point accept:

| Input | Disposition |
| --- | --- |
| `plan`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions` | accepted; empty-prompt exit 1 |
| `default` | accepted though omitted from help choices; empty-prompt exit 1 |
| `manual` | accepted; package strings: `'manual' is accepted as an alias for 'default'` |
| case / snake / kebab variants above | rejected; exit 1 |
| missing value | rejected; exit 1 |

Commander parsing does not close effective tool or write authority. Those gates
live after parse and depend on mode semantics, ambient allow rules, and
optional classifier or prompt surfaces.

## Authority Labels From The Package

UI/settings label strings at `2.1.220` and `2.1.241`:

| Mode | Package label |
| --- | --- |
| `default` | `default (ask each time)` |
| `acceptEdits` | `accept edits (auto-approve file edits and common file commands)` |
| `auto` | `auto (no routine prompts; a reviewer model screens actions)` |
| `dontAsk` | `don't ask (auto-deny anything that would prompt)` |
| `plan` | `plan mode (research and propose changes without making them)` |
| `bypassPermissions` | `BYPASS PERMISSIONS (no further prompts)`; strings also say it auto-approves every tool call except explicit deny rules and skips `canUseTool` |

Plan-mode strings include write blocks (`Cannot write to … while in plan mode`)
and `ExitPlanMode` / plan-approval lifecycle. Auto-mode strings include
headless classifier abort (`Agent aborted: too many classifier denials in
headless mode`). Those are provider behaviors, not host containment.

## Classification Against Current Plan

Current selected argv hardcodes `--permission-mode plan` and
`--tools Read,Glob,Grep` with `--no-session-persistence`.

| Candidate | Vs current Plan | Disposition |
| --- | --- | --- |
| `acceptEdits` | auto-approves working-directory edits and common filesystem commands | widens writes; rejected |
| `auto` | classifier can auto-approve actions; headless denial limits exist; account/model gated | widens approval authority; not containment; rejected |
| `dontAsk` | auto-denies prompting paths; ambient `permissions.allow` via `--setting-sources` can still pre-approve; not Plan write-block / ExitPlanMode lifecycle | not closed as Plan replacement |
| `default` / `manual` | Manual ask-each-time; headless without a prompt tool auto-denies ask paths; loses Plan application/lifecycle; ambient allow rules can widen | not closed as Plan replacement |
| `bypassPermissions` | auto-approves tools | excluded |

A non-empty deliver-now row would need closed effective resource/tool authority
that does not silently widen writes, tools, approvals, or isolation relative to
Plan, plus closed application, terminal, cleanup, and omission. `acceptEdits`
and `auto` fail the widen test on package labels alone. `default` and `dontAsk`
remain parse-accepted but do not close Plan-equivalent application or
operation-private authority under the selected `--setting-sources
user,project,local` argv.

## Precedence And Ambient Authority

Docs and package strings agree that `--permission-mode` outranks
`permissions.defaultMode` and the built-in start mode. For `claude -p`, docs
say the built-in start is Manual/`default` when flag and settings do not select
otherwise.

Selected Swallowtail headless argv already passes
`--setting-sources user,project,local`. Ambient `permissions.defaultMode` and
`permissions.allow` can therefore change effective approval without an argv mode
change. Disposable-home probes with ambient `defaultMode: acceptEdits` still
reach the empty-prompt reject when `--permission-mode plan` is present and when
the mode flag is omitted; that proves parse survival only, not effective
authority. Caller argv alone is therefore not operation-private effective
permission policy on this route.

## Application, Terminal, Cleanup

Permission mode is a provider approval-policy label. It is not host
containment and not portable working-resource access. Plan remains the current
selected application: research/propose without making edits, with Plan-mode
tools and lifecycle strings present in the package.

Unsupported tokens reject before prompt effects. Accepted modes share the same
empty-prompt reject terminal under print mode. No distinct Swallowtail
permission-mode diagnostic exists. Live tool-denial, plan-exit, and classifier
terminals were not exercised; they need provider turns this card forbids.
Cleanup is process exit under `--no-session-persistence`.

## Current Driver Omission

`claude_code_command::arguments` emits `--permission-mode plan`. Omission of an
alternate mode keeps that exact pair and the rest of the prior argv
byte-identical. No claim that the built-in `-p` Manual default is selected
follows from keeping Plan. No claim that ambient settings are inert follows
either.

## Truth Separation

| Layer | What this lane proves | What remains unproved / rejected |
| --- | --- | --- |
| requested | caller could pass another `--permission-mode` value | safe Plan replacement |
| parsed | closed help/invalid choice list; `default`/`manual` accepted; case/snake rejected | effective authority at parse |
| configured | selected argv fixes Plan + read tools + setting-sources | ambient allow-rule inventory |
| dispatched | empty-print accepts listed modes before empty-prompt reject | live tool dispatch under each mode |
| accepted | invalid modes rejected pre-effect | live org/account auto-mode gates |
| effective | package labels show acceptEdits/auto widen; Plan has write-block/ExitPlanMode strings | prompt-free observation of tool allow/deny under subscription |
| returned | shared empty-prompt error for accepted modes | live denial/plan-exit decoder mapping |
| observed | help/strings/empty-print only | live tool turns |
| persisted | `--no-session-persistence` on selected route | ambient settings mutation |

## Claim Strength

| Claim | Strength at the exact evidence boundary |
| --- | --- |
| help advertises `--permission-mode` with fixed choice list at every published `2.1.220..=2.1.241` point | observed |
| invalid / missing values reject before prompt effects | observed at every published point |
| `default` accepted; `manual` aliases to `default` | observed |
| `acceptEdits` auto-approves edits per package label | observed |
| `auto` uses classifier and can abort headless on denial limit | observed in strings; not live-run |
| `dontAsk` auto-denies prompting paths per package label | observed |
| Plan researches without making changes and blocks writes in plan mode | observed in strings + docs lead |
| selected route can replace Plan with `default`/`dontAsk` without widening or losing Plan application | not closed |
| provider permission label is host containment | rejected |
| current headless argv already uses a non-Plan mode | not observed; Plan retained |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `--permission-mode acceptEdits` replacing Plan | package auto-approves edits | empty; widens writes |
| `--permission-mode auto` replacing Plan | classifier auto-approval; not containment | empty; widens approvals |
| `--permission-mode dontAsk` replacing Plan | parse-accepted; ambient allow rules + non-Plan lifecycle | empty; authority/application do not close |
| `--permission-mode default` / `manual` replacing Plan | parse-accepted Manual; headless ask→deny ≠ Plan lifecycle | empty; authority/application do not close |
| `--permission-mode bypassPermissions` | excluded | rejected |
| unsupported tokens | pre-effect parser rejection | rejected as binding values |
| treat mode label as host containment / portable access | contradicted by docs + contracts posture | rejected |
| omission (keep Plan) | current argv unchanged | unchanged; remains exact |
| live tool-authority proof | requires provider turns or ambient mutation | withheld |

Deliver-now rows: **none**.

## Decision

Card 252 is complete as an evidence stop with an honest empty set. Exact
packages advertise and parse `--permission-mode` across every published
`2.1.220..=2.1.241` point. `acceptEdits` and `auto` widen authority relative to
Plan. `default`/`manual` and `dontAsk` parse but do not close Plan-equivalent
application or operation-private effective authority under selected
setting-sources. `bypassPermissions` stays excluded. No production binding,
guide capability claim, matrix row, or shared closeout follows from this
record. The current headless route keeps `--permission-mode plan`.

## Sources

- npm `@anthropic-ai/claude-code` `2.1.220` and `2.1.241`
- official `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241`
- [Permission modes](https://code.claude.com/docs/en/permission-modes)
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Settings reference](https://code.claude.com/docs/en/settings-reference)
- [Research 202 Claude Code 2.1.241 Identity](./202-claude-code-2-1-241-identity.md)
- [Research 226 Claude Code Headless Maximum Turns](./226-claude-code-headless-maximum-turns-evidence.md)
- [Research 245 Claude Code Headless Advisor](./245-claude-code-headless-advisor-evidence.md)
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-permission-mode.json`
