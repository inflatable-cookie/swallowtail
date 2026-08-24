# 204 Grok Build ACP Reasoning-Selection Evidence

Status: promoted
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.057 / 158
Correction: 2026-08-24 split exact 1.0.4 from 1.0.5 open-time hint evidence;
classify exact `x.ai/sessionConfig` response channel without inferring option
contents from later source

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
- maintained `1.0.4` and `1.0.5` both bind `grok-4.6` but differ on new-session
  effort application
- mid-gap `0.2.118..=0.2.121` and `1.0.0..=1.0.3` stay incompatible
- later stable `UnverifiedNewer` may use only the latest qualified private
  mapping

Current Swallowtail spawn is `grok --no-auto-update agent stdio`. Interactive
`SessionOptions` must be empty. Structured runs expose no reasoning input.
`session/new` sends only `cwd` and an empty MCP server list. No
`session/set_config_option` request is sent.

Claude Agent and Kimi private `session/set_config_option` parsers are
implementation references, not Grok evidence.

Public `xai-org/grok-build` is a later filtered export (`SOURCE_REV`
`437c7c928f3fcd13e9d37a51d887f41d7f84185d`, not 1.0.4 gitHead `d846eb93` or
1.0.5 gitHead `5115b46b`). Exact version truth is the matching binaries plus
the existing no-prompt handshake corpus. Later GitHub files are cited only
where their strings match those binaries.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [CLI reference](https://docs.x.ai/build/cli/reference) HTML | `--effort <LEVEL>` is a common CLI flag, not an ACP method | 2026-08-24 | `2f91e1c52e62dcf3d956e660d2a0fd486c43c7caee01f5d3db9143f632967eaf` |
| [CLI reference markdown](https://docs.x.ai/build/cli/reference.md) | same page as stable text | 2026-08-24 | `d6c944c885ac72a4f4d6036c1796537b795f805bff512c8a31a5f5f10932ee6e` |
| [Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting) HTML | official ACP example: `initialize` → `authenticate` → `session/new` `{cwd, mcpServers:[]}` → `session/prompt`; no effort field | 2026-08-24 | `a8f9412aeb9c0fb9573a55e466d08faec8fb6ad83462a94c2d97132ef69dc6ad` |
| [Headless & Scripting markdown](https://docs.x.ai/build/cli/headless-scripting.md) | same ACP example as stable text | 2026-08-24 | `a4f39daf25f81aba5dba79265d12d0e4ec444e6b28dcbe3c2335c05401052097` |
| [Changelog](https://x.ai/build/changelog) | 1.0.4 and 1.0.5 notes; no ACP effort-selection item | 2026-08-24 | tool-normalized markdown `50be72458da49580b86fc944e3967e9a2cc7dfe9b338818044e9b56d1959fd6b` |
| [Agent mode](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/15-agent-mode.md) | current public ACP `_meta`: `rules`, `systemPromptOverride`, `agentProfile`, `yoloMode`, `autoMode`; no effort | 2026-08-24 | `cb9ee253b86bdb1e1ac37d3e186bf0ff7d023e03ba428190508eaccb2a85d849` |
| [Configuration](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/05-configuration.md) | `GROK_CONFIG` overlay example `models.default_reasoning_effort`; not an ACP option | 2026-08-24 | `16615172b14e9aa152b9c46016c3a426aa2a13e13dd71979369556127f9d34cd` |
| [SOURCE_REV](https://raw.githubusercontent.com/xai-org/grok-build/main/SOURCE_REV) | public tree monorepo SHA `437c7c928f3fcd13e9d37a51d887f41d7f84185d`; later than 1.0.4/1.0.5 | 2026-08-24 | `abd59a1b2dfb5f9f1e85a4d08e43b867b4e5923aa78b36dd013f74d25c649926` |
| [reasoning_effort.rs](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-shell/src/agent/mvp_agent/reasoning_effort.rs) | later-source corroboration of exact 1.0.5 binary strings; not 1.0.4 | 2026-08-24 | `4c33d2916f8853004a766a0ae45f46f742c4193bbc08aee2d966bb6544e99b9e` |
| [types.rs](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-sampling-types/src/types.rs) | later-source corroboration of `meta.reasoningEffort` parser strings present in both 1.0.4 and 1.0.5 binaries | 2026-08-24 | `f5942d9565959a72020bf81b0ae21aeb6cbdb1b81d2ff1f9ddcf2672e3406ec2` |
| npm `@xai-official/grok@1.0.4` metadata | identity; gitHead `d846eb93d94d603191984d97f5d9f48170e93c6a` | 2026-08-24 | `618028b35b0f2d1d6b05abce3dd28d2da6339cac6d5acc63751467ef04055856` |
| npm `@xai-official/grok@1.0.5` metadata | identity; gitHead `5115b46bc909ae5c7f5fc064455197440e796b6b` | 2026-08-24 | `2bd9e85de16bdea2e2b44d414359a86cbb57777120fa02dcffdf345df72918fd` |
| local `grok-1.0.4-macos-aarch64` | exact 1.0.4 strings | 2026-08-24 | `39366f7756a090b735cc1df8c93a8c0c3c7871555cf6cbb28f9351ca82936485` |
| local `grok-1.0.5-macos-aarch64` | exact 1.0.5 strings | 2026-08-24 | `3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86` |
| local `grok-macos-aarch64` | exact `0.2.114`; SHA matches Research 070 | 2026-08-24 | `e715f57f9018a1737c1a64ef1cb260ac2a5045dfa6a1a0e1c7a7cbe193a083b2` |

Changelog HTML returned HTTP 403 from this environment. The changelog digest is
the tool-normalized markdown body retrieved the same day.

Npm launcher integrity matches Research 129/163 and the frozen identity
fixtures. Binaries were not executed.

## Advertisement Versus Selection

Existing secret-free handshakes record effort **values in initialize
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
`configOptions` and `config_option_update` occur four times in each binary and
match ACP schema vocabulary, not a Grok selection implementation.

Official ACP documentation and the official stdio example never send an effort
field. `session/new` is documented as `cwd` plus `mcpServers`. Session `_meta`
currently documents permission/profile fields, not effort. A planning triage
note attributed "ACP clients can specify reasoning effort when opening or
resuming" to the 1.0.x changelog; the frozen changelog pages do not contain
that sentence.

## Exact 1.0.4 Versus 1.0.5 Effort Path

Both binaries parse ACP `_meta.reasoningEffort` in
`crates/codegen/xai-grok-sampling-types/src/types.rs`:

- `meta.reasoningEffort: expected string, ignoring`
- `meta.reasoningEffort: parse failed, ignoring`

Malformed or unknown tokens are ignored. That parser is not by itself
new-session application.

### 1.0.4

No `crates/codegen/xai-grok-shell/src/agent/mvp_agent/reasoning_effort.rs`.
No `reasoning_effort: applied effort`. No `reasoning_effort: model does not
support effort; ignoring it`. No `apply_supported_effort`.

Effort application strings are on `session/set_model`:

- `set_session_model: applying reasoning_effort override from meta`
- `set_session_model: ignoring reasoning_effort override`

Swallowtail does not send `session/set_model` and does not switch models.
1.0.4 therefore has parser vocabulary without a new-session apply path.

### 1.0.5

The exact 1.0.5 binary contains
`crates/codegen/xai-grok-shell/src/agent/mvp_agent/reasoning_effort.rs`
(debug events at `:42`, `:67`, `:69`) and:

- `reasoning_effort: applied effort`
- `reasoning_effort: model does not support effort; ignoring it`
- application targets `new_session` and `model_switch`
- symbol `apply_supported_effort`

It no longer contains `set_session_model: ignoring reasoning_effort override`.

Later public
[`reasoning_effort.rs`](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-shell/src/agent/mvp_agent/reasoning_effort.rs)
repeats those exact log strings. It is corroboration, not 1.0.5 source. That
later file also states: `_meta.reasoningEffort` wins over last-used /
`[models].default_reasoning_effort`; catalog default is last resort;
unsupported values are ignored.

1.0.5 therefore accepts a request-dispatchable open-time hint on
`session/new` `_meta.reasoningEffort`. Handshake-advertised `grok-4.6` values
remain `low|medium|high|xhigh`. Later `types.rs` `FromStr` also names
`none|minimal|max`; unknown strings are ignored. Those extra enum names are
not handshake-advertised on this route and are not deliver-now.

`session/new` success and an internal tracing log are not confirmation.

## Exact `x.ai/sessionConfig` Response Channel

Both maintained binaries insert vendor session-new metadata. Exact strings
are concatenated in `agent_ops.rs` rodata:

`x.ai/sessionConfig` `options` `x.ai/sessionDetail` `x.ai/schedulerBackgroundLoops`

| Version | Binary SHA-256 | `x.ai/sessionConfig` offset | `insert_session_config_meta` |
| --- | --- | --- | --- |
| `1.0.4` | `39366f7756a090b735cc1df8c93a8c0c3c7871555cf6cbb28f9351ca82936485` | `107073191` | present |
| `1.0.5` | `3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86` | `107433096` | present |

Frozen shape from those exact strings: `_meta["x.ai/sessionConfig"].options`,
plus sibling vendor keys `x.ai/sessionDetail` and
`x.ai/schedulerBackgroundLoops`. The response channel is not absent.

That does not freeze option membership. Exact 1.0.4 and 1.0.5 contain:

- no `session_config.rs`
- no `"effort"` option-id string
- no binary proof that `options` contains effort rows or a `selected` /
  `currentValue` equal to a requested `_meta.reasoningEffort`

ACP protocol schema types `SessionConfigSelectOption` (4 fields),
`SessionConfigSelectGroup` (4 fields), `SessionConfigOptionValue`,
`SessionConfigOptionCategory`, `currentValue`, and `ConfigOptionUpdate` appear
in both binaries. Those are compiled protocol-crate names, the same class of
evidence as `config_option_update` schema vocabulary. They do not prove Grok
fills `x.ai/sessionConfig.options` with effort.

Preserved Research 130/163 handshakes recorded `session/new` success and
discarded the result body, including `_meta`. Swallowtail fixtures return only
`{"sessionId": ...}`. There is no retained no-prompt `session/new` payload that
shows effort/selected truth.

Later public `session_setup.rs` / `agent_ops.rs` / `session_config.rs` name
effort rows with `selected: Some(effort.value) == current_effort`. That is not
exact 1.0.4/1.0.5 evidence and is not used as a mapping.

Exact conclusion: 1.0.5's requested `_meta.reasoningEffort` is **not proven**
to return as a selected effort row. The selected-response confirmation question
is unanswered from exact package/handshake evidence, so it is not confirmation.

## Contract 034, Contract 040, And Open-Time Hint

Required sequence for a claimed negotiated option:

1. create or attach
2. receive one bounded option snapshot
3. identify one adapter-private option mapped to the portable value
4. require that value to be selectable
5. send one correlated `session/set_config_option`
6. require a response or update that confirms the effective value
7. return ready / first prompt only after confirmation

Exact qualified Grok ACP does not expose that **negotiated** seam. Absence of
`session/set_config_option` does not by itself disqualify an exact open-time
request-field mapping under Contract 040.

Contract 040 keeps requested, planned, dispatched, accepted, effective, and
observed separate. Dispatch of `_meta.reasoningEffort` would still need:

- no clamp, ignore, or default substitution
- explicit confirmation of the applied value before an effective claim

1.0.5's `_meta.reasoningEffort` remains an **open-time hint**:

- it is sent on `session/new`, not after a bounded snapshot
- unsupported and unparsable values are ignored
- later-source corroboration substitutes last-used/config/catalog defaults
  when the hint is absent
- exact `x.ai/sessionConfig.options` contents are not frozen, so the hint is
  not proven selected in the `session/new` result

Adjacent non-qualifying surfaces:

| Surface | Finding | Why it is not deliver-now |
| --- | --- | --- |
| CLI `--effort` / `--reasoning-effort` | documented for TUI/headless; changelog 0.2.89 makes the names aliases | not on `grok agent stdio`; copying child argv is forbidden |
| `/effort` slash command | TUI; changelog 0.2.82 | not ACP |
| `GROK_CONFIG` `models.default_reasoning_effort` | 1.0.5 binary and current config guide | generic overlay and default substitution |
| initialize `reasoning_efforts` | advertised values | advertisement is not selectability |
| 1.0.4 `session/set_model` effort override | apply or ignore from meta | model switching is out of scope; not `session/new` |
| 1.0.5 `session/new` `_meta.reasoningEffort` | request-dispatchable hint; ignore-on-unsupported | fail-open; exact `sessionConfig` effort/selected payload unfrozen |
| 1.0.4/1.0.5 `x.ai/sessionConfig.options` | vendor response channel exists | option membership and selected-effort truth not frozen |

## Version / Model / Value Disposition

| Version | Model | Value | Advertised | Open-time hint | Negotiated ACP option | Confirmed effective | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0.2.114..=0.2.117` | `grok-4.5` | `low` | yes | no | no | no | no |
| `0.2.114..=0.2.117` | `grok-4.5` | `medium` | yes | no | no | no | no |
| `0.2.114..=0.2.117` | `grok-4.5` | `high` | yes; catalog default | no | no | no | no |
| `0.2.114..=0.2.117` | `grok-4.5` | `xhigh` | no | no | no | no | no |
| `1.0.4` | `grok-4.6` | `low` | yes | no new-session apply path | no | no | no |
| `1.0.4` | `grok-4.6` | `medium` | yes | no new-session apply path | no | no | no |
| `1.0.4` | `grok-4.6` | `high` | yes; catalog default | no new-session apply path | no | no | no |
| `1.0.4` | `grok-4.6` | `xhigh` | yes | no new-session apply path | no | no | no |
| `1.0.5` | `grok-4.6` | `low` | yes | dispatchable; ignore if unsupported | no | no | no |
| `1.0.5` | `grok-4.6` | `medium` | yes | dispatchable; ignore if unsupported | no | no | no |
| `1.0.5` | `grok-4.6` | `high` | yes; catalog default | dispatchable; ignore if unsupported | no | no | no |
| `1.0.5` | `grok-4.6` | `xhigh` | yes | dispatchable; ignore if unsupported | no | no | no |
| any qualified | any | `off`/`none`, `minimal`, `max`, aliases | CLI / later enum names | ignore on parse failure | no | no | no |

No row is deliver-now. The empty set is because exact 1.0.4/1.0.5 evidence does
not freeze `x.ai/sessionConfig.options` effort/selected truth, 1.0.5 fail-opens
on unsupported/unparsable values, and preserved handshakes discarded the
`session/new` result body. It is not because the response channel is absent,
not because 034 `set_config_option` is required for every mapping, and not
because the 1.0.5 hint is later-source only.

## Lifecycle Disposition

| Lifecycle | Disposition |
| --- | --- |
| Interactive `session/new` on 1.0.4 | current wire: `cwd` + empty `mcpServers`; parser exists; no new-session apply path |
| Interactive `session/new` on 1.0.5 | same current Swallowtail wire; provider may apply `_meta.reasoningEffort` and may attach `x.ai/sessionConfig.options`; effort/selected membership is unfrozen |
| Operation-private structured-run session | same `start_session` path, then one prompt; no reasoning input |
| Omission | retain current wire; do not infer `high` or any provider default as a selected value |
| Attachment recovery | empty options already required; 1.0.5 new-session hint does not authorize mutation |
| Load / resume | advertised on 1.0.4/1.0.5; still unqualified; not implied by new-session evidence |
| `UnverifiedNewer` | no latest qualified private mapping to inherit; keep current omission path; fail closed on later snapshot/request/confirmation drift if a future mapping appears |

## Application, Failure, And Revision Posture

Advertised, selectable, requested, accepted, effective, and observed remain
distinct. 1.0.5 can request an open-time hint. That is not selectable
negotiated confirmation.

There is no adapter-private option id, category, or `session/set_config_option`
request shape to freeze. A `session/new` result that may carry
`x.ai/sessionConfig.options` is not enough without exact effort/selected
membership.

Post-allocation Contract 034 selection failure does not arise: Swallowtail
never allocates a session in order to set effort. Existing initialize /
authenticate / `session/new` failure still aborts before a ready handle, joins
owned work, and preserves provider-owned durable-session truth.

No behavior, driver, claim, or configured-instance revision is proposed. The
current public API stays empty `SessionOptions` and no run reasoning member.

## Promotion

Research 204 promotes an empty deliver-now set.

Cards 159-160 stay blocked. A later lane may reopen this family only when exact
1.0.4/1.0.5 (or a later qualified point) evidence freezes `session/new`
`x.ai/sessionConfig.options` effort/selected membership, or a Contract 034
snapshot/selection/confirmation sequence, without ignore-on-unsupported,
default substitution, model switch, load/resume mutation, or a generic
settings map.

CLI `--effort`, hosted xAI Responses `reasoning.effort`, UltraCode-style
aliases, and `GROK_CONFIG` overlays remain out of scope.
