# 229 Codex App-Server Model Verbosity Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.082 / 228

## Question

Which exact `codex.app-server` version, model, value, operation, and lifecycle
rows can bind caller-selected `model_verbosity` without borrowing exec argv or
allowing ambient defaults, unsupported-model ignore, or unconfirmed
substitution?

## Decision

No. Research 229 admits an empty deliver-now set. No app-server
`model_verbosity` binding is authorized on `swallowtail.codex.app-server`.

Exact tagged `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` expose
`model_verbosity` only through ambient config layering and an untyped
`thread/start` / `thread/resume` / `thread/fork` `config` map. The stable RPC
surface that Swallowtail uses has no typed verbosity field on session open,
turn start, thread settings update, or `model/list`. Responses and
`ThreadSettings` confirm `effort` but never verbosity. Unsupported models still
warn and omit Responses `text.verbosity` after the session is live. That
combination fails the card’s confirmation, catalog admission, and pre-effect
rejection gates. Exec Research 213 remains exec-only and must not be copied.

Cards that would bind app-server verbosity stay blocked. No app-server verbosity
feature ships from this lane.

## Method And Boundary

Evidence was collected on 2026-08-27 from exact `openai/codex` tags
`rust-v0.147.0` through `rust-v0.149.1`, Research 201 identity, Research 037
lifecycle corpus, Research 213 exec lead, and current Swallowtail adapter
production bytes. No Codex install, login, credential capture, account/catalogue
inspection, provider request, prompt, or paid operation ran. GitHub tag tarballs
were downloaded to disposable `/tmp` only.

Route: `codex.app-server`, driver `swallowtail.codex.app-server`, axis
`codex.cli`, maintained behavior `codex.app-server.v2.workspace-roots` from
`0.131.0`. Exec argv and `CodexExecProfileInput::with_model_verbosity()` are out
of scope.

Deterministic range corpus:

`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-model-verbosity-range.json`

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `rust-v0.149.1` tarball | exact source tree | 2026-08-27 | SHA-256 `85139f405ce455bf14ff452615cdb2572d752e31a1e0da6891ac8325915d10ce` |
| Research 201 | npm/Git identity, peeled commit `ff29a44391deccde0aba0f8390337d7f3c319ea4` | 2026-08-24 | see Research 201 |
| Research 213 | exec-only lead; shared schema/models/client semantics | 2026-08-25 | see Research 213 |
| Research 037 | app-server lifecycle and schema authority | 2026-07-27 | see Research 037 |
| `codex-rs/core/config.schema.json` @ `0.149.1` | closed `model_verbosity` enum | 2026-08-27 | SHA-256 `affe54cce9b9945ffd32d322415ff4cc844c62068c1190be6355580be4ca9350` |
| `codex-rs/models-manager/models.json` @ `0.149.1` | exec/catalog metadata; not app-server `model/list` | 2026-08-27 | SHA-256 `c18214b1ba88ab9bd164753115324a7a29c0582e8d071f7b3babf749d892f549` |
| `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json` @ `0.149.1` | generated v2 bundle | 2026-08-27 | SHA-256 `9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9` |
| `codex-rs/app-server-protocol/src/protocol/v2/thread.rs` @ `0.149.1` | `ThreadStartParams`, `ThreadSettings` | 2026-08-27 | SHA-256 `27b068150d650ec6da10cd811cd176a72dd7844d701ff351989f4423032a0e07` |
| `codex-rs/app-server-protocol/src/protocol/v2/turn.rs` @ `0.149.1` | `TurnStartParams` | 2026-08-27 | SHA-256 `3d76c3154f5d092eb2460fa77f1e8befdf4255447afb741724658afbf30a7704` |
| `codex-rs/app-server-protocol/src/protocol/v2/model.rs` @ `0.149.1` | `ModelListResponse` rows | 2026-08-27 | SHA-256 `3991f4a8f595767ac0de5d39a4edd7c2eea68ecaedfb007a4554e5fb1497d4e8` |
| `codex-rs/app-server-protocol/src/protocol/v2/config.rs` @ `0.149.1` | `config/read` `Config.model_verbosity` | 2026-08-27 | present in tag tree |
| `codex-rs/protocol/src/config_types.rs` @ `0.149.1` | `Verbosity` enum | 2026-08-27 | SHA-256 `80c1f9a5026019fe813c064ee0ec05a33772f5e6cdd2863d19dda7e4414221ba` |
| `codex-rs/core/src/client.rs` @ `0.149.1` | Responses `text.verbosity`; unsupported ignore | 2026-08-27 | SHA-256 `7af5d4c0c15673564b455725c98d34379c4bdf579f52c4d89aecb4ceb4b190fe` |
| `codex-rs/tui/src/app_server_session.rs` @ `0.149.1` | upstream `config.model_verbosity` map usage | 2026-08-27 | SHA-256 `a2b32e794ab9c02e88235b7d5d61be93e991cce88ff5c5363ff150561ce8f24f` |
| `codex-rs/state/src/model/thread_metadata.rs` @ `0.149.1` | persisted resume metadata | 2026-08-27 | no `verbosity` field |
| `app-server-releases.json` | exact tag commits and v2 bundle hashes | 2026-08-27 | workspace fixture |

`turn.rs` is byte-identical across `0.147.0..=0.149.1`. `model.rs` is
byte-identical at `0.148.0`, `0.149.0`, and `0.149.1`. Earlier tag diffs do
not add typed verbosity fields. The v2 schema bundle is byte-identical at
`0.149.0` and `0.149.1`.

## Syntax And Shared Config

Exact `0.149.1` schema defines `model_verbosity` as optional
`low|medium|high`, same closed enum as Research 213. The Rust enum is:

```rust
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    #[default]
    Medium,
    High,
}
```

That parser exists for config layering and exec. It is not a typed app-server
session or turn RPC field.

## App-Server Configuration Surface

| Surface | `model_verbosity` present? | Swallowtail relevance |
| --- | --- | --- |
| `config/read` effective config | yes, typed | ambient/effective read only; not a prepared-session dispatch seam |
| `thread/start.params.config` map | yes, string key `model_verbosity` | untyped config override; not used by Swallowtail |
| `thread/resume.params.config` map | yes | same |
| `thread/fork.params.config` map | yes | same |
| `turn/start` typed params | no | reasoning has `effort`; verbosity does not |
| `thread/settings/update` typed params | no | reasoning has `effort`; verbosity does not |
| `model/list` `Model` rows | no | no `support_verbosity` / `default_verbosity` admission metadata |
| `ThreadStartResponse` / `ThreadSettings` | no | no confirmation field |
| persisted `ThreadMetadata` | no | resume restores `reasoning_effort`, not verbosity |

Upstream TUI code maps selected config into the generic map:

```text
config: { "model_verbosity": "low", ... }
```

for `thread/start`, `thread/resume`, and `thread/fork`. That proves Codex can
consume the key when embedded in config resolution. It does not create a
first-class, confirmable prepared-session control comparable to app-server
reasoning selection or exec `--config model_verbosity`.

Config map overrides merge as CLI/session overrides inside
`ConfigManager::load_with_overrides`. They can beat user/project layers, but
they still ride the ambient config stack rather than a dedicated RPC seam.

## Request Construction And Unsupported Behavior

At session creation, `ModelClient::new(..., config.model_verbosity, ...)` binds
verbosity for the live session. There is no later typed per-turn override.

For Responses requests, exact `client.rs` still does:

```rust
let verbosity = if model_info.support_verbosity {
    self.state.model_verbosity.or(model_info.default_verbosity)
} else {
    if self.state.model_verbosity.is_some() {
        warn!(
            "model_verbosity is set but ignored as the model does not support verbosity: {}",
            model_info.slug
        );
    }
    None
};
```

That is the same warn-and-ignore path as exec Research 213, but app-server has
no frozen `model/list` metadata or pre-thread admission surface to reject
unsupported slugs before session work. The warning happens during provider
request construction inside an already-started session.

Omitted Swallowtail app-server bytes do not serialize `config` or
`model_verbosity` on `thread/start` or `turn/start`. Omission must not claim the
ambient or model default as caller-selected. Codex may still send model defaults
after session bind; that is provider-side request construction, not Swallowtail
RPC proof.

## Lifecycle Seam Audit

Current Swallowtail prepared app-server paths:

| Operation | Current bytes | Verbosity seam |
| --- | --- | --- |
| new / open | `thread/start { model, developerInstructions?, dynamicTools?, workspace fields }` | none |
| turn / follow-up | `turn/start { threadId, input, effort?, collaborationMode?, sandboxPolicy? }` | none |
| load / resume | `thread/resume { threadId, model, developerInstructions?, workspace fields }` | none |
| fresh restoration | same resume/open family | none |
| import | not a verbosity surface in current adapter | none |

Reasoning selection is bound through plan requirements plus turn `effort`.
Verbosity has no parallel typed binding, evidence field, plan constraint, or
response confirmation.

`HarnessConfigurationPosture::Ambient` is required for app-server preflight.
That posture matches config-layer resolution, not an isolated caller-only map
with explicit effective-state proof.

## Cross-Version Finding

The decisive negative shape is stable across the exact retrieved release set
`0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1`:

- no typed verbosity on `turn/start` or thread settings APIs
- no verbosity in `model/list`
- no verbosity in `ThreadSettings`
- no persisted verbosity in `ThreadMetadata`
- only generic config-map and config-read exposure

Research 213’s exec deliver-now table, models.json rows, and
`--config model_verbosity` argv must not be promoted onto this route.

## Claim Strength

| Claim | Strength |
| --- | --- |
| config parser accepts `low\|medium\|high` | proved at exact tag schema |
| app-server lacks typed session/turn verbosity params | proved from v2 protocol sources and generated schemas |
| `model/list` lacks verbosity metadata | proved from `model.rs` and generated bundle |
| `ThreadSettings` confirms effort but not verbosity | proved from `thread.rs` and `thread_summary.rs` |
| generic `config.model_verbosity` can be supplied on thread start/resume/fork | proved from TUI map construction |
| Swallowtail omission lacks verbosity bytes | proved from current adapter sources |
| unsupported model warns and omits `text.verbosity` | proved from exact `client.rs`; post-session only |
| provider acceptance or effective response length | unproved; not authorized |
| live catalog replacement after session bind | possible; not admission authority |

## Deliver-Now Table

No row is deliver-now.

| Row | Disposition |
| --- | --- |
| any version / model / value on `codex.app-server` | not deliver-now; no typed confirmable seam |
| generic `thread/*/config.model_verbosity` | ambient config override only; not a Swallowtail prepared binding |
| `config/read.model_verbosity` | read/effective only; not caller dispatch |
| omitted Swallowtail app-server bytes | current behavior; not a default serialization |
| `codex.exec` Research 213 table | not applicable to this route |
| exec `--config model_verbosity` | not applicable to this route |
| unknown values via config map | fail at config load when serde rejects enum; still no confirmation |
| unsupported slug with explicit verbosity | warn-and-ignore at provider request time; not pre-thread rejection |
| provider-accepted / effective / observed verbosity | withheld |

Deliver-now rows: **0**.

## Adapter Binding Requirements

No app-server verbosity binding is authorized. If a future Codex release adds
a typed, confirmable, catalog-backed seam, re-qualify from primary sources.
Until then:

- do not add `CodexModelVerbosity` or config-map serialization to app-server
  prepared profiles
- do not reuse exec admission tables or `--config` argv evidence
- keep current omission byte-equivalent
- do not claim ambient defaults as caller-selected verbosity

## Decision

Card 228 is complete with an honest empty set. App-server model-verbosity
production binding remains blocked.
