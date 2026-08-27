# 243 Cursor ACP Model-Parameter Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.087 / 246

## Question

Which exact qualified `cursor-agent.acp` build, model, Fast, effort, context,
and lifecycle rows, if any, can bind caller-selected model parameters with
exact membership, application, selected-value confirmation, and omission?

## Method And Boundary

Authorized evidence frozen on 2026-08-27:

- official Cursor ACP page
- exact installed `2026.07.01-41b2de7` and `2026.08.04-aaa8809`
- official darwin-arm64 archive extracts for `2026.07.23-e383d2b` and
  `2026.08.11-e8db854` (read-only; no host install/update)
- prompt-free `--version` / `--help` on those binaries
- tagged ACP agent and SDK chunks for all four builds
- frozen initialize fixture
  `acp-v1-cursor-agent-2026.07.01-41b2de7/initialize.ndjson`
- current Swallowtail ACP initialize and `CursorAcpSessionProfileInput`
- Research 076 / 135 / 183 as contrast and identity only

No login, `authenticate`, `session/new`, `session/set_config_option`, account
or catalogue inspection, provider prompt, paid work, host install, update, or
host mutation. Headless bracket grammar stays sibling-route evidence.

Frozen corpus:
`crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-acp-model-parameters-2026.07.01-2026.08.11/`.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Cursor ACP docs](https://cursor.com/docs/cli/acp) | no Fast/effort/context; no `set_config_option`; modes only | 2026-08-27 | `847b6884b6bc037cee0d606066435895e8fa80db42c84e0bd313a59a4475af16` |
| initialize fixture | no model/config capability on initialize | frozen | `e2347b643242e300b09fc24bb67704708b80f44a889c542911c36e9dd1e98435` |
| `2026.07.01` `5721.index.js` | ACP agent: auth-gated `newSession`, picker modes, config apply | 2026-08-27 | `0332efbd33814b900e00b52753eb2b9d4ab0fa022dc264c162d2b4f535bda48f` |
| `2026.07.01` `8096.index.js` | wire `session/set_config_option` | 2026-08-27 | `139cb9c33b3464c2763044ab82b79a599ab197a27c5691dd6fce86d00f6557d5` |
| `2026.07.23` archive | identity | 2026-08-27 | `f2eb25851f2079dcdf0558a816e06c402d187abfca93255d35167020439ebbf2` |
| `2026.07.23` `6447.index.js` | ACP agent seams match July/August host specimens | 2026-08-27 | `7cebbaa556244f35fc1f45c281c6b8959c05ba5c163e20b7de96cf9e22fb79e9` |
| `2026.08.04` `8869.index.js` | ACP agent seams | 2026-08-27 | `5c702c38a79ca379d19b6aaf7cf13545fc5b8d887842e90f02e3e942fb5da178` |
| `2026.08.04` `8096.index.js` | wire `session/set_config_option` | 2026-08-27 | `05be7f7005e3e0025f3de79de667b9fffc561ce0614d425055e47c0e0bcbd07c` |
| `2026.08.11` archive | identity (matches Research 135) | 2026-08-27 | `46044d6d7bcbd7b49a0cf1cd01aa4ca79aaa2ea5f2c7a32965fc0ebe29841790` |
| `2026.08.11` `2996.index.js` | ACP agent seams (matches Research 135) | 2026-08-27 | `db023030987a1c0a54b2c7102e27faea657a4ad1320b2da153ed590ddd21eb39` |
| headless help excerpt fixture | sibling `--model` bracket contrast only | frozen | `df64efb90545441ed6786304babb02c7dd5acfb9a698c3b9ae9e5e6699f021c6` |

## ACP Surface Versus Headless Contrast

Official ACP docs cover initialize, `cursor_login`, `session/new` or load,
prompt, permissions, and modes `agent` / `plan` / `ask`. They do not name
Fast, effort, context, bracket model grammar, or `session/set_config_option`.

All four qualified builds still publish headless `--model` help with
`claude-opus-4-8[context=1m,effort=high,fast=false]`. That is Research 183
authority for `cursor-agent.headless` only. It is not ACP route authority.

## Exact ACP Seams

Across all four ACP agent chunks:

1. `newSession` rejects when unauthenticated (`authRequired` / login message).
2. Authenticated `session/new` builds `models` and `configOptions` from
   `modelManager` / `getAcpAvailableModels` — account-dependent membership.
3. SDK dispatch exposes `session/set_config_option` → `setSessionConfigOption`.
4. Picker mode is `parameterized` only when
   `clientCapabilities._meta.parameterizedModelPicker === true`; otherwise
   `variants`.
5. Swallowtail initialize sends only `fs` client capabilities. No `_meta`.
   Current prepared route therefore stays in `variants` mode.
6. In `variants` mode, non-`model` config ids are rejected
   (`Unknown config option`). Fast / effort / context are not independent
   config options on that path.
7. In `parameterized` mode, non-model config ids must match the current
   model's account `parameterDefinitions`. ACP agent chunks contain no
   literal `"fast"` or `"context"` strings; `effort` appears only in a
   thought-level name heuristic.
8. `setSessionConfigOption` returns rebuilt `configOptions`. That is not
   provider-effective confirmation. Effective application needs a turn and
   is out of this lane's boundary.

Initialize fixture and Research 135 initialize probes advertise
`cursor_login`, load/list, image, MCP HTTP/SSE. They do not advertise model
or config-option capability.

Current preparation: `CursorAcpSessionProfileInput` is request id + working
resource only. Ambient read-write. No model, reasoning, parameter, tool,
permission, or plan-mode option.

## Deliver-Now Table

| Build | Model | Parameter | Value | Selection seam | Membership | Confirmation | Scope | Omission | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| all four | any | `fast` | any | none on prepared ACP | account-gated; unfrozen | none | n/a | retain ambient ACP | no |
| all four | any | `effort` | any | none on prepared ACP | account-gated; unfrozen | none | n/a | retain ambient ACP | no |
| all four | any | `context` | any | none on prepared ACP | account-gated; unfrozen | none | n/a | retain ambient ACP | no |

No row is deliver-now. Honest empty set.

## Truth Layers

| State | Exact truth |
| --- | --- |
| Requested | no Fast/effort/context member on ACP session profile |
| Negotiated | initialize has no model/config capability; picker meta absent |
| Dispatched | no `session/set_config_option`; no headless `--model` on ACP |
| Accepted | unproved without authenticated membership |
| Effective | unproved; provider turn forbidden here |
| Returned | no prepared selected-value field |
| Observed | source shows auth-gated, account-dependent seams only |
| Omission | keeps current ambient ACP frames and capability claims |

## Lifecycle

| Lifecycle | Disposition |
| --- | --- |
| New session | auth-gated; ambient account model/config state only |
| Turn | no pre-prompt parameter select/confirm on current route |
| Process loss / replacement | nothing to reassert |
| Omission | current prepare/open/close authority unchanged |

Permission, working-resource, and isolation authority do not widen.

## Promotion

Research 243 promotes an empty deliver-now set.

A later lane may reopen only when secret-free exact evidence closes, for a
named qualified build:

1. ACP-local pre-prompt selection without ambient account inspection
2. independent Fast / effort / context membership per model
3. pre-effect rejection and returned selected-value confirmation
4. turn/replacement/omission scope
5. no promotion of headless bracket grammar or catalogue observations

Until then, `cursor-agent.acp` stays ambient without model-parameter binding.
