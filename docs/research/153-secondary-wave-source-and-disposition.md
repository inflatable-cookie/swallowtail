# 153 Secondary Wave Source And Disposition

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 286

## Question

After the primary wave closed (086-091 accepted, 092 negative), which
secondary and watchlist candidates are ready for route evidence, which
need a new contract, and which should stay watchlist or deferred?

## Method

Reconciled Research 143/144/145 with the 2026-08-19 ACP registry snapshot,
official docs, GitHub/PyPI/npm latest metadata, and the current 45-route
matrix. Compared topology to existing OpenCode HTTP, Kimi/DeepSeek local
servers, and ACP stdio routes.

No executable was installed. No provider account, login, prompt, or live
session was used. Observed versions are not qualified claims. No production
matrix, package, or README count changed.

## Inventory

Current source stays 37 packages and 45 production routes. Watchlist names
do not appear as production rows.

Existing overlap that must not be flattened:

- OpenCode HTTP, Kimi local-server, and DeepSeek harness local-server already
  own attached loopback HTTP/WebSocket servers
- Contract 035 owns experimental remote ACP, not a proprietary REST/WS API
- `cline.acp` / `cline.headless` already cover the Cline-family ACP and
  NDJSON print shapes
- `pi.rpc` already covers Pi; community ACP wrappers stay rejected
- Qwen/Command Code/Antigravity/Vibe/Qoder already cover structured
  headless print routes

ACP registry membership remains discovery only.

## Observed official points

Observation timestamps, not Swallowtail bounds.

| Candidate | Official channel | Observed | Registry metadata (not a claim) |
| --- | --- | --- | --- |
| OpenHands Agent Server | PyPI `openhands-agent-server` | `1.42.1` (wheel 2026-08-12; SHA-256 `772a73b19684acab5f9f61b1c244f156052625ade51a5e48a424b3c13039f7a7`) | absent |
| Kiro CLI | docs + changelog | CLI `2.18.0` (2026-08-12); official `kiro-cli acp` docs dated 2026-08-18 | absent |
| Aider | PyPI `aider-chat` | `0.86.2` (2026-02-12) | absent |
| Deep Agents | npm `deepagents-acp` | `0.1.25` (2026-08-14; integrity `sha512-5S6Rpd74vV3YKVxAEqQkXKek+y1ChTpL0D2xf+WLaAYneJQZ9haZ4lPgjPy2VvszqErVsSr+T5tq8vdjuAWShQ==`; bin `deepagents-acp`) | `deepagents` `0.1.7` / `npx deepagents-acp@0.1.7` |
| Crush | Charm docs `crush run` | official non-interactive run exists; `crush-acp` is community `willbnu/crush-acp` | absent |
| Continue | npm `@continuedev/cli` | `1.5.47`; `cn -p` / `--format json` | absent |
| MiMo Code | npm `@xiaomi-mimo/cli` | `0.3.0-alpha.0` | absent |
| Kilo | npm `@kilocode/cli` | `7.4.22`; `kilo acp` / `kilo run` | `kilo@7.4.22`, args `acp` |
| Roo Code | — | no first-party ACP registry row | absent |

## Secondary dispositions

### 1. OpenHands — admit `openhands.agent-server` identity

Official package `openhands-agent-server` exposes HTTP plus WebSocket for
conversations, workspace files, and commands. Local start:

`python -m openhands.agent_server --host 127.0.0.1 --port 8000`

Health is `GET /health`. `/api/*` may require `X-Session-API-Key` from
`OH_SESSION_API_KEYS_*`. Default bind can be unauthenticated on loopback.
Conversation state lives under `workspace/`. `OH_SECRET_KEY` encrypts
stored LLM keys.

First Swallowtail topology is an **owned local loopback child**, one
bounded conversation, then join/kill that process. That matches existing
attached-server contracts used by OpenCode HTTP and DeepSeek
`local-server`. No new provider-neutral contract is required to start
identity.

Stay unmapped until a later card: Docker sandbox, hosted runtime API,
browser CORS, attaching to a caller-owned remote server, and OpenHands
ACP/Agent Canvas. Do not flatten onto Contract 035 remote ACP. Do not
embed the Python SDK as the route.

| Field | Value |
| --- | --- |
| Route | `openhands.agent-server` |
| Entrypoint | `python -m openhands.agent_server` on loopback |
| Axis (provisional) | `openhands-agent-server.package` |
| Package (provisional) | `swallowtail-adapter-openhands` |
| Topology | owned loopback HTTP/WS child |
| Access | optional session API key; Swallowtail does not mint keys or log in |
| Cleanup | join/kill the owned server; disconnect-only is not the first topology |
| First useful op | health/ready, one bounded conversation, joined cleanup |
| Identity card | 287 |

### 2. Kiro — admit `kiro.acp`; defer `kiro.headless`

Research 143 named `kiro.headless`. Official docs now document first-party
ACP: `kiro-cli acp` over stdio JSON-RPC 2.0 (`initialize`, `session/new`,
`session/prompt`, `session/cancel`, advertised `loadSession` and image
prompts). That is the Swallowtail-shaped first route. Registry still has
no Kiro row; registry absence is not a stop.

Headless remains `kiro-cli chat --no-interactive` plus `KIRO_API_KEY`.
It is text-to-stdout, wants `--trust-all-tools` / `--trust-tools`, and
the 3.0 docs replace trust flags with `permissions.yaml` while changelog
latest is CLI `2.18.0`. Keep headless as a later sibling, not this
roadmap's first op. Cloud `--cloud` sessions stay unmapped.

g03.094 retargets from `kiro.headless` to `kiro.acp`.

| Field | Value |
| --- | --- |
| Route | `kiro.acp` |
| Entrypoint | `kiro-cli acp` |
| Axis (provisional) | `kiro-cli.release` |
| Package (provisional) | `swallowtail-adapter-kiro` |
| Topology | owned stdio child |
| Access | provider-owned Kiro login or `KIRO_API_KEY`; Swallowtail does not log in |
| Cleanup | join/kill the ACP child |
| First useful op | ACP initialize + one bounded prompt |
| Identity card | 291 |
| Deferred sibling | `kiro.headless` |

### 3. Aider — defer `aider.headless`

Official scripting is still `aider --message` / `--message-file`: one
natural-language instruction, optional `--yes`, default auto-commits, then
exit. Python `Coder` API is explicitly unsupported. No structured event
protocol. PyPI `0.86.2` last uploaded 2026-02-12.

That is a maintained CLI, but it duplicates text/Git mutation without
adding a Swallowtail event or lifecycle shape. Do not start cards 295-298
until official JSON/NDJSON or an equivalent bounded event surface exists.
No new contract is named because the route is not admitted.

### 4. Deep Agents — admit `deepagents.acp`

The executable/package boundary Research 143 asked for now exists:
first-party LangChain npm `deepagents-acp`, bin `deepagents-acp`,
`npx deepagents-acp`, stdio ACP. npm `latest` is `0.1.25`; registry still
pins `0.1.7`. Bind the CLI, not the library `DeepAgentsServer` embed and
not a custom `tsx` script.

Default model expects `ANTHROPIC_API_KEY`. HITL `interruptOn`, slash
commands, and multi-agent programmatic config stay adapter-private.

| Field | Value |
| --- | --- |
| Route | `deepagents.acp` |
| Entrypoint | `deepagents-acp` / `npx deepagents-acp` |
| Axis (provisional) | `deepagents-acp.package` |
| Package (provisional) | `swallowtail-adapter-deepagents` |
| Topology | owned stdio child |
| Access | provider API key env; Swallowtail does not log in |
| Cleanup | join/kill the ACP child |
| First useful op | ACP initialize + one bounded prompt |
| Identity card | 299 |

## Watchlist dispositions (closed by Research 158 / card 303)

Intake placeholders. Binding add/defer/reject rows live in Research 158.

| Candidate | Disposition | Why | Revisit when |
| --- | --- | --- | --- |
| Crush | watchlist | Official `crush run` is non-interactive text. `crush-acp` is a community wrapper that spawns `crush run`. An in-tree `crush acp` PR is not a released official wire. | Charm ships maintained `crush acp` or structured `crush run` events in a stable release |
| Continue | watchlist | `cn -p` / `--format json` is real headless; also `cn serve` and browser login. Overlaps existing print routes; account/platform coupling. | first-party ACP or a distinct attached-server identity card is justified |
| MiMo Code | watchlist | npm `@xiaomi-mimo/cli@0.3.0-alpha.0`; OpenCode-family TUI/CLI; GPL-2.0. Alpha is not a production-route source. | stable non-alpha release with a distinct wire from OpenCode |
| Kilo | watchlist | Official `kilo acp` at `@kilocode/cli@7.4.22` is in the registry, but the family overlaps Cline/OpenCode. Do not add a third Cline-shaped ACP from registry presence. | identity proves a material wire/lifecycle divergence from `cline.acp` and OpenCode |
| Roo Code | watchlist | No ACP registry row; Cline-family IDE/CLI overlap. | first-party machine-facing surface distinct from Cline |
| Amp | watchlist / likely reject wrapper | Registry `amp-acp` is community `tao12345666333/amp-acp`, same collapse class as `pi-acp`. | official Amp native ACP |
| Auggie, CodeBuddy, Cortex, Devin, Factory Droid, Junie, GLM Agent, others | watchlist | Registry discovery only. | transport, authority, install, and lifecycle evidence strong enough for a named roadmap |

None of these names appear in the production route matrix.

## Contract fit

No new provider-neutral contract is required before card 287.

| Route | First op | Shape | Existing contracts | Distinct from |
| --- | --- | --- | --- | --- |
| `openhands.agent-server` | owned loopback server + one bounded conversation | attached local HTTP/WS child | 009-010, 014 (loopback grant), 023, 029, 032-033, 044, 051 | Contract 035 remote ACP; Docker/hosted sandbox; OpenHands ACP canvas |
| `kiro.acp` | ACP initialize + one bounded prompt | ACP stdio child | 015 + 005-006, 009-010, 023, 029, 032-033, 039-041, 044, 051 | `kiro.headless`; `--cloud` |
| `deepagents.acp` | ACP initialize + one bounded prompt | ACP stdio child | 015 + same host/lifecycle set | library embed; custom `tsx` server |
| `aider.headless` | none | deferred | no new contract | text/Git one-shot |

Adapter-private, not new contracts: OpenHands session API keys and
`OH_SECRET_KEY`; Kiro `_kiro.dev/*` extensions and advertised
`loadSession` (do not inherit continuation recovery); Deep Agents HITL
and slash commands.

## Secondary order after this gate

1. card 287 `openhands.agent-server` identity
2. card 291 `kiro.acp` identity after g03.093 closeout, including negative
3. card 299 `deepagents.acp` identity after g03.094 closeout, including
   negative
4. card 303 closed the watchlist in Research 158; add none
5. `aider.headless` and `kiro.headless` stay unstarted

## Non-goals

- qualifying any version range
- wrapping `crush-acp`, `pi-acp`, or `amp-acp`
- installing, login, live inference, matrix edits, README count changes
- promoting Docker/hosted OpenHands, Kiro cloud sessions, or Kilo/Roo
  merely because they are popular
