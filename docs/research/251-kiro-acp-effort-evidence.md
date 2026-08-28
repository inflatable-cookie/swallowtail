# 251 Kiro ACP Effort Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.089 / 254

## Question

Which exact `kiro.acp` `2.18.1` model, effort value, and lifecycle rows, if
any, can bind `--effort` with closed ACP application, confirmation, failure,
and omission truth?

## Decision

No. Research 251 admits an empty deliver-now set. No typed effort binding is
admitted on `kiro.acp` at exact `2.18.1`.

Official ACP documentation for the production entrypoint documents
`kiro-cli acp` and optional `--agent` only. It does not document `--effort`.
Official `--effort low|medium|high|xhigh|max` is documented under
`kiro-cli chat`, with `/effort` as an interactive slash command. Effort
membership depends on the active model. The production route selects no
model, does not map `session/set_model`, and does not map `_kiro.dev/*`
extensions. Exact `2.18.1` package/source parser bytes were not recoverable
from the official CDN on 2026-08-28 (platform archives return HTTP 403;
current stable installer manifest is `2.20.1`).

Do not promote chat/interactive effort onto ACP. Do not infer ACP support
from unsupported `session/set_model` or from third-party live probes.

## Method And Boundary

Official Kiro ACP, reasoning-effort, CLI-commands, and slash-commands pages
were retrieved on 2026-08-28 and digested. Research 156 identity digests,
fixture corpus `kiro-acp-2.18.1`, guide
`docs/guides/kiro-acp-prepared-integration.md`, and adapter argv
`["acp"]` were reconciled. Current stable installer manifest and install
script were retrieved for channel currentness contrast only.

No Kiro install, host `PATH` mutation, login, credential or account
inspection, ACP initialize, `session/new`, provider prompt, paid work, or
ambient `~/.kiro/` write was used. Platform archives for both frozen
`2.18.1` and current `2.20.1` returned HTTP 403 on ranged GET; no binary
was extracted or executed.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [ACP](https://kiro.dev/docs/cli/acp/) (page updated 2026-08-04) | production entrypoint `kiro-cli acp`; optional `--agent`; method list including `session/set_model`; `_kiro.dev/commands/*`; no `--effort` | 2026-08-28 | `f9f2b1a99592e132eb9cb83dac460346e76c5f45dfee3ffa876f102144fd474b` |
| [Reasoning effort](https://kiro.dev/docs/models/effort/) (page updated 2026-08-04) | official levels `low\|medium\|high\|xhigh\|max`; model-dependent membership; precedence including `--effort` / `/effort`; persistence under `~/.kiro/settings/cli.json` | 2026-08-28 | `f07ff2b477a91dd6d59c24f48e73b483d2b926d8a300090e6b2d1b112b30c109` |
| [CLI commands](https://kiro.dev/docs/reference/cli-commands/) (page updated 2026-08-04) | `--effort <LEVEL>` under `kiro-cli chat`; example `kiro-cli chat --effort high "..."` | 2026-08-28 | `12050b3e31addf40e478b276457aebb7a36062b0e24dfb48c53717071a6b11ff` |
| [Slash commands](https://kiro.dev/docs/reference/slash-commands/) (page updated 2026-08-28) | `/effort`; levels depend on active model; persistence | 2026-08-28 | `5a102970c51ecfad3cdefce1fcbc069770e47d0547bdd450f6680effc9604f19` |
| Research 156 identity | exact axis `kiro-cli.release` `2.18.1`; entrypoint `kiro-cli acp`; unmapped `session/set_model` and `_kiro.dev/*` | 2026-08-19 | see Research 156 |
| Fixture `identity.json` | recorded `2.18.1` DMG / linux tar.xz digests; archives not extracted | 2026-08-19 | corpus-local |
| Installer script `https://cli.kiro.dev/install` | channel base `prod.download.cli.kiro.dev/stable/latest` | 2026-08-28 | `91a21bfa05cd7b58601cb83e0f1f187a9d0084726e5b824d4a4cf60306250908` |
| Current stable manifest | channel tip `2.20.1`; contrast only; not the qualified route | 2026-08-28 | `f96d0134f48cb623543b5c9d129f0f5e17a2ce2da25a7129f7ebc6bc5e51a996` |

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee and do not substitute for unrecovered `2.18.1`
package parser bytes.

Lane-local frozen summary:
`crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.18.1-effort-evidence/`.

## Official Surface Separation

| Surface | Effort finding | ACP production relevance |
| --- | --- | --- |
| `kiro-cli acp` docs | spawn examples `kiro-cli acp` and `kiro-cli acp --agent …`; zero `--effort` occurrences in retrieved ACP HTML | production entrypoint; current Swallowtail argv |
| `kiro-cli chat` docs | `--effort <LEVEL>` with values `low\|medium\|high\|xhigh\|max`; example is chat argv | deferred `kiro.headless` / interactive family; not this route |
| `/effort` slash command | session override; levels depend on active model; persists to settings | interactive / `_kiro.dev/commands/execute` lead only; extensions unmapped |
| ACP `session/new` | docs params `{cwd, mcpServers}`; no effort field | no session-open effort byte |
| ACP `session/set_model` | listed as model change | unsupported on first driver; must not invent effort binding |
| `_kiro.dev/commands/execute` | docs example names `/agent swap`, `/context add` | unmapped; `/effort` via this extension is not production truth |

## Truth Layers

| Layer | Exact finding |
| --- | --- |
| Requested | no Swallowtail effort request exists on `kiro.acp` |
| Parsed | official chat parser documents five named levels; ACP docs do not document an ACP effort parse path; exact `2.18.1` binary parser unrecovered |
| Configured | ambient `chat.modelDefaults` / persisted `/effort` may exist in host `~/.kiro/`; Swallowtail does not read or set them |
| Dispatched | production spawn remains exactly `kiro-cli` + `acp` |
| Accepted | no ACP accept/reject seam closed for effort values on this route |
| Effective | no confirmable effective effort without model membership and provider observation |
| Returned | no returned ACP config/status field closed for effort |
| Observed | no live observation in this lane |
| Persisted | official docs persist effort under provider-owned settings; out of Swallowtail authority |

## Value Disposition

| Candidate | Chat CLI docs | ACP docs / session bytes | Model membership | Contract 040 / deliver-now |
| --- | --- | --- | --- | --- |
| `low` | documented chat `--effort` | not on ACP spawn docs; not in `session/new` | depends on active model | withheld |
| `medium` | documented | same | same | withheld |
| `high` | documented; chat example | same | same | withheld |
| `xhigh` | documented | same | same | withheld |
| `max` | documented | same | same | withheld |
| omitted | current Swallowtail argv | exact production omission | n/a | retain absent path |

No candidate is deliver-now.

## Package/Source Gate

Research 156 recorded platform digests for `2.18.1/Kiro CLI.dmg` and
`2.18.1/kirocli-aarch64-linux.tar.xz` without extraction. On 2026-08-28,
ranged GET of those `stable/latest/2.18.1/…` URLs and of current
`2.20.1/…` archives returned HTTP 403. Manifest JSON remains readable and
now names `2.20.1`.

Therefore this lane cannot freeze an exact `2.18.1` clap/help/parser
specimen proving or disproving `acp --effort` as argv. Official ACP
documentation remains the production-route authority and does not document
that flag. Chat documentation alone is insufficient to bind ACP.

## Lifecycle And Omission

| Item | Finding |
| --- | --- |
| Before ACP startup | no documented ACP effort argv on the production page |
| Session open | `session/new` carries cwd and MCP servers only |
| Model-selection surface | `session/set_model` unsupported; no effort field recovered |
| Confirmation | unavailable without mapped accept/return bytes |
| Failure before provider effects | unsupported values cannot be proved reject-closed without package/live seams |
| Cleanup | unchanged join/cancel path; no effort-specific cleanup |
| Omission | retain exact `["acp"]`; matches guide and `command.rs` |

## Promotion

Research 251 promotes no deliver-now Kiro ACP effort row.

Card 254 is complete as an honest empty set. No production binding card
starts from this lane. A later lane may reopen only with:

1. exact recoverable `kiro-cli.release` package/source parser for the
   qualified version proving ACP spawn or session application;
2. closed model/value membership without live account inference;
3. confirmable accept/return/omission seams on the production ACP route;
4. no promotion of chat flags, unsupported `session/set_model`, or
   `_kiro.dev/*` extensions by implication.

## Non-goals

- production code, public API, guide, or shared matrix edits
- `kiro.headless`, `--cloud`, `--agent`, trust-all tools, model routing
- install, login, credentials, provider prompts, paid work, host mutation
- currentness bump from `2.18.1` to `2.20.1`
