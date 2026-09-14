# 319 Goose ACP 1.50.0 Identity Stop

Status: promoted; identity evidence only. This record lands a typed stop and
does not change the production claim.

Owner: Tom
Date: 2026-09-14
Card: g05.071 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 148, 250, 253, 308, and 318; the Goose
prepared-integration guide; and the official GitHub `aaif-goose/goose`
release channel.

## Answer

Official GitHub stable `latest` is `v1.50.0`, published
`2026-09-08T19:32:41Z`; tags above it are `v2` prerelease spellings, not
stable authority. The exact `goose.release` point stays at `1.46.0`. The
selected ACP route cannot move to `1.50.0` as a compatible extension: at the
exact hop **`1.46.0..1.47.0`** provider-authentication failure stops being
generic assistant text plus `end_turn` and becomes a typed JSON-RPC
`auth_required` error on the selected `session/new` and `session/prompt`
paths. That is a selected provider-failure mapping change, and it needs an
operator ruling on the route's failure-binding policy before any claim edit.

## Method and channel boundary

The official latest release, the full stable release list, and all five tag
objects were rechecked at the start of the run and again immediately before
the identity commit. The exact `1.46.0` baseline and all **4** published
stable successors `1.47.0..=1.50.0` were retrieved as exact tagged source
trees from the official channel into `/tmp`; the ten frozen `1.46.0` module
digests reproduce byte-for-byte, confirming tree identity. Darwin-arm64
`goose-aarch64-apple-darwin.tar.bz2` assets for the four successors were
hashed without extraction and never executed. No prompt, login, credential,
provider session, installation, host update, or provider credit was used.
`goose` remains absent from this host, so there is no installed `--version`
observation.

The compared stable points are `1.46.0` through `1.50.0` inclusive. `v2-rc.1`,
`v2.0-rc-04-27-0`, and `v2.0.0-rc-04-27-0` are prerelease channels and are not
stable authority; a `v2` major-line reset would need its own operator ruling.

The reproducible identity and inventory records are frozen in
[`goose-acp-1.50.0`](../../crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.50.0/):
`identity.json` holds the five-tag publication and digest record,
`tree-ledger.json` holds the 26-file mapped-module closure with per-version
hashes and consecutive-hop sets, and `protocol.json` holds the
selected-surface classification and the stop boundary. The
mutation-sensitive assertions live in
[`goose_1_50_0_delta_ledger.rs`](../../crates/swallowtail-adapter-goose/tests/goose_1_50_0_delta_ledger.rs).

## Complete mapped-tree result

The 26-file closure covers CLI dispatch, the ACP server and its
session/prompt/dispatch/extension/provider subsurfaces, mode and permission
authorities, builtin registries, host config, session persistence, provider
error kinds, workspace pins, and the ACP client guide. Seven files are
byte-identical across all five tags: `acp/common.rs`, `acp/server/prompts.rs`,
`goose_mode.rs`, `builtin_extension.rs`, `platform_extensions/mod.rs`,
`goose-mcp` `BUILTIN_EXTENSIONS`, and `permission_inspector.rs`.

Per-hop changed counts are 12, 16, 14, and 4, with no added paths and one
removal (`documentation/docs/guides/acp-clients.md` deleted at `1.50.0`; the
selected argv and wire are proved from source, not docs). The ACP dependency
jumps once, at the stop hop: schema `1.1` to `=1.5.0`, SDK `1.0` to `2.0.0`
(pinned git `c97a5203`), then stays fixed through `1.50.0`.

## Selected-surface classification

Stable at every hop unless named:

- `Command::Acp` argv and `run(builtins, enable_scheduler)` dispatch are
  byte-stable; `session_cwd` host override (from `1.48.0`) stays `None` on
  the stdio path so absolute-cwd authority is unchanged; mode seeding stays
  `get_goose_mode().unwrap_or_default()`.
- `initialize` still echoes the requested protocol version; the only response
  additions are advertised-only `sessionCapabilities.delete` and the empty
  `recipeParameterScopes` meta object, both unmapped.
- Prompt content conversion, stop reasons (`cancelled`/`max_tokens`/
  `end_turn`), `on_cancel` token lookup, permission kinds
  (`allow_once`/`allow_always`/`reject_once`/`reject_always`), and
  `build_mode_state` are identical; the adapter parses only `sessionId` from
  `session/new`, so additive thinking-effort and recipe-scope options are
  inert.
- Bounded unmapped deltas: Goose-as-ACP-client provider paths
  (`configured_model_for_provider`, `use_default_model`, strict mode for
  `codex_acp`), client-MCP OAuth fields under `mcpServers: []`, extension
  management and replay-tail siblings, thinking-effort selection, recipes,
  serve/roam/TUI/desktop, MCP protocol version, telemetry, session naming,
  and usage/available-commands notifications.

## The stop boundary: provider-auth failure becomes typed

Research 148 proved that at exact `1.46.0` missing provider/model fails
`session/new` as JSON-RPC internal error and provider errors mid-prompt
surfaced as assistant text with a normal `end_turn`.

At exactly `1.46.0..1.47.0`:

| Path | `1.46.0` | `1.47.0..=1.50.0` |
| --- | --- | --- |
| `session/new` agent-creation failure with chained ACP `AuthRequired` | `internal_error` always | `auth_required` via `agent_creation_error` |
| `ProviderError::Authentication` mid-prompt | generic catch-all arm: assistant text, `end_turn` | dedicated arm persists `MessageContent::Error(Authentication)` via `Message::from_provider_error`, trips `prompt_error_from_message_content`, prompt RPC fails `auth_required` |

Effect on the selected route: under identical host provider-auth state, a
prompt that completed with error text at `1.46.0` fails the turn from
`1.47.0` onward, and the adapter maps those two outcomes differently
(`Completed` vs turn failure). The frozen `1.46.0` negative cases contain no
auth failure code, so this is novel selected failure behavior, not a covered
projection.

## Decision and disposition

The identity-first decision is `stop`. No claim edit lands:

- `GOOSE_RELEASE_VERSION` stays exact `1.46.0`
- claim `goose.acp.release-window-1` stays one exact `QualifiedOnly`
  maintained point
- behavior revision `goose.acp.stdio-v1` is unchanged
- no range, second point, exclusion, or unverified-newer posture is added
- the Goose prepared guide, route matrix, activity matrix, feature matrix,
  and architecture ceiling stay as they are, because the qualified point did
  not move

Research 148's `1.46.0` decoder specimens and Research 250/253's empty
builtin/mode deliver-now dispositions are untouched. `session/delete`,
thinking-effort selection, and recipe scopes stay independently gated and no
advertised capability is converted into a Swallowtail operation.

The route now returns to the operator via Chatterbox for a planning ruling:
on a `1.47.0`-or-later point, does the route bind the new typed
`auth_required` failure projection on a new behavior revision, or stay at
`1.46.0`? That is a new public failure decision under Contract 029 rather
than something this lane may settle by inference.

## Validation

Provider-free only. The identity corpus and its mutation-sensitive Rust
ledger test compile and pass; focused package validation passes; no
downloaded artifact was executed or extracted; no provider operation,
prompt, login, credential use, installation, or host update occurred.
