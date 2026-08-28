# 250 Goose ACP Builtin Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Card: g04.089 / 253

## Question

Which exact `goose.acp` `1.46.0` builtin and lifecycle rows, if any, can bind
`--with-builtin` with closed membership, host-extension authority,
application, confirmation, failure, and omission truth?

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
`allow_always` and GooseMode `auto` stay unselected.

The adapter, shared guides/matrices, and production surfaces were inspected
and not changed. No Contract 029 window movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `v1.46.0` | exact release commit | 2026-08-28 | commit `98c11ce2ee7b9b302978aa64b1eab7d0895607c7` (2026-08-11T22:32:02Z) |
| `crates/goose-cli/src/cli.rs` | `Command::Acp` clap `--with-builtin`; CLI register; dispatch to `run` | 2026-08-28 | SHA-256 `b7f45c9204319fecd593a05584952e1c008b5e11879c6509c29785d7baede1f4` (90390 bytes) |
| `crates/goose-mcp/src/lib.rs` | `BUILTIN_EXTENSIONS` MCP spawn registry | 2026-08-28 | SHA-256 `ba998bc5a3b825848115ffa2059b0bd71f1d78fc1499c6f10b863c786ec82a7a` (2050 bytes) |
| `crates/goose/src/builtin_extension.rs` | global name → spawn registry | 2026-08-28 | SHA-256 `3f3c1aca88295b24473e6e16c8713977089c592cbd3baf495e74e43a0d48904e` (1033 bytes) |
| `crates/goose/src/agents/platform_extensions/mod.rs` | `PLATFORM_EXTENSIONS` membership and defaults | 2026-08-28 | SHA-256 `cb578bbe8eab9d9f5c2ba9547e8d7b08b7582814a25e4e4870b9ca3dc07d388b` (11334 bytes) |
| `crates/goose/src/acp/server.rs` | `AcpBuiltinSelection`, `selected_builtin_extensions`, `initial_session_extensions`, `run` | 2026-08-28 | SHA-256 `3eb611c4ae0f37a1d4affde82a4c29a5d56e9d70cb796d8cc7cc6082bc13b47f` (103385 bytes) |
| `crates/goose/src/acp/server/new_session.rs` | `session/new` activation and response meta | 2026-08-28 | SHA-256 `10146d0e24a148f5d89bbdb3447f4720cff74b2fdb62d80b8febf2cc3e2a6dad` (13004 bytes) |
| `crates/goose/src/acp/response_builder.rs` | `_meta.extensionResults` | 2026-08-28 | SHA-256 `b1a18f5ec51f49a2bc5bf45cbc86a1c2fcb2fdbfd1d5524cc4b243b5bbece571` (26953 bytes) |
| `crates/goose/src/agents/extension_manager.rs` | platform vs MCP builtin load; `Unknown extension` | 2026-08-28 | SHA-256 `04ebc8b18026f9f9cfac9417407da232bc3f42ed878da246e6b5127dfab30f79` (125459 bytes) |
| `crates/goose/src/agents/agent.rs` | `ExtensionLoadResult`; soft-fail bulk load | 2026-08-28 | SHA-256 `2227f114aae644241feaeb3a4f1a9feed1545cf6e9a792b39783deac89c19be6` (217307 bytes) |
| `crates/goose/src/config/extensions.rs` | `name_to_key`; enabled host extensions | 2026-08-28 | SHA-256 `8e8668b532c0e0702dd8678c36d9a76ac212dbf342e4c471b93a93b2e3204486` (24920 bytes) |
| `crates/goose-cli/src/commands/configure.rs` | configure UI builtin id list | 2026-08-28 | SHA-256 `d1ed85b4809f9c4bb55e62cfe72f1081b13ec55f2c5fa905970b0692428e7e91` (79180 bytes) |
| `documentation/docs/guides/goose-cli-commands.md` | docs lead for `--with-builtin` | 2026-08-28 | SHA-256 `5016c46d266409d804c57320c59b5d3ee497068dcc68b6813f9f1a3cb8ebc85f` (34960 bytes) |
| `documentation/docs/guides/acp-clients.md` | ACP clients spawn `["acp"]`; host extensions carry over | 2026-08-28 | SHA-256 `9db93def1b6920317463cf20949b0979c96520ce8b57cf557b44a727c41143bf` (10014 bytes) |
| Swallowtail fixtures `goose-acp-1.46.0/protocol.json` | current selected argv and unmapped `--with-builtin` | 2026-08-28 | existing identity corpus |

Lane-local frozen summary:
`crates/swallowtail-adapter-goose/tests/fixtures/g04-089b-acp-builtins/`.

Moving `main` docs and untagged trees do not qualify delivery.

## Syntax And Dispatch

`Command::Acp` registers:

```rust
#[arg(
    long = "with-builtin",
    value_name = "NAME",
    help = "Add builtin extensions by name (e.g., 'developer' or multiple: 'developer,github')",
    long_help = "Add one or more builtin extensions that are bundled with goose by specifying their names, comma-separated",
    value_delimiter = ','
)]
builtins: Vec<String>,
```

No clap default on `goose acp`. Empty `builtins` is the omitted case.
`cli()` always calls `register_builtin_extensions(goose_mcp::BUILTIN_EXTENSIONS.clone())`
before parse, then:

```rust
Some(Command::Acp { builtins, enable_scheduler }) =>
    goose::acp::server::run(builtins, enable_scheduler).await,
```

`run` builds:

```rust
AcpBuiltinSelection {
    explicit: builtins,
    ..Default::default()  // defaults: []
}
```

That differs from `goose serve`, which defaults omitted builtins to
`defaults: ["developer"]`. Serve also sets `ArgAction::Append`. Stdio ACP does
not inherit the serve default. Research 148 already separated those commands.

## Membership (Source Tables, Not Deliver-Now)

Two registries resolve selected names at load time:

**Platform (`PLATFORM_EXTENSIONS` keys)** — in-process clients:

| Key | `default_enabled` | Notes |
| --- | --- | --- |
| `analyze` | true | |
| `todo` | true | |
| `apps` | true | |
| `chatrecall` | false | unit-tested as explicit default-off load |
| `extensionmanager` | true | def name `"Extension Manager"`; lookup uses `name_to_key` |
| `scheduler` | true | hidden; `client_factory` may return `None` without scheduler |
| `summon` | true | |
| `summarize` | false | |
| `developer` | true | shell/file tools; ACP may overlay client fs/terminal |
| `orchestrator` | false | hidden |
| `tom` | true | |
| `skills` | true | |
| `code_execution` | false | only under `feature = "code-mode"` |

**MCP builtins (`goose_mcp::BUILTIN_EXTENSIONS`)** — duplex child MCP servers
registered at CLI start:

- `autovisualiser`
- `computercontroller`
- `memory`
- `tutorial`

Configure UI lists `autovisualiser`, `computercontroller`, `developer`,
`memory`, `tutorial`. Clap help and docs also cite `github`. `github` is not
in either registry above; ACP fixtures treat it as an MCP server name. Docs
membership is not a closed argv table.

`builtin_to_extension_config` maps known platform keys to
`ExtensionConfig::Platform`; every other string becomes
`ExtensionConfig::Builtin` with no clap-time membership check.

## Application Path

On `session/new` (after provider/model resolution):

1. `selected_builtin_extensions` emits configs for `defaults` (respecting host
   `enabled: false`) then `explicit` (overrides disabled host entries).
2. When client `mcpServers` is empty — Swallowtail's current plan —
   `initial_session_extensions` also merges
   `get_enabled_extensions_with_config` and enabled plugin MCP servers.
3. Non-empty client `mcpServers` replaces that host merge with client MCP
   only, but argv builtins still prepend.
4. `add_extensions_bulk` loads each config. Platform hits
   `PLATFORM_EXTENSIONS`; else `get_builtin_extension` or
   `Unknown extension: {name}`.
5. Platform `client_factory` returning `None` returns `Ok(())` without
   registering tools — silent decline, not a hard error.
6. Load outcomes become `ExtensionLoadResult` and are copied into
   `session/new` response `_meta.extensionResults` only. `session/new` still
   succeeds when individual loads fail.

`initialize` does not advertise selected builtins. There is no standard ACP
tool table or session config option for `--with-builtin`.

## Claim Strength

| Stage | Exact finding |
| --- | --- |
| Requested | not a Swallowtail input today |
| Parsed | clap accepts any comma-delimited `NAME`; no enum |
| Configured | host `~/.config/goose/` enabled extensions remain ambient under `mcpServers: []` |
| Dispatched | would be `goose acp --with-builtin …`; production stays `goose acp` |
| Accepted | unknown names are not spawn/parse failures |
| Effective | load is session-activation side effect; soft-fail or silent platform decline possible |
| Returned | custom `_meta.extensionResults` only; requires successful provider/model `session/new` |
| Observed | no secret-free live ACP observation on this host |
| Persisted | bulk load may persist extension state when any load succeeds |

## Failure And Omission

| Case | Finding |
| --- | --- |
| Unknown name | `ExtensionError::ConfigError("Unknown extension: …")` recorded as `success: false`; session continues |
| Unavailable platform dep | `client_factory` → `None` → `Ok(())` with no tools |
| Omitted `--with-builtin` | empty `AcpBuiltinSelection`; host-enabled extensions still load when `mcpServers: []` |
| Current Swallowtail omission | fixtures and driver keep argv exactly `["acp"]`; `pass_with_builtin: false` |

Omission of the flag retains exact current argv. It does not prove isolation
from host-configured extensions.

## Deliver-Now Table

| Builtin | Membership closed | Host deps authorized without mutation | Applied + confirmed without provider effect | Unknown fail-closed before prompt | Omission closed | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- |
| *(none)* | — | — | — | — | — | — |

Honest empty set. Named gates:

1. **No pre-effect rejection** — clap and `session/new` accept unknown names;
   failure is soft `_meta.extensionResults`, after provider/model resolution.
2. **Ambient host merge** — Swallowtail's `mcpServers: []` still loads host
   enabled extensions and plugin MCP servers; argv builtins are not an
   isolated closed set.
3. **Host-config authority** — defaults honor disabled host entries; explicit
   flags override them. Host config is mutable ambient authority and is not
   portable builtin membership.
4. **Confirmation gap** — only custom `_meta.extensionResults`; `initialize`
   has no builtin delta; confirming application needs `session/new`, which
   needs host provider/model. No goose on PATH; no live probe authorized.
5. **Silent decline** — missing platform host services can skip registration
   without failing the session.
6. **Authority widening** — binding `developer` (and other tool-bearing
   names) expands tools relative to current omission; this lane forbids
   install/configure/permission widening to force a row.
7. **Docs/registry drift** — help cites `github`; registries do not list it as
   a builtin spawn name.

## Production Seam Audit

`swallowtail-adapter-goose` still plans `goose acp` only.
`protocol.json` keeps `swallowtail_passes_with_builtin: false` and lists
`--with-builtin` under `unmapped_cli`. Prepared guide withholds the flag.
Negative fixture `with-builtin-unmapped` remains unselected. No production
binding starts from this evidence.

## Promotion

Research 250 promotes an empty deliver-now set for exact `goose.acp`
`1.46.0` `--with-builtin`.

A later lane may reopen only when secret-free evidence closes pre-effect
unknown rejection, isolates argv builtins from ambient host extension merge,
confirms application without provider prompts or host mutation, and freezes
dependency authority without treating host-configured presence as portable
membership. Goose mode, MCP management, extension install, and currentness
stay out of scope.
