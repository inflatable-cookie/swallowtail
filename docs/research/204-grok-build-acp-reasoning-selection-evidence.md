# 204 Grok Build ACP Reasoning-Selection Evidence

Status: promoted
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.057 / 158

## Question

Can exact `grok-build.acp` sessions and operation-private structured runs map
portable `ReasoningSelection` through Grok Build's ACP configuration channel,
with exact model/value/version qualification and confirmed effective selection
before the first prompt?

## Method And Boundary

Evidence was frozen on 2026-08-24 from current official public documentation,
exact published npm metadata, existing secret-free handshake fixtures, and
string inspection of already-downloaded exact darwin-arm64 binaries. No Grok
install, login, account inspection, credential capture, provider prompt,
external inference request, or paid work.

The selected operation remains `grok-build.acp`, driver
`swallowtail.grok-build.acp`, axis `grok-build.executable`, ACP v1 stdio:

- deprecated `0.2.114..=0.2.117` bind `grok-4.5`
- maintained `1.0.4..=1.0.5` bind `grok-4.6`
- mid-gap `0.2.118..=0.2.121` and `1.0.0..=1.0.3` stay incompatible
- later stable `UnverifiedNewer` may use only the latest qualified private
  mapping

Current Swallowtail spawn is `grok --no-auto-update agent stdio`. Interactive
`SessionOptions` must be empty. Structured runs expose no reasoning input.
`session/new` sends only `cwd` and an empty MCP server list. No
`session/set_config_option` request is sent.

Claude Agent and Kimi private `session/set_config_option` parsers are
implementation references, not Grok evidence.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [CLI reference](https://docs.x.ai/build/cli/reference) HTML | `--effort <LEVEL>` is a common CLI flag, not an ACP method | 2026-08-24 | `2f91e1c52e62dcf3d956e660d2a0fd486c43c7caee01f5d3db9143f632967eaf` |
| CLI reference markdown | same page as stable text | 2026-08-24 | `d6c944c885ac72a4f4d6036c1796537b795f805bff512c8a31a5f5f10932ee6e` |
| [Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting) HTML | official ACP example: `initialize` → `authenticate` → `session/new` `{cwd, mcpServers:[]}` → `session/prompt`; no effort field | 2026-08-24 | `a8f9412aeb9c0fb9573a55e466d08faec8fb6ad83462a94c2d97132ef69dc6ad` |
| Headless & Scripting markdown | same ACP example as stable text | 2026-08-24 | `a4f39daf25f81aba5dba79265d12d0e4ec444e6b28dcbe3c2335c05401052097` |
| [Changelog](https://x.ai/build/changelog) | 1.0.4 and 1.0.5 notes; no ACP effort-selection item | 2026-08-24 | tool-normalized markdown `50be72458da49580b86fc944e3967e9a2cc7dfe9b338818044e9b56d1959fd6b` |
| User-guide [Agent mode](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/15-agent-mode.md) | current public ACP `_meta` fields: `rules`, `systemPromptOverride`, `agentProfile`, `yoloMode`, `autoMode`; no effort | 2026-08-24 | `cb9ee253b86bdb1e1ac37d3e186bf0ff7d023e03ba428190508eaccb2a85d849` |
| User-guide [Configuration](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/05-configuration.md) | `GROK_CONFIG` overlay example `models.default_reasoning_effort`; not an ACP option | 2026-08-24 | `16615172b14e9aa152b9c46016c3a426aa2a13e13dd71979369556127f9d34cd` |
| Current public `reasoning_effort.rs` | later-source lead only: `_meta.reasoningEffort` on `session/new` with last-used/config/catalog fallback and ignore-on-unsupported | 2026-08-24 | `4c33d2916f8853004a766a0ae45f46f742c4193bbc08aee2d966bb6544e99b9e` |
| Current public `SOURCE_REV` | public GitHub tree is monorepo `437c7c928f3fcd13e9d37a51d887f41d7f84185d`, not 1.0.4 or 1.0.5 | 2026-08-24 | `abd59a1b2dfb5f9f1e85a4d08e43b867b4e5923aa78b36dd013f74d25c649926` |
| npm `@xai-official/grok@1.0.4` metadata | identity; gitHead `d846eb93d94d603191984d97f5d9f48170e93c6a` | 2026-08-24 | `618028b35b0f2d1d6b05abce3dd28d2da6339cac6d5acc63751467ef04055856` |
| npm `@xai-official/grok@1.0.5` metadata | identity; gitHead `5115b46bc909ae5c7f5fc064455197440e796b6b` | 2026-08-24 | `2bd9e85de16bdea2e2b44d414359a86cbb57777120fa02dcffdf345df72918fd` |
| local `grok-1.0.4-macos-aarch64` | exact maintained binary strings | 2026-08-24 | `39366f7756a090b735cc1df8c93a8c0c3c7871555cf6cbb28f9351ca82936485` |
| local `grok-1.0.5-macos-aarch64` | exact maintained binary strings | 2026-08-24 | `3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86` |
| local `grok-macos-aarch64` | exact `0.2.114` binary; SHA matches Research 070 | 2026-08-24 | `e715f57f9018a1737c1a64ef1cb260ac2a5045dfa6a1a0e1c7a7cbe193a083b2` |

Changelog HTML returned HTTP 403 from this environment. The changelog digest is
the tool-normalized markdown body retrieved the same day. It is not a
compatibility guarantee.

Public `xai-org/grok-build` is a later filtered export. It is a lead, not exact
`1.0.4` or `1.0.5` source. Exact version truth is the matching binaries plus
the existing no-prompt handshake corpus.

Npm launcher integrity matches Research 129/163 and the frozen identity
fixtures. Binaries were not executed.

## Advertisement Versus Selection

Existing secret-free handshakes record effort **labels in initialize
observation**, not a selectable ACP option snapshot:

| Version | Model | Observed efforts | Source |
| --- | --- | --- | --- |
| `0.2.114..=0.2.117` | `grok-4.5` | `high`, `medium`, `low` | Research 070/085; `grok-build-0.2.114-0.2.117/compatibility.json` |
| `1.0.4` | `grok-4.6` | `xhigh`, `high`, `medium`, `low` | Research 130; `grok-1-0-4/compatibility.json` |
| `1.0.5` | `grok-4.6` | `xhigh`, `high`, `medium`, `low` | Research 163; `grok-1-0-5/compatibility.json` |

Exact binaries embed `default_models.json` specimens with `reasoning_efforts`
objects `{value, label, description, default}`. `high` is marked `"default":
true`. Those are catalog rows, not ACP `configOptions`. Labels are not
protocol ids. Swallowtail's production initialize parser reads
`_meta.modelState` model ids only; it does not consume efforts.

Exact `0.2.114`, `1.0.4`, and `1.0.5` binaries contain **zero**
`session/set_config_option` or `set_config_option` strings. Concatenated
JSON-RPC method tables include `initialize`, `authenticate`, `session/new`,
`session/load`, `session/prompt`, `session/cancel`, and `session/set_model`.
`session/set_mode` appears as a separate string. `configOptions` and
`config_option_update` occur four times in each binary and match ACP schema
vocabulary, not a Grok selection implementation.

Official ACP documentation and the official stdio example never send an effort
field. `session/new` is documented as `cwd` plus `mcpServers`. Session `_meta`
currently documents permission/profile fields, not effort.

## Contract 034 Seam

Required sequence for a claimed negotiated option:

1. create or attach
2. receive one bounded option snapshot
3. identify one adapter-private option mapped to the portable value
4. require that value to be selectable
5. send one correlated `session/set_config_option`
6. require a response or update that confirms the effective value
7. return ready / first prompt only after confirmation

Exact qualified Grok ACP does not expose that seam.

Adjacent non-qualifying surfaces:

| Surface | Finding | Why it is not deliver-now |
| --- | --- | --- |
| CLI `--effort` / `--reasoning-effort` | documented for TUI/headless; changelog 0.2.89 makes the names aliases | not on `grok agent stdio`; copying child argv is forbidden |
| `/effort` slash command | TUI; changelog 0.2.82 | not ACP |
| `GROK_CONFIG` `models.default_reasoning_effort` | 1.0.5 binary and current config guide | generic overlay, default substitution, not a selected confirmation |
| initialize `reasoning_efforts` | advertised values | advertisement is not selectability |
| `session/set_model` effort override | 0.2.114 and 1.0.4 strings: `set_session_model: ignoring reasoning_effort override` | ignore, not confirmation; model switching is out of scope |
| current public `_meta.reasoningEffort` | later GitHub source: hint wins over last-used/config, else catalog default; unsupported values ignored | not exact 1.0.4/1.0.5; no snapshot; no `set_config_option`; fail-open ignore and default substitution |

## Version / Model / Value Disposition

| Version | Model | Value | Advertised | Selectable ACP option | Confirmed effective | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- |
| `0.2.114..=0.2.117` | `grok-4.5` | `low` | yes | no | no | no |
| `0.2.114..=0.2.117` | `grok-4.5` | `medium` | yes | no | no | no |
| `0.2.114..=0.2.117` | `grok-4.5` | `high` | yes; catalog default | no | no | no |
| `0.2.114..=0.2.117` | `grok-4.5` | `xhigh` | no | no | no | no |
| `1.0.4..=1.0.5` | `grok-4.6` | `low` | yes | no | no | no |
| `1.0.4..=1.0.5` | `grok-4.6` | `medium` | yes | no | no | no |
| `1.0.4..=1.0.5` | `grok-4.6` | `high` | yes; catalog default | no | no | no |
| `1.0.4..=1.0.5` | `grok-4.6` | `xhigh` | yes | no | no | no |
| any qualified | any | `off`, `minimal`, `max`, aliases | CLI headless docs name extra canonical levels | no | no | no |

No row is deliver-now.

## Lifecycle Disposition

| Lifecycle | Disposition |
| --- | --- |
| Interactive `session/new` | current wire: `cwd` + empty `mcpServers`; no config snapshot; no selection request; ready after `session/new` |
| Operation-private structured-run session | same `start_session` path, then one prompt; no reasoning input |
| Omission | retain current wire and behavior; do not infer `high` or any provider default as a selected value |
| Attachment recovery | empty options already required; new-session advertisement does not authorize mutation |
| Load / resume | advertised on 1.0.4/1.0.5; still unqualified; not implied by new-session evidence |
| `UnverifiedNewer` | no latest qualified private mapping to inherit; keep current omission path; fail closed on later snapshot/request/confirmation drift if a future mapping appears |

## Application, Failure, And Revision Posture

Advertised, selectable, requested, accepted, effective, and observed remain
distinct. Today only advertisement exists.

There is no adapter-private option id, category, or `session/set_config_option`
request shape to freeze. A response without a confirming snapshot is not
enough, because no selection request exists.

Post-allocation selection failure does not arise: Swallowtail never allocates a
session in order to set effort. Existing initialize / authenticate /
`session/new` failure still aborts before a ready handle, joins owned work, and
preserves provider-owned durable-session truth. That cleanup path is unchanged.

No behavior, driver, claim, or configured-instance revision is proposed. The
current public API stays empty `SessionOptions` and no run reasoning member.

## Promotion

Research 204 promotes an empty deliver-now set.

Cards 159-160 stay blocked. A later lane may reopen this family only when exact
qualified ACP evidence shows one bounded option snapshot, one selectable
private value, one correlated selection request, and effective confirmation
before readiness or first prompt, without default substitution, ignore-on-
unsupported, model switch, load/resume mutation, or a generic settings map.

CLI `--effort`, hosted xAI Responses `reasoning.effort`, UltraCode-style
aliases, and `GROK_CONFIG` overlays remain out of scope.
