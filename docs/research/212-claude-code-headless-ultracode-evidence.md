# 212 Claude Code Headless Ultracode Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.065 / 181

## Question

Which exact `claude-code.headless` versions, selected models, and operation
profiles can dispatch Claude Code Ultracode through an adapter-local selection
without flattening it into portable reasoning, widening tools or process
topology, or relying on live-provider inference?

## Method And Boundary

Evidence was collected on 2026-08-25 with no Claude installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation authorized for binding claims. Official npm packages and Darwin arm64
platform packages for `2.1.202`, `2.1.203`, `2.1.220`, and `2.1.241` were
downloaded to disposable `/tmp` paths only. Native executables were inspected
through `--version`, `--help`, local `--effort` parser cases, and extracted
implementation strings. The host `claude` executable was not installed or
replaced.

The route is `claude-code.headless`, driver `swallowtail.claude-code.headless`,
axis `claude-code.headless-stream-json`, qualified window
`2.1.220..=2.1.241`, and existing private behavior
`claude-code.headless.stream-json.v1`. This record does not amend response-only
or Claude Agent ACP claims.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Claude Code model configuration](https://code.claude.com/docs/en/model-config.md) | Ultracode definition, `v2.1.203+` requirement, `xhigh` plus workflow orchestration, settings/env limits | 2026-08-25 | `438f76e45c6224ea2af5a711e5946213b67d5f33735f748f28d6dbfed87fe47d` |
| [Claude Code workflows](https://code.claude.com/docs/en/workflows) | dynamic workflow orchestration, keyword trigger, `/effort ultracode`, disable semantics | 2026-08-25 | `5bf0254f3ca0823f20910abfd0d4d6964bdbc2bbbd38ff6f47a3e015f8b77470` |
| [`@anthropic-ai/claude-code@2.1.202`](https://registry.npmjs.org/@anthropic-ai/claude-code/2.1.202) | pre-support boundary | 2026-08-25 | integrity `sha512-70Hcc/NJuME3K...`; shasum `56f99cf3084ff3e9e42fbf4cb59fabb11e9b56fa` |
| [`@anthropic-ai/claude-code@2.1.203`](https://registry.npmjs.org/@anthropic-ai/claude-code/2.1.203) | documented first-support point | 2026-08-25 | integrity `sha512-X5sAxPpCLLuxo...`; shasum `c41435ed8fc671f96145f5e1278ddeeda6ba7ced` |
| [`@anthropic-ai/claude-code@2.1.220`](https://registry.npmjs.org/@anthropic-ai/claude-code/2.1.220) | qualified baseline | 2026-08-25 | integrity `sha512-ogBrvwkqF9f8o...`; shasum `29e7249f01f9602b78c2d5f3c2f1c8a11b2ebcb4` |
| [`@anthropic-ai/claude-code@2.1.241`](https://registry.npmjs.org/@anthropic-ai/claude-code/2.1.241) | qualified ceiling; Research 202 identity | 2026-08-25 | integrity `sha512-S7DWEmJJAsI5taAUjhKm6soXcFJYIVeTH6Lg9kmp3yntFllCP612hGwZ7thOGh8r7YaRUH9+1jCX5A9QGazsxg==`; shasum `150077700180a6f915a486a34b4c34404e4aee59` |
| Darwin arm64 native binaries | exact parser/help/implementation evidence | 2026-08-25 | see `headless-ultracode.json` |
| `headless-ultracode.json` | sanitized deterministic specimen corpus | 2026-08-25 | fixture hash asserted in tests |

Official documentation describes Ultracode as a Claude Code product setting,
not a portable model effort level. It sends `xhigh` and additionally enables
dynamic workflow orchestration for substantive tasks. Passing `ultracode` to
`--effort` or Agent SDK `effortLevel` requires Claude Code `v2.1.203` or later.
Before `v2.1.203`, documentation says `--effort ultracode` is unknown and the
session starts at default effort.

## Exact Help And Parser

Exact `--help` at every probed point advertises only:

`--effort <level> (low, medium, high, xhigh, max)`

`ultracode` is not listed in the help corpus at `2.1.202`, `2.1.203`,
`2.1.220`, or `2.1.241`. The qualified ceiling `2.1.241` help is byte-identical
to Research 202's frozen linux-x64 help
(`71ad650f59e08ae40ede14c534db4f49d8590ee5a4f92f6da2882d3a5560fea6`).

Local parser behavior diverges from the documentation lead at the first-support
boundary:

| Version | `--effort ultracode` parser | stderr |
| --- | --- | --- |
| `2.1.202` | rejected | `Warning: Unknown --effort value 'ultracode' — ignoring it and using the default effort. Valid values: low, medium, high, xhigh, max.` |
| `2.1.203` | accepted | none |
| `2.1.220` | accepted | none |
| `2.1.241` | accepted | none |

Unknown non-ultracode values such as `notavalidvalue` warn and fall back to
default effort at all probed versions with the same valid-value list.

Implementation strings are present from `2.1.202` onward and describe Ultracode
as `xhigh effort plus standing dynamic-workflow orchestration`, a keyword
trigger, and menu text `- ultracode: xhigh + dynamic workflow orchestration
(this session only)`. At `2.1.241` the binary also contains
`apply_flag_settings: ultracode is not available for this session (dynamic
workflows are off, or the model / your organization does not allow xhigh
effort)`.

Parser acceptance at `2.1.203+` therefore establishes a hidden effort value
accepted by argv parsing. It does not establish help truth, model eligibility,
entitlement, effective workflow behavior, or a bindable adapter-local row.

## Model, Entitlement, And Settings

Official documentation says Ultracode is available on models that support
`xhigh`, that `/effort` omits it on other models, and that persisted
`effortLevel` and `CLAUDE_CODE_EFFORT_LEVEL` do not accept `ultracode`. When
workflows are disabled, `--effort ultracode` is documented to set `xhigh`
effort only.

No exact package/help/parser case in this lane freezes which selected models
admit Ultracode, which subscription or organization gates apply, or what
failure shape appears before or after authentication. Those facts require account
or provider work that this card does not authorize for binding claims.

## Dynamic Workflow And Topology

Ultracode's documented product meaning includes dynamic workflow
orchestration, not just `xhigh`. Official workflow documentation describes
multi-step workflows, optional `ultracode` keyword triggering, standing session
orchestration under `/effort ultracode`, and workflow disable semantics.

Extracted implementation strings name workflow scripts, remote workflow args,
workflow size warnings, `CLAUDE_CODE_DISABLE_WORKFLOWS`, and Agent SDK
`apply_flag_settings` ultracode gating tied to dynamic workflows and `xhigh`
availability.

The selected headless command fixes `HarnessMode::Plan`, passes
`Read,Glob,Grep`, disables session persistence, and suppresses MCP servers.
That composition does not prove that Ultracode's workflow orchestration is
disabled or fully contained inside the route's owned child and joined cleanup
truth. No deterministic no-auth specimen in this lane proves:

- whether workflow tools or subagents become model-visible beyond the fixed set
- whether extra child processes spawn outside the owned task scope
- whether internal workflow prompts, teammate detail, or hidden reasoning would
  appear in stream JSON or activity
- whether effective Ultracode was active versus argv acceptance alone

Successful text, usage, or subtype cannot confirm effective Ultracode behavior.
This lane therefore withholds every deliver-now row.

## Portable Reasoning Disposition

Ultracode must not enter portable `ReasoningMode` as a seventh value or as an
alias for `xhigh`. Official documentation and exact implementation strings
treat it as a product setting whose `xhigh` component is coupled to dynamic
workflow orchestration. Mapping it to `ReasoningMode::xhigh` would erase the
workflow behavior and violate Contract 040's prohibition on UI-label translation.

## Claim Strength

| Claim | Strength at exact evidence boundary |
| --- | --- |
| help advertises `ultracode` | not observed at any probed version |
| parser accepts `--effort ultracode` | observed from `2.1.203` through `2.1.241` |
| parser rejects `--effort ultracode` | observed at `2.1.202` |
| hidden implementation exists | observed in extracted strings from `2.1.202+` |
| selected model supports Ultracode | unproved without account/provider work |
| entitlement and billing posture | unproved without account/provider work |
| effective Ultracode under Plan + fixed tools | unproved; topology not bounded |
| dynamic workflows contained by route | unproved |
| omission byte-equivalence for a future binding | not applicable; no binding admitted |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| ordinary effort `low\|medium\|high\|xhigh\|max` | existing route and help corpus | unchanged; remains the only admitted portable effort set |
| `--effort ultracode` at `2.1.202` | exact parser rejects with frozen warning | rejection evidence only |
| `--effort ultracode` at `2.1.203..=2.1.241` | exact parser accepts hidden value; help omits it | withheld; model, entitlement, workflow topology, and effectiveness unproved |
| Ultracode as portable `ReasoningMode` | product docs and implementation strings | rejected |
| Ultracode as `ReasoningMode::xhigh` alias | coupled workflow orchestration in docs/strings | rejected |
| adapter-local opt-in binding | no exact row with bounded topology | withheld |

Deliver-now rows: **none**.

No new Contract 029 facade point, private behavior revision, guide capability
claim, matrix row, or production binding follows from this record. Cards 182
and 183 remain blocked.

## Decision

Card 181 is complete as an evidence stop. Cards 182 and 183 are blocked and
were not executed. The exact evidence is sufficient to retain the current
headless route and to reject a future Ultracode binding until exact package
evidence proves model and entitlement bounds, bounded workflow/process topology
under the selected Plan-mode command, and effective behavior without relying on
live provider inference or ambient settings mutation.
