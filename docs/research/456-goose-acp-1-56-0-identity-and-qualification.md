# Research 456: Goose ACP 1.56.0 Identity and Qualification

Status: candidate for independent review  
Date: 2026-10-26  
Route: `goose.acp`  
Axis: `goose.release`

## Finding

The official Goose GitHub stable channel was re-probed on 2026-10-26. Its
latest stable release was `v1.56.0`, published 2026-10-23, and the latest
matching stable tag agreed. The published successor hops after Swallowtail's
`1.53.0` ceiling are `1.54.0`, `1.55.0`, and `1.56.0`. No additional stable
release or tag appears in the interval. The unpublished interstitial points
`1.53.1`, `1.54.1`, and `1.55.1` remain incompatible; no stable point in the
window was withdrawn, and no independently unqualified published point or
explicit exclusion remains in the window. [Official `v1.56.0` release](https://github.com/aaif-goose/goose/releases/tag/v1.56.0)

The standard `goose acp` route keeps its selected stdio invocation, initialize
at ACP protocol version 1, session creation, one prompt, cancellation, typed
authentication failures, and joined cleanup contract. The claim extends the
same private behavior `goose.acp.stdio-v2.auth-required` through seven exact
published points. It preserves baseline `1.50.1`, claim id
`goose.acp.release-window-1`, and all route exclusions.

## Identity and method

The official GitHub release and tag endpoints were checked before artifact
selection. The exact release publication, source commit, root tree, Darwin
ARM64 release asset, asset byte count, and SHA-256 are recorded for each point
in `crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.56.0/identity.json`.
The `v1.53.0` commit, tree, asset byte count, and asset digest reproduce
Research 392. Asset digests are the GitHub release asset digest field. The
release binaries were not downloaded or executed. The source evidence uses
complete recursive tag-tree inventories with sorted path, mode, and Git blob
SHA rows. The `v1.53.0` manifest digest reproduces Research 392. The canonical
manifest digest and complete per-hop path classification are checked by the
route identity test.

| Hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.53.0` → `1.54.0` | 11 | 3 | 127 | 2,393 |
| `1.54.0` → `1.55.0` | 25 | 31 | 304 | 2,196 |
| `1.55.0` → `1.56.0` | 24 | 7 | 173 | 2,345 |

The `protocol.json` ledger names the selected-source closure and classifies
every changed path inside it for each hop. It also partitions every remaining
added, removed, or changed path into a residual category. Changelogs served as
discovery only; exact tag trees and selected source changes bound the claim.

## Selected-surface classification

- **`1.53.0` → `1.54.0`:** The standard CLI still dispatches `Acp` to the same
  stdio server. Initialize now stamps `ProtocolVersion::LATEST`, and Goose's
  own test requires that value to be V1 even when the client offers V2.
  Swallowtail sends protocol version 1 and accepts only 1. A client-supplied
  goose extension list becomes the whole session set; this route sends no such
  meta, so configured extensions and `mcpServers` stay on the previous path.
  The HTTP/SSE converter is byte-identical, including the SSE rejection.
  Unstable Nostr session share is removed. `session/cancel` still cancels the
  active-run token.
- **`1.54.0` → `1.55.0`:** Prompt meta no longer selects the legacy agent loop.
  This route sends no prompt meta, and `reply()` is the state-machine path
  that was already the default. `on_cancel` still cancels the active-run
  token, and `prompt_stop_reason` still returns `StopReason::Cancelled`.
  Missing `GOOSE_MODE` still resolves to Auto; an invalid value fails closed.
  This route does not set that variable. Login-shell PATH meta is removed;
  its previous default was false. Optional W3C trace context is read from
  prompt meta this route does not send. HTTP MCP servers are stored through
  `update_enabled_extensions`; the converter is unchanged. Live remote MCP
  honouring stays unqualified.
- **`1.55.0` → `1.56.0`:** The separate `goose-acp` binary loads the online
  model catalog before the same server entry. Swallowtail spawns `goose acp`,
  not that binary. `tool_calls/conversion.rs` emits
  `_meta.goose.subagentSessionId` on selected `tool_call_update` notifications,
  and `message_meta.rs` adds `inference` and `operationLogs` on selected
  message-chunk `_meta`. Swallowtail's ACP decoder ignores `_meta` on those
  notifications, so mapped tool and message fields stay ACP v1. A compaction
  notice is emitted only when goose custom notifications are enabled, which
  this route does not advertise. Provider-safe MCP tool names and subagent
  spawn/control stay inside the unselected tool loop.

`crates/goose/src/acp/mod.rs`, `prompts.rs`, `transport/auth.rs`, and
`tool_confirmation.rs` are byte-identical from `1.53.0` through `1.56.0`, so
the typed `auth_required` mapping and the selected prompt and confirmation
paths do not move. Existing decoder, permission, cancellation, cleanup,
prepared-route, and consumer-projection fixtures remain part of the targeted
validation. No adapter operation was added, and no existing route capability
was narrowed.

## Claim and retained boundaries

- Maintain seven exact points: `1.50.1`, `1.51.0`, `1.52.0`, `1.53.0`,
  `1.54.0`, `1.55.0`, and `1.56.0`.
- Keep baseline `1.50.1`, claim id `goose.acp.release-window-1`, behavior
  revision `goose.acp.stdio-v2.auth-required`, and `AllowUnverified`.
- Keep unpublished interstitial versions `1.50.2`, `1.51.1`, `1.52.1`,
  `1.53.1`, `1.54.1`, and `1.55.1` outside all segments. The first later
  stable, `1.56.1`, may be classified `UnverifiedNewer`; it is not qualified
  by this evidence.
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
- Official GitHub stable tag refs and recursive source trees for all four
  points, frozen under `crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.56.0/`.

No provider prompt, authenticated catalogue, live ACP session, MCP server,
credential, Goose installation, host update, or host permission change was
used. Static source evidence does not replace a live proof.
