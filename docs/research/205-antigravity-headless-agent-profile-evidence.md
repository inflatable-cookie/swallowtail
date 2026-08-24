# 205 Antigravity Headless Agent-Profile Evidence

Status: promoted
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.058 / 161

## Question

Can exact qualified `antigravity.headless` versions bind one typed agent
profile through `--agent`, reject invalid or unavailable ids without fallback,
and confirm the selected id through stream-JSON `init.agent` while preserving
the route's current model, access, isolation, permission, and continuation
truth?

## Method And Boundary

Evidence was frozen on 2026-08-24 from:

- current official public Antigravity CLI documentation
- frozen secret-free fixtures under `antigravity-cli-1.1.9` and identity corpora
  through `1.1.17`
- Research 079 / 080 / 177
- promptless host help and `agy agents` listing shape checks
- one invalid-selection probe that was intended to fail before a provider turn

No install, login, account/config/profile mutation, credential capture, or
account-identity recording. Profile bodies, instructions, tool lists, and paths
were not read.

Boundary breach to record honestly: the invalid-`--agent` probe used
`--print` / `--output-format json` expecting a pre-init failure comparable to
invalid `--model`. The host returned `status: SUCCESS` with a model response
and nonzero usage. That is fail-open evidence, not an authorized inference
lane. No further provider prompts were sent after that observation.

Host PATH initially reported `agy` `1.1.9` with stdout help byte-identical to
the frozen fixture. Later in the same session the same PATH reported `1.1.19`
and moved help text to stderr. `1.1.19` is treated only as live
`UnverifiedNewer` observation. It is not newly qualified. Qualified claims
remain Research 177's `1.1.9..=1.1.17`.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Headless mode](https://antigravity.google/docs/cli/headless.md) | `--agent`, `agy agents`, `init.agent` only when overridden; no-silent-fallback promised for `--model` only | 2026-08-24 | `28b62c510bef159b689c5ec9aef07aea487344146df30e3f8f03814a083d445c` |
| [Headless HTML](https://antigravity.google/docs/cli/headless/) | same page rendered | 2026-08-24 | `fac558ce2dff852661f6256e807d0029ac7a87a16e0a65032b875d6784b23d42` |
| [Agents command](https://antigravity.google/docs/cli/commands/agents.md) | custom agents carry specialized instructions and tool permissions; workspace/global markdown definitions | 2026-08-24 | `46e8f1a5d8a4716a6540fc48d1eb6eda9d29928a9ee60bab2c2c14a69c62896d` |
| [Permissions](https://antigravity.google/docs/cli/permissions.md) | fine-grained allow/ask/deny; headless soft-deny posture | 2026-08-24 | `3bb2557f2f7fb9cad044cead2ed6526d7c81a54be324a8e26b5bbe47c1c2ea4d` |
| [CLI reference](https://antigravity.google/docs/cli/reference.md) | slash `/agents` panel; settings defaults | 2026-08-24 | `504992d65d0abf328e41477b72c724721b92859ead3bb245d091198412f009c1` |
| [Sandbox](https://antigravity.google/docs/cli/sandbox.md) | provider sandbox remains separate from agent profiles | 2026-08-24 | `ae9b369ce66155831ebcc9cbf8893b5eecf66c4e1fbca1f13b1413f41c63b7f7` |
| fixture `antigravity-cli-1.1.9/help.txt` | exact `--agent`, `agent`, `agents` on qualified baseline | frozen | `c64e4bf74262cebba7d161d29e1632682f64f844c6ca1a718c77a1fa4e8f8343` |
| live host help stderr after PATH reported `1.1.19` | `--agent` / `agents` still present on UnverifiedNewer host | 2026-08-24 | `a89116526092091e84c15d6e2c7866c5630510d0f57b3ff82be406e2225a2736` |
| Research 177 + `antigravity-cli-1.1.17` identity/protocol | selected route through `1.1.17`; help delta since `1.1.15` is `mcp`; decoder corpus stays `1.1.9` | 2026-08-21 | see Research 177 |
| decoder init fixtures under `antigravity-cli-1.1.9/*.jsonl` | selected and omitted runs; no `init.agent` field | frozen | success init `8e800f4c…`; structured `a16e904f…`; continuation `64968e79…` |

## Advertisement Versus Selection

Help and docs advertise a dispatch surface:

| Surface | Qualified evidence |
| --- | --- |
| `--agent` flag | present on frozen `1.1.9` help and still named on live `1.1.19` help |
| `agy agent` / `agy agents` | list subcommands; help-only flags `-h` / `--help` |
| `init.agent` | official headless docs: string field appears only when `--agent` overrides |
| Custom agent definitions | official agents docs: markdown under workspace `.agents/agents/` or global `~/.gemini/config/agents/`; specialized instructions and tool permissions |

Current Swallowtail production path:

- argv never emits `--agent`
- init validation requires exact model, `permission_mode=request-review`,
  array-shaped tools, and string cwd
- `init.agent` is not inspected
- tools contents are not pinned
- continuation reasserts model and optional conversation id only

Advertisement is not selectable confirmation.

## Listing Shape

Promptless `agy agents` on this host:

| Observation | Result |
| --- | --- |
| early `1.1.9` PATH probe | exit `0`; one plain-text line; 18 bytes; no path/email tokens in shape scan; stderr empty |
| later probes same session | exit `0`; empty stdout and stderr |
| `agent` vs `agents` | identical empty later probes |
| portable catalogue | no — host-local, non-stable across the session, and officially able to include custom/workspace/global names |

Empty or single-line host listings are installation observations, not a
Swallowtail-qualified profile-id domain. Listing output is not a model
catalogue and is not an agent-definition API.

## Invalid Selection And Fallback

Official headless docs explicitly say unknown `--model` does not silently fall
back in headless mode. They do not make that promise for `--agent`.

Live invalid-selection probe on this host:

```text
agy -p noop --model gemini-3.6-flash-high \
  --agent swallowtail-nonexistent-agent-zzzz \
  --output-format json --print-timeout 5s
```

Observed:

| Field | Value |
| --- | --- |
| exit | `0` |
| JSON `status` | `SUCCESS` |
| response text | present |
| usage | nonzero input/output/thinking tokens |
| stderr | empty |
| fallback language | none required — the run completed as an ordinary success |
| stdout SHA-256 | `51da1803f9b6c8f6ee9271c413cde72cfc103b0e0c9b755fa3b060287956fafb` |

Whitespace-only `--agent` also returned `SUCCESS` on a follow-up probe before
provider work was stopped.

This is silent fail-open to an ambient/default agent path. It fails the
deliver-now gate that requires no silent fallback.

Conversation ids, response bodies, and account-visible catalogue text are not
retained in this research record.

## Init Confirmation

Official docs place `init.agent` in the stream-JSON init payload only when
`--agent` overrides. Existing decoder fixtures for success, structured output,
and continuation omit `init.agent` entirely. The parser never reads it.

Because invalid ids can still succeed, a later `init.agent` equality check
cannot by itself prove that Swallowtail dispatched a caller-selected profile
rather than an ambient default. Missing `init.agent` after an explicit
`--agent` would be ambiguous under fail-open behavior. Exact selected
confirmation is therefore not deliverable on this evidence.

## Authority And Composition

Official agents documentation states custom agents load specialized system
instructions and tool permissions. Current Swallowtail accepts any
array-shaped `init.tools` and does not claim tool-set equality. Resource
access (`--mode plan`), provider sandbox (`--sandbox`), and
`permission_mode=request-review` remain independently mapped, but they do not
freeze the tool/instruction surface a profile may change.

Deliver-now requires selected profiles not to widen resource, permission,
tool, isolation, subagent, account, or provider-session authority beyond the
immutable prepared plan. Exact evidence does not prove that composition.

## Lifecycle Disposition

| Lifecycle | Disposition |
| --- | --- |
| Structured-run omission | retain current argv; do not infer an ambient agent id |
| Structured-run explicit `--agent` | advertised; not deliver-now because invalid ids fail open and authority composition is unproved |
| Continuation first / resumed / replacement child | no evidence that conversation id restores or reasserts a profile; reassertion would inherit the same fail-open and authority gaps |
| Invalid / unknown / stale id | live host success without error — stop |
| `UnverifiedNewer` (including live `1.1.19`) | no private mapping to inherit; keep omission path |

## Version / Operation / Profile Disposition

| Version | Operation | Profile id | Listed | Dispatched | Confirmed `init.agent` | No silent fallback | Authority-safe | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `1.1.9..=1.1.17` | structured run | any caller id | host-local only | argv possible | unproved | **no** — live invalid id succeeded | **no** — custom profiles may change tools/instructions | no |
| `1.1.9..=1.1.17` | exact-id continuation child | any caller id | host-local only | unproved | unproved | no | no | no |
| `1.1.9..=1.1.17` | any | omission | n/a | none today | n/a | n/a | current path retained | n/a — omission is not a selection row |
| live `1.1.19` | any | any | empty on later probes | not qualified | not qualified | observed fail-open | not qualified | no |

No row is deliver-now. The empty set is fail-open invalid selection plus
unbounded host-local identity and authority composition risk. It is not because
`--agent` or `init.agent` are undocumented.

## Application, Failure, And Revision Posture

Requested, planned, dispatched, accepted, effective, and observed remain
distinct. Docs and help can request. Live host can dispatch argv. Acceptance
requires exact confirmation and fail-closed invalid handling. Those are absent.

No adapter-local `AntigravityAgentProfileId`, prepared input, plan constraint,
request member, argv emission, or init check is proposed. No behavior, driver,
claim, matrix, guide, contract, or configured-instance revision is proposed.
Omission keeps current production behavior.

## Promotion

Research 205 promotes an empty deliver-now set.

Cards 162-163 stay blocked. A later lane may reopen this family only when exact
qualified evidence shows:

1. invalid/unavailable/stale ids fail closed before accepted output
2. a bounded profile-id domain that does not require private profile bodies
3. exact `init.agent` equality on every selected child before output acceptance
4. composition that retains prepared resource, permission, tool, isolation, and
   continuation boundaries

Until then, `antigravity.headless` continues without `--agent`.
