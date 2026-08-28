# 253 Goose ACP Mode Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.090 / 256

## Question

Which exact `goose.acp` `1.46.0` mode rows, if any, can bind a safe
adapter-local selection with closed membership, application, confirmation,
permission, lifecycle, failure, and omission truth?

## Method And Boundary

Evidence was frozen on 2026-08-28 from exact GitHub tag `v1.46.0` commit
`98c11ce2ee7b9b302978aa64b1eab7d0895607c7` (`block/goose` /
`aaif-goose/goose`), tagged docs under that commit, and the existing
Swallowtail Goose ACP identity fixtures. No Goose install, PATH search,
`goose configure`, login, credential use, provider prompt, paid work,
extension installation, host config mutation, ACP `initialize`, or
`session/new`. Host has no `goose`.

Route remains `goose.acp`, driver `swallowtail.goose.acp`, axis
`goose.release` exact `1.46.0`. Current argv is exactly `goose acp`.
Isolation stays `AmbientHost`. Permission requests stay observe-and-stop;
`allow_always` and GooseMode `auto` stay unselected. Research 250 builtin
evidence stays out of scope and is not reused as a mode disposition.

The adapter, shared guides/matrices, and production surfaces were inspected
and not changed. No Contract 029 window movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `v1.46.0` | exact release commit | 2026-08-28 | commit `98c11ce2ee7b9b302978aa64b1eab7d0895607c7` (2026-08-11T22:32:02Z) |
| `crates/goose-provider-types/src/goose_mode.rs` | closed `GooseMode` membership and default | 2026-08-28 | SHA-256 `d5b0ae31b4882085b572ca7b595f9153f595c417f797d4ef6a50bcba1e7ebdd7` (707 bytes) |
| `crates/goose/src/acp/server/new_session.rs` | `session/new` seeds mode from host config | 2026-08-28 | SHA-256 `10146d0e24a148f5d89bbdb3447f4720cff74b2fdb62d80b8febf2cc3e2a6dad` (13004 bytes) |
| `crates/goose/src/acp/response_builder.rs` | `build_mode_state`; config option `mode` | 2026-08-28 | SHA-256 `b1a18f5ec51f49a2bc5bf45cbc86a1c2fcb2fdbfd1d5524cc4b243b5bbece571` (26953 bytes) |
| `crates/goose/src/acp/server/dispatch.rs` | `session/set_mode` and set-config `mode` | 2026-08-28 | SHA-256 `f7ae4a0e3a52b852df4cbb7f44ac8113f795d55df020de5ea96d36f3d5101f21` (25217 bytes) |
| `crates/goose/src/acp/server.rs` | `on_set_mode`; `build_config_update` | 2026-08-28 | SHA-256 `3eb611c4ae0f37a1d4affde82a4c29a5d56e9d70cb796d8cc7cc6082bc13b47f` (103385 bytes) |
| `crates/goose/src/agents/agent.rs` | `update_goose_mode`; Chat tool skip | 2026-08-28 | SHA-256 `2227f114aae644241feaeb3a4f1a9feed1545cf6e9a792b39783deac89c19be6` (217307 bytes) |
| `crates/goose/src/permission/permission_inspector.rs` | per-mode tool allow / ask / LLM paths | 2026-08-28 | SHA-256 `c9944a12742c443bb834abead6f3a5120f6b09d666a41d80ab1491752b545390` (15971 bytes) |
| `crates/goose/src/session/session_manager.rs` | session `goose_mode` persistence; malformed → default | 2026-08-28 | SHA-256 `d1d402bad5c55e3b50b3d81ad6fd9c29b4f6c69e59ec1198fded81760ebf14c8` (160040 bytes) |
| `crates/goose/src/config/base.rs` | `GOOSE_MODE` config value | 2026-08-28 | SHA-256 `07828d8ca7fdd5f5a80769879ad4bf0f1ca61c788506390db74da1dda1d8378d` (92115 bytes) |
| `documentation/docs/guides/config-files.md` | docs membership and default `"auto"` | 2026-08-28 | SHA-256 `73ab969317594a8b0558d66a9683fd2962d8d74ccb054363fb4294235e934be9` (12504 bytes) |
| `documentation/docs/guides/managing-tools/goose-permissions.md` | product mode semantics; Autonomous default | 2026-08-28 | SHA-256 `0674af239d66af1f2c448ef691ee3a1c68915d84635a5d817ed1e5d504b18e1d` (6403 bytes) |
| `documentation/docs/guides/acp-clients.md` | ACP clients may switch modes mid-session | 2026-08-28 | SHA-256 `9db93def1b6920317463cf20949b0979c96520ce8b57cf557b44a727c41143bf` (10014 bytes) |
| Swallowtail fixtures `goose-acp-1.46.0/` | argv, advertisement shape, auto unselected | 2026-08-28 | existing identity corpus |

Lane-local frozen summary:
`crates/swallowtail-adapter-goose/tests/fixtures/g04-090a-acp-mode/`.

Moving `main` docs and untagged trees do not qualify delivery.

## Membership

Exact `GooseMode` enum (`serde`/`strum` `snake_case`):

| Id | Default | Message |
| --- | --- | --- |
| `auto` | yes (`#[default]`) | Automatically approve tool calls |
| `approve` | no | Ask before every tool call |
| `smart_approve` | no | Ask only for sensitive tool calls |
| `chat` | no | Chat only, no tool calls |

`build_mode_state` advertises every `GooseMode::VARIANTS` entry with those
messages. Official config docs list the same four ids and default `"auto"`.
Permission docs name Completely Autonomous as applied by default.

No other ACP mode ids exist on this tag. Membership is closed.

## Advertisement And Host Seed

On `session/new`:

```rust
let current_mode: GooseMode = config.get_goose_mode().unwrap_or_default();
// ...
.create_session(..., current_mode)
```

Response `modes` comes from `build_mode_state(session.goose_mode)` after
activation. `configOptions` may include select option `mode` with the same
membership when provider/model inventory is present.

Swallowtail fixtures record `availableModes` as
`auto|approve|smart_approve|chat`, placeholder `currentModeId:
"host-owned-goose-mode"`, and `auto_mode_unselected: true`. Advertisement is
not a Swallowtail selection.

## Selection Methods

Two ACP requests call the same `on_set_mode`:

| Request | Params | Success confirmation |
| --- | --- | --- |
| `session/set_mode` | `{ modeId }` | empty `SetSessionModeResponse`; `current_mode_update` notification echoes requested `modeId` before the RPC returns |
| `session/set_config_option` | `{ configId: "mode", value: valueId }` | rebuilds options via `build_config_update` (reads `agent.goose_mode()`); sends `config_option_update`; RPC returns `configOptions` including `mode` current value |

Both require an active session agent after successful `session/new`.
`on_set_mode` parses with `mode_id.parse::<GooseMode>()`; failure is
`invalid_params` `"Invalid mode: …"`.

`update_goose_mode` updates in-memory agent mode, may call
`provider.update_mode`, and persists `goose_mode` on the session row. It does
not write host `GOOSE_MODE` / `config.yaml` through this ACP path.

## Permission Authority By Mode

From `PermissionInspector::inspect` and the Chat branch in `agent.rs`:

| Mode | Tool posture | Safe for Swallowtail deliver-now? |
| --- | --- | --- |
| `auto` | allow every tool (`"Auto mode - all tools approved"`) | no — automatic approval widening |
| `smart_approve` | auto-allow read-only annotations; LLM judge / ask otherwise; honors user AlwaysAllow/NeverAllow/AskBefore | no — automatic approval for some tools; LLM path needs provider |
| `approve` | require approval by default; ignores smart-approve cache/annotation; still honors host user AlwaysAllow | no closed safe row — ambient durable host permission store remains in path |
| `chat` | skips remaining tool calls with `CHAT_MODE_TOOL_SKIPPED_RESPONSE` success text; does not run inspectors | not portable `HarnessMode::Plan`; tool-skip is not Plan/read-only equivalence |

Contract 015 and the prepared guide already forbid Swallowtail choosing
`allow_always` or GooseMode `auto`. Mapping `chat` or `approve` to
`HarnessMode::Plan` without exact semantic equivalence is forbidden by this
card.

## Failure, Drift, And Omission

| Case | Finding |
| --- | --- |
| Unknown / unparsable `modeId` on set | `invalid_params` `"Invalid mode: …"` — fail closed for selection |
| Missing value id on set-config | `invalid_params` `"Expected a value ID"` |
| Unsupported config id | `invalid_params` `"Unsupported config option: …"` |
| Malformed persisted `goose_mode` on reload | parse failure → `GooseMode::default()` = **`auto`** — fail open to auto-approve |
| Omitted Swallowtail mode request | driver rejects `harness_mode`; sends no `session/set_mode` / set-config `mode`; argv stays `goose acp`; `currentModeId` remains host `GOOSE_MODE` or default `auto` |
| Host `GOOSE_MODE` absent | `unwrap_or_default()` / docs default → `auto` |

Omission retains exact current host-owned mode posture and argv. It does not
prove a non-auto effective mode.

## Claim Strength

| Stage | Exact finding |
| --- | --- |
| Requested | not a Swallowtail input today; open rejects `harness_mode` |
| Parsed | closed enum on set; clap/argv has no mode flag on `goose acp` |
| Configured | host `GOOSE_MODE` seeds `session/new`; default `auto` |
| Dispatched | would be `session/set_mode` or set-config `mode` after `session/new` |
| Accepted | unknown set values fail closed; session drift reloads as `auto` |
| Effective | session-row + in-memory agent mode; permission inspector / Chat skip |
| Returned | set-mode: empty body + echoed `current_mode_update`; set-config: options with mode current value |
| Observed | no secret-free live ACP observation on this host |
| Persisted | session DB `goose_mode`; not host `GOOSE_MODE` via ACP set path |

## Deliver-Now Table

| Mode | Membership closed | Pre-prompt select | Applied + confirmed without provider effect | Authority safe / non-widening | Unknown fail-closed | Drift fail-closed | Omission closed | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| *(none)* | — | — | — | — | — | — | — | — |

Honest empty set. Named gates:

1. **Default and drift are `auto`** — missing host `GOOSE_MODE` and malformed
   session `goose_mode` both become `auto`, which automatically approves tools.
   Acceptance requires drifted/malformed modes to fail closed.
2. **`auto` and `smart_approve` widen approval** — forbidden by card, Contract
   015 posture, fixtures (`auto_mode_unselected`), and prepared guide.
3. **`approve` keeps ambient durable host permissions** — user AlwaysAllow in
   host permission store still auto-allows; mode semantics are not separated
   from ambient host configuration.
4. **`chat` is not Plan** — tool-skip success text is not exact
   `HarnessMode::Plan` equivalence; label mapping is withheld.
5. **Confirmation needs a live provider-backed session** — set-mode /
   set-config require `get_session_agent` after `session/new`, which needs host
   provider/model. No goose on PATH; no live ACP or login authorized. Source
   freeze alone does not close selected-value confirmation before first prompt.
6. **set-mode confirmation is request echo** — empty RPC body;
   `current_mode_update` repeats requested `modeId`. Stronger set-config
   confirmation still depends on provider inventory after session activation.

## Production Seam Audit

`swallowtail-adapter-goose` still plans `goose acp` only.
`driver/validation.rs` rejects `harness_mode`. Fixtures keep
`auto_mode_unselected: true` and negative case
`auto-mode-not-swallowtail-authority`. Prepared guide withholds GooseMode
`auto` and does not bind mode selection. Protocol selected methods remain
initialize / session/new / session/prompt / session/cancel.
`session/set_mode` is not a Swallowtail selected method.
No production binding starts from this evidence.

## Promotion

Research 253 promotes an empty deliver-now set for exact `goose.acp`
`1.46.0` ACP mode selection.

A later lane may reopen only when secret-free evidence closes a non-auto
effective mode with fail-closed drift, isolates selection from ambient durable
host approval stores, confirms selected value before first prompt without
provider login/prompt work, and either proves exact portable Plan equivalence
or binds an explicitly named adapter-local mode without implying Plan. Builtin
argv, MCP management, extension install, and currentness stay out of scope.
