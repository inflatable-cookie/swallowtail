# 205 Antigravity Headless Agent-Profile Evidence

Status: promoted
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.058 / 161
Correction: 2026-08-24 review — empty set rests on authorized evidence only;
live `--print` probes recorded as authority-boundary incidents, not as
qualified `1.1.9..=1.1.17` fail-open proof

## Question

Can exact qualified `antigravity.headless` versions bind one typed agent
profile through `--agent`, reject invalid or unavailable ids without fallback,
and confirm the selected id through stream-JSON `init.agent` while preserving
the route's current model, access, isolation, permission, and continuation
truth?

## Method And Boundary

Authorized evidence frozen on 2026-08-24:

- current official public Antigravity CLI documentation
- frozen secret-free fixtures under `antigravity-cli-1.1.9` and identity corpora
  through `1.1.17`
- Research 079 / 080 / 177
- promptless host help and `agy agents` listing shape checks

No install, login, account/config/profile mutation, credential capture, or
account-identity recording. Profile bodies, instructions, tool lists, and paths
were not read. Card 161 does not authorize provider prompts or paid work.

Qualified claims remain Research 177's `1.1.9..=1.1.17`. Host PATH initially
reported `agy` `1.1.9` with stdout help byte-identical to the frozen fixture.
Later in the same session the same PATH reported `1.1.19` and moved help text
to stderr. `1.1.19` is live `UnverifiedNewer` observation only. It is not
newly qualified.

Two `--print` probes ran during the session despite the card boundary. They
are recorded below as authority-boundary incidents only. They do not qualify
fail-open behavior for `1.1.9..=1.1.17` because the executing binary version
was not frozen clearly enough across the host drift.

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

Decoder init-line SHA-256 values under
`crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.9/`
(first line of each file; none contain `init.agent`):

| Fixture | SHA-256 |
| --- | --- |
| `headless-success.jsonl` init line | `8e800f4c92d28f1963fdd2d53b26f54d79f627e76e1c4d9da332c32e88ace4b8` |
| `headless-structured.jsonl` init line | `a16e904f8330dfd4c5adcb78b436cd8907356d1a364fc43b4c68e492c44382a2` |
| `continuation-first.jsonl` init line | `64968e793384cb19013dd6e7f9a998cb5ed04e46f53418532bd253af47a42221` |
| `continuation-second.jsonl` init line | `64968e793384cb19013dd6e7f9a998cb5ed04e46f53418532bd253af47a42221` |
| `continuation-mismatch.jsonl` init line | `ef52dfaa89ee98b6057d2c8747aa3955cedeb0bf65ae56897641187942b1ffbe` |

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

Exact qualified `1.1.9..=1.1.17` evidence does **not** freeze fail-closed
invalid/unavailable/stale `--agent` behavior. No version-pinned pre-init
failure specimen exists in fixtures, public docs, or Research 079/080/177.
Without that proof, deliver-now cannot claim no silent fallback on the
qualified range.

## Authority-Boundary Incidents

Card 161 forbids provider prompts. Two unauthorized `--print` probes still
ran. They are incidents, not qualified evidence:

| Incident | Argv shape | Observed | Notes |
| --- | --- | --- | --- |
| 1 | `--print noop --model gemini-3.6-flash-high --agent swallowtail-nonexistent-agent-zzzz --output-format json --print-timeout 5s` | exit `0`; JSON `status: SUCCESS`; response present; nonzero usage; stderr empty; stdout SHA-256 `51da1803f9b6c8f6ee9271c413cde72cfc103b0e0c9b755fa3b060287956fafb` | intended as pre-init failure check; crossed into a provider turn |
| 2 | same shape with whitespace-only `--agent` | exit `0`; JSON `status: SUCCESS`; response present | second provider turn in the same session |

Host PATH drifted from reported `1.1.9` to reported `1.1.19` during the
session, so the binary that executed those probes is not frozen as exact
qualified `1.1.9..=1.1.17`. Treat the outcomes only as out-of-scope /
`UnverifiedNewer` incident notes. Do not project them onto the qualified
window.

Conversation ids, response bodies, and account-visible catalogue text are not
retained in this research record.

## Init Confirmation

Official docs place `init.agent` in the stream-JSON init payload only when
`--agent` overrides. Existing decoder fixtures for success, structured output,
and continuation omit `init.agent` entirely. The parser never reads it.

Without a selected `init.agent` specimen and without proved fail-closed
invalid handling on the qualified range, exact confirmation before accepted
output is not deliverable.

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
| Structured-run explicit `--agent` | advertised; not deliver-now — fail-closed invalid handling unproved, selected `init.agent` missing, authority composition unproved |
| Continuation first / resumed / replacement child | no evidence that conversation id restores or reasserts a profile; same gaps block delivery |
| Invalid / unknown / stale id | fail-closed semantics unproved on qualified range |
| `UnverifiedNewer` (including live `1.1.19`) | no private mapping to inherit; keep omission path; live `--print` incidents stay out of scope |

## Version / Operation / Profile Disposition

| Version | Operation | Profile id | Listed | Dispatched | Confirmed `init.agent` | Fail-closed invalid | Authority-safe | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `1.1.9..=1.1.17` | structured run | any caller id | host-local only | argv possible | unproved — fixtures omit field | unproved on qualified range | **no** — custom profiles may change tools/instructions | no |
| `1.1.9..=1.1.17` | exact-id continuation child | any caller id | host-local only | unproved | unproved | unproved | no | no |
| `1.1.9..=1.1.17` | any | omission | n/a | none today | n/a | n/a | current path retained | n/a — omission is not a selection row |
| live `1.1.19` | any | any | empty on later probes | not qualified | not qualified | incident-only; not a qualified claim | not qualified | no |

No row is deliver-now. The empty set rests on authorized evidence:

1. host-local unstable listing — no bounded portable profile-id domain
2. official custom-agent authority risk — tools/instructions may change
3. missing selected `init.agent` fixture / confirmation path
4. unproved fail-closed invalid/unavailable/stale `--agent` semantics on the
   exact qualified range

It is not because `--agent` or `init.agent` are undocumented, and it is not
because the unauthorized live `--print` incidents prove qualified fail-open.

## Application, Failure, And Revision Posture

Requested, planned, dispatched, accepted, effective, and observed remain
distinct. Docs and help can request. Acceptance requires exact confirmation
and fail-closed invalid handling. Those remain unproved on the qualified
range.

No adapter-local `AntigravityAgentProfileId`, prepared input, plan constraint,
request member, argv emission, or init check is proposed. No behavior, driver,
claim, matrix, guide, contract, or configured-instance revision is proposed.
Omission keeps current production behavior.

## Promotion

Research 205 promotes an empty deliver-now set.

Cards 162-163 stay blocked. A later lane may reopen this family only when exact
qualified evidence shows:

1. invalid/unavailable/stale ids fail closed before accepted output on a
   version-pinned qualified binary
2. a bounded profile-id domain that does not require private profile bodies
3. exact `init.agent` equality on every selected child before output acceptance
4. composition that retains prepared resource, permission, tool, isolation, and
   continuation boundaries

Until then, `antigravity.headless` continues without `--agent`.
