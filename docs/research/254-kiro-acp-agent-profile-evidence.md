# 254 Kiro ACP Agent-Profile Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.090 / 257

## Question

Which exact `kiro.acp` `2.18.1` `--agent` profile rows, if any, can bind with
closed membership, authority, application, confirmation, failure, lifecycle,
and omission truth?

## Decision

No. Research 254 admits an empty deliver-now set. No typed agent-profile
binding is admitted on `kiro.acp` at exact `2.18.1`.

Official ACP documentation advertises optional
`kiro-cli acp --agent my-agent`. The name is an arbitrary documentation
placeholder, not a closed portable membership set. Built-in ids
(`kiro_default`, `kiro_help`, `kiro_planner`) and custom agent files under
`.kiro/agents/` / `~/.kiro/agents/` belong to interactive and custom-agent
surfaces. Official troubleshooting documents missing-agent behavior as
fallback to the default agent without warning. Initialize `agentInfo`
returns product identity `kiro-cli`, not an applied profile. Exact
`2.18.1` package/source parser bytes were not recoverable from the
official CDN on 2026-08-28 (platform archives return HTTP 403; current
stable installer manifest is `2.20.1`).

Do not promote chat `/agent`, global or chat `--agent`, ambient host
profiles, or unsupported `session/set_mode` onto ACP. Do not create or
mutate a host profile to manufacture a row.

## Method And Boundary

Official ACP, CLI-commands, slash-commands, custom-agents overview,
creating, troubleshooting, built-in, and configuration-reference pages
were retrieved as markdown on 2026-08-28 and digested. The ACP HTML
render was also retrieved for contrast with Research 251. Research 156
identity digests, fixture corpus `kiro-acp-2.18.1`, guide
`docs/guides/kiro-acp-prepared-integration.md`, and adapter argv
`["acp"]` were reconciled. Current stable installer manifest and install
script were retrieved for channel currentness contrast only.

No Kiro install, host `PATH` mutation, login, credential or account
inspection, ACP initialize, `session/new`, provider prompt, paid work,
`kiro-cli agent create|edit|set-default`, or ambient `~/.kiro/agents`
write was used. Platform archives for both frozen `2.18.1` and current
`2.20.1` returned HTTP 403 on ranged GET; no binary was extracted or
executed. Host PATH had no `kiro-cli`. Ambient `~/.kiro/agents` was
absent; its absence was not treated as membership evidence.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [ACP (md)](https://kiro.dev/docs/cli/acp.md) | production entrypoint; optional `--agent my-agent`; `session/set_mode`; `agentInfo.name` = `kiro-cli` | 2026-08-28 | `50e8f6285b5fbc46feb5afa34a7bc8b8fa3fff95ae926ab959f6ce0253745ab0` |
| [ACP (html)](https://kiro.dev/docs/cli/acp/) (page updated 2026-08-04) | same page rendered; HTML digest differs from Research 251 snapshot | 2026-08-28 | `f04096a28b437375f8a80792fd5a23312d44a78e35e9a95ce8e571fd74cb7a27` |
| [CLI commands (md)](https://kiro.dev/docs/reference/cli-commands.md) | global and `chat --agent`; `kiro-cli agent list|create|set-default`; no `kiro-cli acp` section | 2026-08-28 | `c944abed46990077457ba2a809a8542028f5c0613daa3b0541cf901367c941a2` |
| [Slash commands (md)](https://kiro.dev/docs/reference/slash-commands.md) | `/agent` family; built-in ids `kiro_default\|kiro_help\|kiro_planner`; chat `--agent` | 2026-08-28 | `c3a604b7ea3e10b61aa082cde5219cc33d79fd1a0d3f4cf607a6582cd643402a` |
| [Custom agents (md)](https://kiro.dev/docs/custom-agents.md) | workspace `.kiro/agents/` and global `~/.kiro/agents/` namespaces | 2026-08-28 | `88caf34d827cc86c91ae609bdb4e583e1f52133755bc360df5f80fc943b37f27` |
| [Creating (md)](https://kiro.dev/docs/custom-agents/creating.md) | local-then-global precedence; default named `kiro_default` | 2026-08-28 | `74d4fa17750b380be3d4526508576946576bb792b76b903d5276a3f76b17c5cd` |
| [Troubleshooting (md)](https://kiro.dev/docs/custom-agents/troubleshooting.md) | missing agent falls back to default without warning | 2026-08-28 | `502ed22561f5c93ab2f4f329e99459e3bb746917743271739aab2f1874b2e8d7` |
| [Built-in (md)](https://kiro.dev/docs/custom-agents/built-in.md) | built-in display roster; example spawn is `kiro-cli --agent`, not closed ACP argv | 2026-08-28 | `626f170bc1de5d4e6872811e8cb8f59a45f783198a3adbbc10e04621fb8859ae` |
| [Config reference (md)](https://kiro.dev/docs/custom-agents/configuration-reference.md) | `tools` / `allowedTools` / `permissions`; authority composition | 2026-08-28 | `80484017b11180fe9e27f9ee5f71c0e8812afa53591793d162bfb7027b99a4c3` |
| Research 156 identity | exact axis `kiro-cli.release` `2.18.1`; entrypoint `kiro-cli acp`; `pass_agent_flag: false` | 2026-08-19 | see Research 156 |
| Fixture `identity.json` / `protocol.json` / `negative-cases.json` | production omission; `--agent` recorded as unmapped; `agentInfo` is product identity | 2026-08-19 | corpus-local |
| Installer script `https://cli.kiro.dev/install` | channel base `prod.download.cli.kiro.dev/stable/latest` | 2026-08-28 | `91a21bfa05cd7b58601cb83e0f1f187a9d0084726e5b824d4a4cf60306250908` |
| Current stable manifest | channel tip `2.20.1`; contrast only; not the qualified route | 2026-08-28 | `f96d0134f48cb623543b5c9d129f0f5e17a2ce2da25a7129f7ebc6bc5e51a996` |

Markdown digests identify retrieved documentation bodies. They are not a
compatibility guarantee and do not substitute for unrecovered `2.18.1`
package parser bytes.

Lane-local frozen summary:
`crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.18.1-agent-profile-evidence/`.

## Official Surface Separation

| Surface | Agent-profile finding | ACP production relevance |
| --- | --- | --- |
| `kiro-cli acp` docs | spawn examples `kiro-cli acp` and `kiro-cli acp --agent my-agent`; JetBrains/Zed examples use `["acp"]` only | production entrypoint; current Swallowtail argv omits `--agent` |
| CLI commands reference | `--agent` under global args and `kiro-cli chat`; `kiro-cli agent` management; no `kiro-cli acp` section | management and chat family; not closed ACP membership |
| `/agent` slash command | list/create/swap/set-default; built-in ids named; ambient storage paths | interactive / `_kiro.dev/commands/*` lead only; extensions unmapped |
| Custom agents | user-owned files under `.kiro/agents/` and `~/.kiro/agents/`; local precedes global | unbounded ambient membership; out of Swallowtail authority |
| Built-in agents page | display roster (Default, Spec, Plan, Help, …); slash ids elsewhere | no closed ACP argv membership table for those display names |
| ACP `initialize` | docs `agentInfo` = `{name: kiro-cli, version: …}` | product identity; not selected profile confirmation |
| ACP `session/set_mode` | listed as switching agent configs | unsupported on first driver; must not invent profile binding |
| Missing-agent troubleshooting | fallback to default without warning | fails the pre-prompt rejection gate |

## Truth Layers

| Layer | Exact finding |
| --- | --- |
| Requested | no Swallowtail agent-profile request exists on `kiro.acp` |
| Parsed | ACP docs document optional `--agent <string>`; exact `2.18.1` binary parser unrecovered |
| Configured | ambient agent files and `set-default` may exist under host `~/.kiro/`; Swallowtail does not read or set them |
| Dispatched | production spawn remains exactly `kiro-cli` + `acp` |
| Accepted | no fail-closed ACP accept/reject seam; official missing-agent docs describe silent default fallback |
| Effective | no confirmable effective profile without applied-profile confirmation |
| Returned | `agentInfo` is `kiro-cli`; no returned selected-profile field closed |
| Observed | no live observation in this lane |
| Persisted | official agent configs and ACP sessions are provider-owned; out of Swallowtail authority |

## Profile Disposition

| Candidate | Docs surface | Membership | Failure | Applied confirmation | Deliver-now |
| --- | --- | --- | --- | --- | --- |
| `my-agent` | ACP placeholder example | not a portable member | unproved on ACP | none | withheld |
| `kiro_default` | slash / creating default name | interactive built-in id; not ACP argv table | silent fallback risk | none on ACP | withheld |
| `kiro_help` / `kiro_planner` | slash built-in ids | same | same | none | withheld |
| built-in display names | built-in agents page | display roster only | same | none | withheld |
| arbitrary custom name | `.kiro/agents/` / `~/.kiro/agents/` | host-ambient unbounded | silent fallback | none | withheld |
| omitted `--agent` | current Swallowtail argv | n/a | n/a | host-owned default remains host-owned | retain absent path |

No candidate is deliver-now.

## Package/Source Gate

Research 156 recorded platform digests for `2.18.1/Kiro CLI.dmg` and
`2.18.1/kirocli-aarch64-linux.tar.xz` without extraction. On 2026-08-28,
ranged GET of those `stable/latest/2.18.1/…` URLs and of current
`2.20.1/…` archives returned HTTP 403. Manifest JSON remains readable and
now names `2.20.1`.

Therefore this lane cannot freeze an exact `2.18.1` clap/help/parser
specimen for `acp --agent` rejection or application bytes. Official ACP
documentation remains the production-route advertisement and does not
close membership, failure, or confirmation. Chat and custom-agent
documentation alone are insufficient to bind ACP.

## Lifecycle And Omission

| Item | Finding |
| --- | --- |
| Before ACP startup | optional `--agent` advertised; no closed membership or reject path |
| Session open | `session/new` carries cwd and MCP servers only |
| Mode/profile surface | `session/set_mode` unsupported; must not invent confirmation |
| Confirmation | unavailable; `agentInfo` is product identity |
| Failure before provider effects | missing names are documented to fall back without warning |
| Cleanup | unchanged join/cancel path; no profile-specific cleanup |
| Omission | retain exact `["acp"]`; matches guide, `command.rs`, and negative corpus |

## Promotion

Research 254 promotes no deliver-now Kiro ACP agent-profile row.

Card 257 is complete as an honest empty set. No production binding card
starts from this lane. A later lane may reopen only with:

1. exact recoverable `kiro-cli.release` package/source parser for the
   qualified version proving ACP `--agent` parse, reject, and application;
2. closed portable membership that does not depend on host ambient files
   or account inspection;
3. pre-prompt reject-closed invalid-name behavior on the production ACP
   route (no silent default fallback);
4. confirmable applied-profile return or update that is not generic
   `agentInfo`;
5. no promotion of chat `/agent`, unsupported `session/set_mode`, or
   `_kiro.dev/*` extensions by implication.

## Non-goals

- production code, public API, guide, or shared matrix edits
- `kiro.headless`, `--cloud`, effort, trust-all tools, model routing
- install, login, credentials, provider prompts, paid work, host mutation
- profile create/edit/set-default to force a passing row
- currentness bump from `2.18.1` to `2.20.1`
