# 252 Mistral Vibe Headless Agent-Profile Evidence

Status: promoted
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.089 / 255

## Question

Which exact `mistral-vibe.headless` `2.24.2` agent profiles beyond fixed Plan,
if any, can bind with closed resource/tool authority, application, terminal,
lifecycle, and omission truth?

## Decision

Promote an honest empty deliver-now set. Exact `2.24.2` freezes builtin
`--agent` membership and profile overrides, but no profile beyond fixed Plan
closes a non-widening authority row for this route.

## Method And Boundary

Evidence frozen on 2026-08-28 from GitHub tag `v2.24.2` commit
`5e6aa0f6beb3454454f4c1de74a7652ba577ab05`, the matching PyPI sdist digest from
Research 150, tagged README plus CLI/agent/runtime sources, the existing
`mistral-vibe-headless-2.24.2` identity corpus, and the current adapter command
builder / prepared guide.

No Vibe install, platform-archive extraction, login, credentials, provider
prompt, paid work, or ambient host mutation. Host PATH has no `vibe`.

Selected operation stays `mistral-vibe.headless` on axis `mistral-vibe.release`
`2.24.2`. Current argv remains
`vibe --prompt --output streaming --max-turns <1..=8> --trust --agent plan
--workdir <cwd>`. Streaming NDJSON, maximum-turn binding, host-approved
executable/environment, and lifecycle stay unchanged. `auto-approve`,
`--auto-approve`, and `--yolo` stay excluded.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| GitHub tag zip `v2.24.2` | exact tagged tree | 2026-08-28 | `145d2280605451c921d24733bab51fda9c41096e11fc91c169bc01d94d67a8d1` |
| [README.md](https://raw.githubusercontent.com/mistralai/mistral-vibe/v2.24.2/README.md) | builtin agent blurbs; programmatic `--auto-approve` note | 2026-08-28 | `6016ef2167e602c955577706452b37b04241244ea37fad3ffae8a7c44a1ad421` |
| `vibe/core/agents/models.py` | builtin profile overrides and safety labels | 2026-08-28 | `d80dd5cfc04d51a826fd7c23b866f3004b3ee9266c3448ef32a3c06fbce5b227` |
| `vibe/cli/entrypoint.py` | `--agent` parser; `--auto-approve`/`--yolo` | 2026-08-28 | `b1d17309da8f2b24c2232a18549ca4e19522be66e689f08f2271a760422136df` |
| `vibe/cli/cli.py` | programmatic SessionOptions; always-disabled tools | 2026-08-28 | `295ce3994f8bb73ae53af8a129fdcc66b889ebfd0b7a1c68d3a9192093fff1dc` |
| `vibe/cli/programmatic.py` | streaming wire; `deny_callback`; `session.close` | 2026-08-28 | `36a5008914136714a851880b3c6921f6ba196371e28422f58a45c8a728b25b34` |
| `vibe/core/config/vibe_schema.py` | `default_agent = accept-edits` | 2026-08-28 | `8e7b3042430fdac5497604ac268d423302e64f4880894ada46e97cae881a0e97` |
| `vibe/agents.py` | `AgentSafety` enum | 2026-08-28 | `cae01a841335af39378a0cf63e397b951b305e0c008396c217531bdd7e72588b` |
| `vibe/core/agents/manager.py` | membership, availability, override install | 2026-08-28 | `7ea04b7acebd73a2521e01a91458f76310823c6dfaa31239edaa8ea52d245467` |
| `vibe/core/agents/registry.py` | `apply_profile_overrides` | 2026-08-28 | `4f2f36cb8c6a17a6889119ff809846f30ba85d30342197fb0ae72e9f700275dc` |
| `vibe/core/config/layers/agent_profile.py` | agent-profile config layer | 2026-08-28 | `a252e3773cce8fac025dcd458ccdeef62e37ebc95a4829fea0176a381265272f` |
| `vibe/app_server/_runtime.py` | agent resolution; `force_bypass_tool_permissions` | 2026-08-28 | `905e8491a4ba8d3e9a451750f6d4646cc836df3b1716d4b66062f2b2adabc209` |
| fixture `mistral-vibe-headless-2.24.2/` | identity, command, protocol baseline | 2026-08-19 | Research 150 |
| fixture `mistral-vibe-headless-2.24.2-agent-profiles/` | closed dispositions + frozen `models.py` | 2026-08-28 | workspace |
| Research 150 / 199 | identity and max-turns | promoted | siblings |
| prepared guide | fixed Plan argv | workspace | workspace |

`models.py` digest matches Research 150. Docs that programmatic mode defaults
to `auto-approve` remain stale versus tagged source; schema default on omit is
`accept-edits`.

## Parser And Membership

`--agent NAME` has no argparse choices and no argparse default. Help names
builtins `ask`, `plan`, `accept-edits`, `auto-approve`, plus custom TOML under
`~/.vibe/agents/`. Omit inherits `default_agent`; schema default is
`accept-edits`.

Builtin primary profiles from tagged `models.py`:

| Profile | Safety | Decisive overrides |
| --- | --- | --- |
| `ask` | `neutral` | disables `exit_plan_mode` only |
| `plan` | `safe` | `write_file`/`edit` `never` with `$VIBE_HOME/plans/*` allowlist; `read_file` allowlist to that plans pattern |
| `accept-edits` | `destructive` | `write_file`/`edit` `permission: always`; disables `exit_plan_mode` |
| `auto-approve` | `yolo` | `bypass_tool_permissions: true`; disables `exit_plan_mode` |

`explore` is `subagent` and cannot be primary `--agent`. `lean` requires
`installed_agents`. Custom TOML agents remain ambient.

## Classification Against Fixed Plan

| Candidate | Vs current `--agent plan` | Deliver-now |
| --- | --- | --- |
| `ask` | drops plans-only `read_file` allowlist; replaces write/edit `never` with tool-default `ask` | no — widens read and softens write gates |
| `accept-edits` | auto-approves `write_file`/`edit` | no — wider write by default |
| `auto-approve` | full tool-permission bypass | excluded |
| `explore` / `lean` / custom | not a closed primary beyond-Plan row | no |
| omission of `--agent` | inherits ambient/`accept-edits` | forbidden; retain exact `--agent plan` |

`AgentSafety` is a UI label. It does not prove host isolation or portable
resource access.

## Application, Terminal, Lifecycle, Omission

Requested `--agent NAME` becomes `SessionOptions.agent` or
`config.default_agent`. `AgentManager` installs profile overrides before
`act(prompt)` and raises `ValueError` for missing, excluded, or
subagent-as-primary names. Programmatic mode always sets `headless=True`,
disables `ask_user_question` and `exit_plan_mode`, and observe-and-denies
callbacks. `--auto-approve`/`--yolo` sets `force_bypass_tool_permissions`.

Headless `ASK` → `deny_callback` → skip is not the same closed authority as
Plan `NEVER`. Effective tool config still merges ambient layers. Frozen
streaming NDJSON fixtures do not confirm the applied agent id.

Terminal exits, limit mapping, and `session.close` cleanup stay as Research
150. Profile candidates do not change that lifecycle claim. Omission retains
exact current `--agent plan` argv; Swallowtail must not omit the flag.

## Truth Layers

| Layer | Exact `2.24.2` fact |
| --- | --- |
| requested | `--agent ask\|plan\|accept-edits\|auto-approve\|custom` |
| parsed | free-form `NAME`; no enum |
| configured | session agent or `default_agent` |
| dispatched | profile overrides installed pre-prompt |
| accepted | `AgentManager` membership/availability |
| effective | merged tool permissions; not host containment |
| returned | no agent id on frozen public-history lines |
| observed | source-level only here |
| persisted | ambient `default_agent` possible; `--trust` is this-invocation-only |

## Deliver-Now Table

| Version | Profile | Resource/tool authority closed | Application closed | Terminal/lifecycle closed | Non-widening vs Plan | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- |
| `2.24.2` | `ask` | no — headless deny ≠ Plan NEVER; read widened | parser/application known; stream confirm absent | lifecycle unchanged | no | no |
| `2.24.2` | `accept-edits` | write always | known | unchanged | no | no |
| `2.24.2` | `auto-approve` | bypass | excluded | n/a | no | no |
| `2.24.2` | custom / `lean` / `explore` | ambient or non-primary | open or rejected | n/a | no | no |
| `2.24.2` | omission | inherits default agent | forbidden | n/a | no | no |

No deliver-now row. The empty set rests on authorized evidence:

1. `ask` widens read and softens write relative to fixed Plan
2. `accept-edits` auto-approves file writes
3. bypass/`yolo` stay excluded
4. membership is not a closed portable domain once custom/ambient gates count
5. headless callback deny is not Plan-equivalent closed tool authority
6. no frozen stream confirmation of applied agent before provider effects

## Promotion

Research 252 promotes no deliver-now agent-profile row beyond fixed Plan.

Card 255 must not bind `ask`, `accept-edits`, or any other profile. The Mistral
Vibe adapter, prepared guide, matrices, and unreleased package API baseline
stay unchanged. The only corpus addition is
`mistral-vibe-headless-2.24.2-agent-profiles/`.

A later lane may reopen only when an exact package point proves a non-widening
profile with closed tool/resource authority, pre-effect rejection of unsupported
ids, applied-agent confirmation without provider work, and unchanged terminal
lifecycle. Until then, `mistral-vibe.headless` keeps `--agent plan`.
