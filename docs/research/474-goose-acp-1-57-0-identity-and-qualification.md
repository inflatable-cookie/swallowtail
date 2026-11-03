# Research 474: Goose ACP 1.57.0 Identity and Qualification

Status: candidate for independent review  
Date: 2026-11-03  
Route: `goose.acp`  
Axis: `goose.release`

## Finding

The official Goose GitHub stable channel was re-probed on 2026-11-03. Its
latest stable release was `v1.57.0`, published 2026-10-28, and the latest
matching stable tag agreed. The published successor hop after Swallowtail's
`1.56.0` ceiling is `1.57.0`; no other stable release or tag appears in the
interval. The unpublished interstitial point `1.56.1` remains incompatible; no
stable point in the window was withdrawn, and no independently unqualified
published point or explicit exclusion remains in the window. [Official
`v1.57.0`
release](https://github.com/aaif-goose/goose/releases/tag/v1.57.0)

Prerelease-only `v2` tags (`v2-rc.1`, `v2.0-rc-04-27-0`,
`v2.0.0-rc-04-27-0`) exist on the repository with no published `v2` stable
release. They are outside the stable channel this axis qualifies, and
`goose_release_binding` rejects prerelease and build metadata, so they cannot
bind. The first later stable remains synthetic `1.57.1`, classified
`UnverifiedNewer`.

The standard `goose acp` route keeps its selected stdio invocation, initialize
at ACP protocol version 1, session creation, one prompt, cancellation, typed
authentication failures, and joined cleanup contract. The claim extends the
same private behavior `goose.acp.stdio-v2.auth-required` through eight exact
published points. It preserves baseline `1.50.1`, claim id
`goose.acp.release-window-1`, and all route exclusions.

## Identity and method

The official GitHub release and tag endpoints were checked before artifact
selection. The exact release publication, source commit, root tree, Darwin
ARM64 release asset, asset byte count, and SHA-256 are recorded for each point
in `crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.57.0/identity.json`.
The `v1.53.0` through `v1.56.0` commit, tree, asset byte count, and asset
digest values reproduce Research 456; the four earlier tag-tree manifests
reproduce the `goose-acp-1.56.0` fixture byte for byte, including canonical
manifest digests. Asset digests are the GitHub release asset digest field. The
release binaries were not downloaded or executed. The source evidence uses
complete recursive tag-tree inventories with sorted path, mode, and Git blob
SHA rows. The canonical manifest digest and complete per-hop path
classification are checked by the route identity test.

| Hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.53.0` → `1.54.0` | 11 | 3 | 127 | 2,393 |
| `1.54.0` → `1.55.0` | 25 | 31 | 304 | 2,196 |
| `1.55.0` → `1.56.0` | 24 | 7 | 173 | 2,345 |
| `1.56.0` → `1.57.0` | 9 | 21 | 130 | 2,391 |

The `protocol.json` ledger names the selected-source closure and classifies
every changed path inside it for each hop. The closure adds
`crates/goose/src/model_config.rs` this hop because the `session/new`
provider and model resolution moved into that file; it is identical through
`1.56.0`, so the earlier hop classifications carry over unchanged. The
closure also adds five prompt-lifecycle files on the session/prompt reply
pipeline — `agents/prompt_manager.rs`, `agents/reply_parts.rs`,
`state_machine/inference_preparation.rs`, `state_machine/ops_compaction.rs`,
and `state_machine/ops_exit_on_error.rs` — each classified on every hop that
touched it back through `1.53.0`, so the prompt lifecycle inside the
provider-owned reply pipeline is recorded rather than lumped into a residual
category. Changelogs served as discovery only; exact tag trees and selected
source changes bound the claim.

## Selected-surface classification

- **`1.56.0` → `1.57.0`:** `Cargo.toml` moves the workspace version; the
  standard CLI still dispatches `Acp` to the same stdio server, with the
  provider-unavailable configure-dialog retry living only in the unselected
  interactive session builder. Goose removes its reverse-direction ACP tool
  bridge (`acp/fs.rs`, `acp/tools.rs`, `AcpTools`,
  `apply_acp_extension_overrides`, client fs/terminal capability storage):
  Swallowtail advertises filesystem and terminal callbacks false and never
  calls ACP tool methods, so no selected surface used that bridge.
  `tool_calls/conversion.rs` drops the v1.56.0 ACP-aware suppression flag, so
  selected `tool_call_update` fields return to the always
  content-and-locations shape 1.55.0 had; the decoder reads
  `toolCallId`, `title`, `kind`, `status`, `content`, and `locations` only.
  `session/new` provider and model resolution relocates to
  `model_config::resolve_provider_and_model_config` with the same host-config
  default path for the no-meta, no-recipe requests this route sends; a new
  registry check runs only when recipe settings or prompt meta name a
  provider. `session_manager.rs` adds caller-supplied session IDs for the
  desktop; the default `create_session` path keeps generated IDs and this
  route sends no session id meta.
- **Typed auth failures stay typed.** `prompt_error_from_message_content`
  keeps the `Authentication` branch on `Error::auth_required()` with the same
  code and message and now attaches `data {"reason": "turn_failed"}`;
  Swallowtail's `RpcError` decoder carries only code and message and drops
  `data`, and the adapter still matches the `auth_required` message string.
- **Two prompt-path failure outcomes change.** First, generic provider
  errors: a new catch-all maps other `MessageContent::Error` content to
  internal error `-32603` with the same `data`, and detection moves from
  mid-stream to after the stream via `last_message_was_error` plus a
  persisted-session `turn_failure` lookup, so a turn ending in a generic
  error now fails the `session/prompt` RPC instead of ending with error
  content in the message stream; the adapter reports such failures through
  its existing rejected-request class. Second, system-prompt render
  failures: the prompt builder returns a `Result` and the silent
  general-purpose fallback prompt is removed, so a render failure propagates
  into the turn and surfaces through that same post-stream mapping instead
  of proceeding on the fallback. A compaction failure now emits an
  error-kind (or `CreditsExhausted`) message that the same lookup can
  surface. Mapped initialize, `session/new`, message chunks,
  `session/cancel`, and cleanup contracts are unchanged.
- **Byte-identical selected files.** `crates/goose/src/acp/mod.rs`, byte
  identical from `1.53.0` through `1.56.0`, changes this hop only to remove
  the deleted `fs`/`tools` module declarations and add the
  `TURN_FAILED_REASON` constant. The other twelve byte-identical selected
  files from the previous ledger — including `acp/transport/auth.rs`,
  `acp/transport/mod.rs`, `state_machine/tool_confirmation.rs`,
  `config/permission.rs`, and `session_context.rs` — stay byte identical
  through `1.57.0`, so the typed `auth_required` transport, permission, and
  confirmation paths do not move.

Existing decoder, permission, cancellation, cleanup, prepared-route, and
consumer-projection fixtures remain part of the targeted validation. No
adapter operation was added, and no existing route capability was narrowed.

## Claim and retained boundaries

- Maintain eight exact points: `1.50.1`, `1.51.0`, `1.52.0`, `1.53.0`,
  `1.54.0`, `1.55.0`, `1.56.0`, and `1.57.0`.
- Keep baseline `1.50.1`, claim id `goose.acp.release-window-1`, behavior
  revision `goose.acp.stdio-v2.auth-required`, and `AllowUnverified`. The
  1.57.0 failure-mapping extension stays on that revision because the typed
  auth detection contract — message match on `session/new` and
  `session/prompt` responses — is unchanged, and mapped methods, Auto
  default, `session/cancel` stop reason `Cancelled`, and joined cleanup are
  unchanged. The generic-error and render-failure outcome changes alter when
  the route reports a failed request and through which bounded failure code,
  not the mapped method set, wire shape, or auth contract.
- Keep unpublished interstitial versions `1.50.2`, `1.51.1`, `1.52.1`,
  `1.53.1`, `1.54.1`, `1.55.1`, and `1.56.1` outside all segments. The first
  later stable, `1.57.1`, may be classified `UnverifiedNewer`; it is not
  qualified by this evidence.
- Keep typed `auth_required` mapping on `session/new` and `session/prompt`.
  Local provider configuration stays host-owned; no credential lease is
  introduced.
- Keep Goose `serve`, `--with-builtin`, `--enable-scheduler`, provider
  management, session load/list/delete/close, ACP tool methods, usage, and
  live-voice methods unmapped.
- Keep one consumer streamable-HTTP declaration as emission-only.
  `client_mcp_servers` remains `No` pending its exact live gate; this research
  does not prove remote MCP honouring. SSE remains unsupported.
- Preserve the older currentness fixtures and historical research records.
  Production claim id and new evidence follow the current `selection.rs` claim
  without rewriting history.

## Sources and limits

- [Goose v1.53.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.53.0)
- [Goose v1.54.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.54.0)
- [Goose v1.55.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.55.0)
- [Goose v1.56.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.56.0)
- [Goose v1.57.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.57.0)
- Official GitHub stable tag refs and recursive source trees for all five
  points, frozen under `crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.57.0/`.

No provider prompt, authenticated catalogue, live ACP session, MCP server,
credential, Goose installation, host update, or host permission change was
used. Static source evidence does not replace a live proof.
