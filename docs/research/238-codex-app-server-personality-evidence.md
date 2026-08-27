# 238 Codex App-Server Personality Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.085 / 239

## Question

Which exact `codex.app-server` version, model, value, operation, and lifecycle
rows can bind caller-selected personality with pre-effect rejection and honest
dispatch, confirmation, persistence, and restoration truth?

## Decision

No. Research 238 admits an empty deliver-now set. No app-server personality
binding is authorized on `swallowtail.codex.app-server`.

Exact tagged `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` expose a closed
typed `Personality` enum (`none|friendly|pragmatic`) on `thread/start`,
`thread/resume`, `turn/start`, and `thread/settings/update`, and return the
preference on `ThreadSettings` / `thread/settings/updated`. That typed surface
is real. It still fails the card’s gates:

- unsupported model membership does not reject before session or turn work
- `ThreadStartResponse` / `ThreadResumeResponse` / `TurnStartResponse` do not
  echo personality, so open/turn confirmation is preference-only via settings
- preference echo is not selected-operation / prompt-effect truth
- current default bundled family `gpt-5.6-*` has
  `supports_personality() == false` while still accepting the field
- `ThreadMetadata` does not persist personality for cold resume restoration
- live catalogue can rewrite `model_messages`, so static membership is not
  runtime admission authority without forbidden live inspection

Research 229 remains the typed-field lead only. Do not invent a
provider-neutral persona control from Codex vocabulary.

Cards that would bind app-server personality stay blocked. No personality
feature ships from this lane.

## Method And Boundary

Evidence was collected on 2026-08-27 from exact `openai/codex` tags
`rust-v0.147.0` through `rust-v0.149.1`, Research 201 identity, Research 229
typed-field lead, official app-server README in the tag tree, and current
Swallowtail adapter production bytes. No Codex install, login, credential
capture, account/catalogue inspection, provider request, prompt, or paid
operation ran. GitHub tag tarballs were reused from disposable `/tmp` evidence
trees already retrieved for Research 229; digests match that freeze.

Route: `codex.app-server`, driver `swallowtail.codex.app-server`, axis
`codex.cli`, maintained behavior `codex.app-server.v2.workspace-roots` from
`0.131.0`. Codex exec, Fast/service tier, verbosity, Plan effort, and
multi-agent are out of scope.

Deterministic range corpus:

`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-personality-range.json`

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `rust-v0.149.1` tarball | exact source tree | 2026-08-27 | SHA-256 `85139f405ce455bf14ff452615cdb2572d752e31a1e0da6891ac8325915d10ce` |
| Research 201 | npm/Git identity, peeled commit `ff29a44391deccde0aba0f8390337d7f3c319ea4` | 2026-08-24 | see Research 201 |
| Research 229 | typed personality fields as lead; verbosity empty set | 2026-08-27 | see Research 229 |
| `codex-rs/app-server/README.md` @ `0.149.1` | official personality values and RPC examples | 2026-08-27 | SHA-256 `664c71bc798035d7c6a91e6f45a1b6f4a5cd33745c8df0b1376717dbeed62ed5` |
| `codex-rs/core/config.schema.json` @ `0.149.1` | closed `Personality` enum; feature `personality` boolean | 2026-08-27 | SHA-256 `affe54cce9b9945ffd32d322415ff4cc844c62068c1190be6355580be4ca9350` |
| `codex-rs/models-manager/models.json` @ `0.149.1` | bundled model_messages / static membership | 2026-08-27 | SHA-256 `c18214b1ba88ab9bd164753115324a7a29c0582e8d071f7b3babf749d892f549` |
| `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json` @ `0.149.1` | generated v2 bundle | 2026-08-27 | SHA-256 `9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9` |
| `codex-rs/app-server-protocol/src/protocol/v2/thread.rs` @ `0.149.1` | start/resume/settings params and `ThreadSettings` | 2026-08-27 | SHA-256 `27b068150d650ec6da10cd811cd176a72dd7844d701ff351989f4423032a0e07` |
| `codex-rs/app-server-protocol/src/protocol/v2/turn.rs` @ `0.149.1` | `TurnStartParams.personality` | 2026-08-27 | SHA-256 `3d76c3154f5d092eb2460fa77f1e8befdf4255447afb741724658afbf30a7704` |
| `codex-rs/app-server-protocol/src/protocol/v2/model.rs` @ `0.149.1` | `Model.supports_personality` | 2026-08-27 | SHA-256 `3991f4a8f595767ac0de5d39a4edd7c2eea68ecaedfb007a4554e5fb1497d4e8` |
| `codex-rs/app-server-protocol/src/protocol/v2/config.rs` @ `0.149.1` | typed `config/read` `Config` lacks `personality` | 2026-08-27 | present in tag tree |
| `codex-rs/protocol/src/config_types.rs` @ `0.149.1` | `Personality` enum | 2026-08-27 | SHA-256 `80c1f9a5026019fe813c064ee0ec05a33772f5e6cdd2863d19dda7e4414221ba` |
| `codex-rs/protocol/src/openai_models.rs` @ `0.149.1` | `supports_personality()`, instruction substitution | 2026-08-27 | SHA-256 `39939bf67ac473b5921d4edd0864df2c1d491edd7de5577957fc877eb3e012c3` |
| `codex-rs/models-manager/src/model_info.rs` @ `0.149.1` | feature-gated personality template rewriting | 2026-08-27 | SHA-256 `ac0f35917888f3041737589a8df88924e9728a4967350531515d0d4f5c0a0744` |
| `codex-rs/app-server/src/request_processors/thread_summary.rs` @ `0.149.1` | `ThreadSettings.personality` from config snapshot | 2026-08-27 | SHA-256 `8d209a9d9a6f99a4551e771cc06e12eb496bbc03af9b962fa9a4845e14a3c9f1` |
| `codex-rs/state/src/model/thread_metadata.rs` @ `0.149.1` | persisted resume metadata | 2026-08-27 | SHA-256 `dda4cff3a12a7631502ecc59da8cc46a3bf15f3bbbd937dc8a1f22812cec61f1` |
| `codex-rs/features/src/lib.rs` @ `0.149.1` | `Feature::Personality` default enabled | 2026-08-27 | SHA-256 `791121524b5269c72254911823b77253cc98121d1dd29608663dd9d73fa7d61a` |
| `app-server-releases.json` / verbosity fixture tag commits | exact tag commits | 2026-08-27 | workspace fixtures |

`turn.rs` is byte-identical across `0.147.0..=0.149.1`. `model.rs` is
byte-identical at `0.148.0`, `0.149.0`, and `0.149.1`. `thread.rs` and the v2
schema bundle are byte-identical at `0.149.0` and `0.149.1`. `models.json` is
byte-identical at `0.149.0` and `0.149.1`. The closed enum and bundled
supports_personality membership shape are stable across the exact set.

## Syntax And Closed Values

Exact `0.149.1` schema and Rust enum:

```rust
#[serde(rename_all = "lowercase")]
pub enum Personality {
    None,
    Friendly,
    Pragmatic,
}
```

Official app-server README states valid values are `"friendly"`, `"pragmatic"`,
and `"none"`. Unknown strings fail serde at RPC decode before thread/turn
handlers run. That is value-level rejection only.

## App-Server Configuration Surface

| Surface | `personality` present? | Swallowtail relevance |
| --- | --- | --- |
| `thread/start.params.personality` | yes, typed | typed dispatch; start response does not echo it |
| `thread/resume.params.personality` | yes, typed | same; cold resume needs re-supply or ambient |
| `turn/start.params.personality` | yes, typed; “this turn and subsequent turns” | turn response is turn-only; no personality echo |
| `thread/settings/update.params.personality` | yes, typed | ack only; wait for `thread/settings/updated` |
| `ThreadSettings.personality` | yes | preference confirmation, not prompt-effect proof |
| `thread/settings/updated` | yes, via `thread_settings` | same preference confirmation |
| `ThreadStartResponse` / `ThreadResumeResponse` | no | contrasts with `reasoningEffort` on start |
| `TurnStartResponse` | no | no turn confirmation field |
| `config/read` typed `Config` | no | ambient `personality` may only appear in flatten/additional |
| `config.schema.json` / user config.toml | yes | ambient default; omission inherits it |
| `model/list` `Model.supportsPersonality` | yes | advertising only; server does not enforce |
| persisted `ThreadMetadata` | no | resume restores model/effort, not personality |

`Feature::Personality` is stable and default-enabled. When disabled, model
instruction templates are rewritten and personality variables are cleared;
RPC still accepts the typed field. Ambient `config.personality` remains a
session default when callers omit the field.

## Model Membership

`ModelInfo::supports_personality()` is derived from bundled or live
`model_messages`: template must contain `{{ personality }}` and
`instructions_variables` must supply `personality_default`,
`personality_friendly`, and `personality_pragmatic`.

Exact bundled `models.json` at `0.149.1` (and `0.149.0`):

| Slug | `supports_personality()` |
| --- | --- |
| `gpt-5.5` | true |
| `gpt-5.4` | true |
| `gpt-5.4-mini` | true |
| `codex-auto-review` | true |
| `gpt-5.6-sol` | false |
| `gpt-5.6-terra` | false |
| `gpt-5.6-luna` | false |
| `gpt-5.2` | false |

`gpt-5.6-*` templates bake a personality section without the placeholder and
ship empty personality variable strings. Selecting `friendly`/`pragmatic` on
those slugs can still be stored and returned on `ThreadSettings` while prompt
substitution is non-operational. No app-server or core path rejects that
mismatch before process, access, resource, or provider work.

Live catalogue replacement can change `model_messages` after the tag freeze.
Static membership is evidence-only, not live admission authority. Live
catalogue/account inspection is out of scope for this lane.

## Request Construction And Truth Separation

| Truth | Finding |
| --- | --- |
| requested | caller may send typed `personality` on start/resume/turn/settings |
| RPC-dispatched | Swallowtail currently omits the field on open and turn bytes |
| accepted | unknown enum rejects at decode; supported and unsupported models both accept known values into session config |
| effective | depends on model_messages, `Feature::Personality`, and baked vs placeholder instructions |
| returned | `ThreadSettings` / settings-updated preference only; start/turn responses omit personality |
| persisted | not in `ThreadMetadata`; may remain in ambient user config if written elsewhere |
| restored | cold resume does not restore prior thread personality from metadata; resume param or ambient config required |
| observed | provider-visible tone unproved; not authorized |

Omitted Swallowtail app-server bytes do not serialize `personality` on
`thread/start`, `thread/resume`, or `turn/start`. Omission must not claim the
ambient config default as caller-selected personality.

## Lifecycle Seam Audit

| Operation | Current Swallowtail bytes | Personality seam |
| --- | --- | --- |
| new / open | `thread/start` without `personality` | typed field available; response does not confirm |
| turn / follow-up | `turn/start` without `personality` | typed override available; response does not confirm |
| settings update | not used by current adapter | preference confirmable via `thread/settings/updated` |
| load / resume | `thread/resume` without `personality` | no metadata restore; ambient or re-supply only |
| interrupted-turn reconciliation | existing turn/settings paths | preference may be live in session; not metadata-backed |
| fresh replacement | new `thread/start` | no inherited personality claim |

Personality is turn-overridable and then subsequent-turn scoped while a session
is live. It is not durable thread metadata across cold resume.

## Cross-Version Finding

Decisive negative shape is stable across `0.147.0`, `0.148.0`, `0.149.0`, and
`0.149.1`:

- closed typed vocabulary and settings preference echo exist
- start/turn responses do not confirm personality
- unsupported bundled models still accept the field
- no server pre-effect rejection for unsupported membership
- no `ThreadMetadata` persistence
- live catalogue can move membership

## Claim Strength

| Claim | Strength |
| --- | --- |
| closed values `none\|friendly\|pragmatic` | proved at schema, enum, and official README |
| typed dispatch on start/resume/turn/settings | proved from v2 protocol sources |
| `ThreadSettings` returns preference | proved from protocol and thread_summary |
| unknown values reject at RPC decode | proved by serde enum; pre-handler |
| unsupported models do not reject | proved by absence of enforcement paths; acceptance into session config |
| bundled supports_personality membership | proved from models.json + `supports_personality()` rules at exact tags |
| live catalogue membership | unproved; not authorized |
| start/turn response confirmation | absent |
| ThreadMetadata persistence/restoration | absent |
| effective / observed tone | unproved; not authorized |
| Swallowtail omission lacks personality bytes | proved from current adapter sources |

## Deliver-Now Table

No row is deliver-now.

| Row | Disposition |
| --- | --- |
| any version / model / value on `codex.app-server` | not deliver-now; preference echo without unsupported-model rejection or start/turn confirmation of selected operation |
| `gpt-5.5` / `gpt-5.4` / `gpt-5.4-mini` / `codex-auto-review` × closed values | bundled membership only; live catalogue open; no server rejection gate for mismatches |
| `gpt-5.6-*` / `gpt-5.2` × closed values | accepted preference possible with non-operational effect; dishonest as selected-operation binding |
| `thread/settings/updated` preference | returned preference only; not effective/observed proof |
| ambient `config.personality` / feature flag | ambient; not caller-selected prepared binding |
| omitted Swallowtail app-server bytes | current behavior; not a default serialization |
| provider-accepted / effective / observed personality | withheld |
| generic provider-neutral persona vocabulary | not applicable |

Deliver-now rows: **0**.

## Adapter Binding Requirements

No app-server personality binding is authorized. If a future Codex release adds
server-side unsupported-model rejection before session/turn work, start or turn
confirmation of the selected value, durable restore truth, and frozen
membership that does not require live catalogue inspection, re-qualify from
primary sources. Until then:

- do not add personality serialization to app-server prepared profiles
- do not promote Codex personality names into a provider-neutral persona API
- keep current omission byte-equivalent
- do not claim ambient defaults or settings preference echoes as
  caller-selected effective personality

## Decision

Card 239 is complete with an honest empty set. App-server personality
production binding remains blocked.
