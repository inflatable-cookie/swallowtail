# 246 Codex App-Server Plan-Mode Effort Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.088 / 249

## Question

Which exact `codex.app-server` version, model, Plan mode, effort value, and
lifecycle rows, if any, can bind `plan_mode_reasoning_effort` with closed
selection, request, confirmation, persistence, restoration, and omission truth?

## Decision

No. Research 246 admits an empty deliver-now set. No app-server
`plan_mode_reasoning_effort` binding is authorized on
`swallowtail.codex.app-server`.

Exact tagged `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` expose
`plan_mode_reasoning_effort` only as an ambient `config.toml` /
`config/batchWrite` key. The generated v2 protocol bundle never names that key.
Typed thread, turn, settings, and `config/read` surfaces carry ordinary
`reasoning_effort` / `effort` and experimental `collaborationMode`, not a
Plan-mode-specific effort field. App-server itself never reads
`config.plan_mode_reasoning_effort` into a Plan collaboration mask; only the
TUI applies that key client-side before sending ordinary
`collaboration_mode.settings.reasoning_effort`. Cold resume restores persisted
`model_reasoning_effort`, not the Plan-mode config key.

Ordinary turn reasoning, Codex exec `--config`, Fast, personality, and
verbosity remain contrast only. Current Swallowtail Plan bytes that embed
`SessionOptions.reasoning_mode` into `collaborationMode.settings.reasoning_effort`
are ordinary ReasoningSelection under Plan selection; they must not be promoted
as `plan_mode_reasoning_effort`.

Cards that would bind app-server Plan-mode effort stay blocked. No Plan-mode
effort feature ships from this lane.

## Method And Boundary

Evidence was collected on 2026-08-28 from exact `openai/codex` tags
`rust-v0.147.0` through `rust-v0.149.1`, Research 201 identity, Research 229 /
238 / 242 typed-field leads, official config-reference documentation, and
current Swallowtail adapter production bytes. No Codex install, login,
credential capture, account/catalogue inspection, provider request, prompt, or
paid operation ran. GitHub tag tarballs were downloaded to disposable `/tmp`
only.

Route: `codex.app-server`, driver `swallowtail.codex.app-server`, axis
`codex.cli`, maintained behavior `codex.app-server.v2.workspace-roots` from
`0.131.0`. Codex exec Plan-mode config argv stays out.

Deterministic range corpus:

`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-plan-mode-effort-range.json`

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `rust-v0.147.0` tarball | exact source tree | 2026-08-28 | SHA-256 `355bde4b40d5ba6deea2e469d36f91708315729f3e84c9c69cce6b041a5ba593` |
| GitHub tag `rust-v0.148.0` tarball | exact source tree | 2026-08-28 | SHA-256 `a45e90403eb36b7d6093b167fe1c7dba9b36063bef6d39359eed52c47a21f94a` |
| GitHub tag `rust-v0.149.0` tarball | exact source tree | 2026-08-28 | SHA-256 `3c20d9bb8eb707939472ff9058868f2ac09e885674ebd72261a9db4c8705c2e4` |
| GitHub tag `rust-v0.149.1` tarball | exact source tree | 2026-08-28 | SHA-256 `85139f405ce455bf14ff452615cdb2572d752e31a1e0da6891ac8325915d10ce` |
| Research 201 | npm/Git identity, peeled commit `ff29a44391deccde0aba0f8390337d7f3c319ea4` | 2026-08-24 | see Research 201 |
| Research 229 / 238 / 242 | app-server typed-field / confirmation pattern | 2026-08-27 | see those files |
| [Config reference](https://learn.chatgpt.com/docs/config-file/config-reference.md) | official `plan_mode_reasoning_effort` ambient docs | 2026-08-28 | SHA-256 `b60fbf91a216b9a1136fb42c7dbbbf42d3943b212101aae09a1d5b946805e6dd` |
| `codex-rs/core/config.schema.json` @ `0.149.1` | ambient schema key | 2026-08-28 | SHA-256 `affe54cce9b9945ffd32d322415ff4cc844c62068c1190be6355580be4ca9350` |
| `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json` @ `0.149.1` | generated v2 bundle; key absent | 2026-08-28 | SHA-256 `9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9` |
| `codex-rs/app-server-protocol/src/protocol/v2/thread.rs` @ `0.149.1` | start/resume/settings params | 2026-08-28 | SHA-256 `27b068150d650ec6da10cd811cd176a72dd7844d701ff351989f4423032a0e07` |
| `codex-rs/app-server-protocol/src/protocol/v2/turn.rs` @ `0.149.1` | `TurnStartParams` | 2026-08-28 | SHA-256 `3d76c3154f5d092eb2460fa77f1e8befdf4255447afb741724658afbf30a7704` |
| `codex-rs/app-server-protocol/src/protocol/v2/config.rs` @ `0.149.1` | typed `config/read` lacks Plan-mode key | 2026-08-28 | SHA-256 `e67c812b34a9fddce9ec157870e338dcedebf9ff3003527e7e5b24bd51818d7e` |
| `codex-rs/app-server-protocol/src/protocol/v2/collaboration_mode.rs` @ `0.149.1` | Plan preset mask list types | 2026-08-28 | SHA-256 `1edf8056787546dacb4764fe6c5d6fbffe5668f04d8222a2fb7e91b2ca5be7ef` |
| `codex-rs/models-manager/src/collaboration_mode_presets.rs` @ `0.149.1` | Plan preset Medium | 2026-08-28 | SHA-256 `cc438ddcc2f2e7a8311a23c977ad1d9aeee5ab79d7d2382fb7c488202db98fa8` |
| `codex-rs/protocol/src/config_types.rs` @ `0.149.1` | `CollaborationMode` / `Settings` | 2026-08-28 | SHA-256 `80c1f9a5026019fe813c064ee0ec05a33772f5e6cdd2863d19dda7e4414221ba` |
| `codex-rs/protocol/src/openai_models.rs` @ `0.149.1` | `ReasoningEffort` wire values | 2026-08-28 | SHA-256 `39939bf67ac473b5921d4edd0864df2c1d491edd7de5577957fc877eb3e012c3` |
| `codex-rs/state/src/model/thread_metadata.rs` @ `0.149.1` | persisted resume metadata | 2026-08-28 | SHA-256 `dda4cff3a12a7631502ecc59da8cc46a3bf15f3bbbd937dc8a1f22812cec61f1` |
| `codex-rs/app-server/src/request_processors/config_processor.rs` @ `0.149.1` | session-static batchWrite key | 2026-08-28 | SHA-256 `c09f147c0a685f8daf0f7c527ee7d721b58f414808a7be015e0f60066ec84bd0` |
| `codex-rs/app-server/src/request_processors/thread_processor.rs` @ `0.149.1` | resume restores `model_reasoning_effort` | 2026-08-28 | SHA-256 `49269190a4962d74e51678774c07b2e06bb19e54806f7fecf0cc1ef2250fa8d4` |
| `codex-rs/app-server/src/request_processors/turn_processor.rs` @ `0.149.1` | collaboration mode normalize | 2026-08-28 | SHA-256 `5d6fc08a66946ca4c01f0e624d1886a8575215e0422f71ad127eefa6f032c5d6` |
| `codex-rs/app-server/README.md` @ `0.149.1` | Plan preset Medium; session-static note | 2026-08-28 | SHA-256 `664c71bc798035d7c6a91e6f45a1b6f4a5cd33745c8df0b1376717dbeed62ed5` |
| `app-server-releases.json` / sibling fixtures | exact tag commits | 2026-08-28 | workspace fixtures |

`turn.rs` and `collaboration_mode_presets.rs` are byte-identical across
`0.147.0..=0.149.1`. `thread.rs`, typed `config.rs`, and the v2 schema bundle
are byte-identical at `0.149.0` and `0.149.1`. The v2 bundle never contains
`plan_mode_reasoning_effort` on any exact tag in the set.

## Syntax And Shared Config

Official config reference documents:

```text
plan_mode_reasoning_effort = none | minimal | low | medium | high | xhigh
```

Exact `0.149.1` `config.schema.json` types the key as open `ReasoningEffort`
(non-empty string). The Rust enum also knows `max`, `ultra`, and
`Custom(String)`. That ambient parser is not a typed app-server Plan-mode RPC
field.

Official README states Plan-mode reasoning-effort defaults written through
`config/batchWrite` are session-static and do not reload existing threads.

## App-Server Configuration Surface

| Surface | `plan_mode_reasoning_effort` present? | Swallowtail relevance |
| --- | --- | --- |
| `config.toml` / profile | yes, ambient | not a prepared-session dispatch seam |
| `config/batchWrite` key path | yes; session-static | disk write; does not reload live threads |
| `config/read` typed `Config` | no | may only appear in flatten/`additional` |
| `thread/start` typed params | no | has ordinary `reasoningEffort` and `collaborationMode` |
| `thread/resume` typed params | no | same |
| `turn/start` typed params | no | has ordinary `effort` and `collaborationMode` |
| `thread/settings/update` typed params | no | has ordinary `effort` and `collaborationMode` |
| `ThreadSettings` / settings-updated | no | returns ordinary `effort` + live `collaborationMode` |
| `collaborationMode/list` Plan preset | no Plan-mode key | preset `reasoning_effort = medium` |
| `model/list` | no Plan-mode key | ordinary supported reasoning efforts |
| persisted `ThreadMetadata` | no | resume restores `reasoning_effort` as `model_reasoning_effort` |

JSON wire for Plan selection remains experimental `collaborationMode` with
`settings.reasoning_effort`. That is a different control from the ambient
`plan_mode_reasoning_effort` key.

## Plan Selection Versus Plan-Mode Effort

Plan must be selected before any Plan-specific effort claim. On app-server,
Plan selection is `collaborationMode.mode = plan`. Built-in Plan preset effort
is Medium. Callers may override via `collaborationMode.settings.reasoning_effort`
or ordinary `effort` / `reasoningEffort` fields.

Exact tagged app-server and core sources never consult
`config.plan_mode_reasoning_effort` when normalizing or applying a Plan
collaboration mode. TUI `set_plan_mode_reasoning_effort` mutates the active Plan
mask client-side, then sends ordinary collaboration-mode settings. That proves
upstream interactive UX can honor the ambient key; it does not create a
confirmable prepared-session RPC for Swallowtail.

## Request Construction And Truth Separation

| Truth | Finding |
| --- | --- |
| configured ambient | `plan_mode_reasoning_effort` may exist in user/profile config |
| Plan-selected | requires `collaborationMode.mode = plan` (or TUI equivalent) |
| dispatched Plan effort | `collaborationMode.settings.reasoning_effort` and/or ordinary `effort` |
| dispatched `plan_mode_reasoning_effort` key | absent from typed v2 RPCs; Swallowtail does not serialize it |
| accepted | ambient config load / batchWrite; no typed Plan-mode field to accept |
| effective | TUI may apply ambient key client-side; app-server does not auto-apply it |
| returned | ordinary effort / collaboration mode preference only |
| persisted | not in `ThreadMetadata` |
| restored | cold resume restores ordinary `model_reasoning_effort`, not Plan-mode key |
| omitted Swallowtail bytes | current Plan path still sends ordinary reasoning under Plan settings |

Unsupported or drifted ambient values are not a typed pre-effect rejection seam
for prepared sessions. Membership for ordinary effort remains model-catalog /
live-catalogue territory and is out of scope for this Plan-mode key.

## Lifecycle Seam Audit

| Operation | Current Swallowtail bytes | Plan-mode effort seam |
| --- | --- | --- |
| new / open | `thread/start` without Plan-mode key or `collaborationMode` | none for this control |
| turn / follow-up | `turn/start` may send `effort` and Plan `collaborationMode` with ordinary reasoning | ordinary ReasoningSelection under Plan; not `plan_mode_reasoning_effort` |
| settings update | not used by current adapter | no typed Plan-mode key |
| load / resume | `thread/resume` without Plan-mode key | metadata restores ordinary effort only |
| fresh replacement | new open/turn family | no inherited Plan-mode key claim |

## Cross-Version Finding

Decisive negative shape is stable across `0.147.0`, `0.148.0`, `0.149.0`, and
`0.149.1`:

- ambient `plan_mode_reasoning_effort` exists
- v2 protocol never exposes a typed Plan-mode effort field
- app-server does not auto-apply the ambient key to Plan
- confirmation and metadata speak ordinary effort / collaboration mode
- Plan preset Medium is stable and byte-identical

## Claim Strength

| Claim | Strength |
| --- | --- |
| ambient key documented and present in schema | proved at exact tags + official config reference |
| v2 schemas omit `plan_mode_reasoning_effort` | proved; count zero across exact set |
| typed Config omits the key | proved from `config.rs` |
| app-server does not read the key into Plan | proved; only config storage + batchWrite session-static list |
| TUI applies the key client-side | proved from TUI settings; not an app-server RPC |
| Plan preset Medium | proved from `collaboration_mode_presets.rs` and README |
| resume restores ordinary effort only | proved from `thread_processor` + `ThreadMetadata` |
| Swallowtail Plan uses ordinary reasoning under collaboration settings | proved from adapter `session_input.rs` / `interactive.rs` |
| provider-visible Plan-mode effort effect of ambient key alone | unproved; not authorized |

## Deliver-Now Table

No row is deliver-now.

| Row | Disposition |
| --- | --- |
| any version / model / Plan / value on `codex.app-server` for `plan_mode_reasoning_effort` | not deliver-now; no typed confirmable seam |
| ambient `config.toml` / `config/batchWrite` | session-static ambient only; not prepared binding |
| `collaborationMode.settings.reasoning_effort` | distinct control; ordinary Plan override, not this key |
| ordinary `effort` / `reasoningEffort` | ordinary turn/session reasoning; must not be promoted |
| omitted Swallowtail Plan bytes that reuse ReasoningSelection | current behavior; not `plan_mode_reasoning_effort` |
| TUI client-side ambient apply | interactive UX only; not Swallowtail prepared dispatch |
| Codex exec `--config plan_mode_reasoning_effort` | not applicable to this route |
| provider-accepted / effective / observed Plan-mode effort from ambient key | withheld |

Deliver-now rows: **0**.

## Adapter Binding Requirements

No app-server `plan_mode_reasoning_effort` binding is authorized. If a future
Codex release adds a typed, Plan-gated, confirmable, persistence-honest seam
for this exact control, re-qualify from primary sources. Until then:

- do not invent a Plan-mode effort API from ambient config or TUI behavior
- do not promote ordinary turn reasoning or `collaborationMode.settings.reasoning_effort`
  as this control
- keep Plan selection, ordinary ReasoningSelection, Fast, personality, and
  verbosity distinct
- omission of the ambient key must not claim Plan preset Medium or caller
  intent as Swallowtail-selected `plan_mode_reasoning_effort`
