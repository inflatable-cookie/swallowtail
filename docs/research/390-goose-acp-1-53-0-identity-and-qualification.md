# Research 390: Goose ACP 1.53.0 Identity and Qualification

Status: candidate for independent review  
Date: 2026-10-08  
Route: `goose.acp`  
Axis: `goose.release`

## Finding

The official Goose GitHub stable channel was re-probed on 2026-10-08. Its
latest stable release was `v1.53.0`, published 2026-10-02, and the latest
matching stable tag agreed. The published successor hops after Swallowtail's
`1.50.1` ceiling are `1.51.0`, `1.52.0`, and `1.53.0`. No additional stable
release or tag appears in the interval. The unpublished interstitial points
`1.50.2`, `1.51.1`, and `1.52.1` remain incompatible; no stable point in the
window was withdrawn, and no independently unqualified published point or
explicit exclusion remains in the window. [Official `v1.53.0` release](https://github.com/aaif-goose/goose/releases/tag/v1.53.0)

The standard `goose acp` route keeps its selected stdio invocation, initialize,
session creation, one prompt, cancellation, typed authentication failures, and
joined cleanup contract. The claim extends the same private behavior
`goose.acp.stdio-v2.auth-required` through four exact published points. It
preserves baseline `1.50.1`, claim id `goose.acp.release-window-1`, and all
route exclusions.

## Identity and method

The official GitHub release and tag endpoints were checked before artifact
selection. The exact release publication, source commit, root tree, Darwin
ARM64 release asset, asset byte count, and SHA-256 are recorded for each point
in `crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.53.0/identity.json`.
The release binaries were not executed. The source evidence uses complete
recursive tag-tree inventories with sorted path, mode, and Git blob SHA rows.
The canonical manifest digest and complete per-hop path classification are
checked by the route identity test.

| Hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.50.1` → `1.51.0` | 28 | 7 | 213 | 2,234 |
| `1.51.0` → `1.52.0` | 40 | 3 | 172 | 2,300 |
| `1.52.0` → `1.53.0` | 14 | 3 | 156 | 2,353 |

The `protocol.json` ledger names the selected-source closure and classifies
every changed path inside it for each hop. It also partitions every remaining
added, removed, or changed path into a residual category. Changelogs served as
discovery only; exact tag trees and selected source changes bound the claim.

## Selected-surface classification

- **`1.50.1` → `1.51.0`:** The standard CLI still dispatches `Acp` to the same
  stdio server. The ACP server adds ResourceLink metadata handling and accepts
  optional prompt metadata for `unrolledAgentLoop`; the Swallowtail adapter
  sends text blocks and no prompt metadata. ACP schema and provider/tool
  management additions are not called. Goose makes its local config and
  session directories owner-private on Unix; this upstream hardening was not
  run on the host, and Swallowtail does not own those provider paths.
- **`1.51.0` → `1.52.0`:** Active-run bookkeeping moves to a shared registry;
  the selected cancel method continues to target the active session/run and
  the exact route tests cover cancellation and joined cleanup. Goose adds
  usage notifications, but this adapter does not project them, so
  `usage_evidence` stays unavailable. Provider readiness, session load/list,
  schedule and live-voice methods are adjacent API surfaces and remain
  unselected. A streamable-HTTP extension client is added, but it does not
  qualify live remote-MCP tool honouring.
- **`1.52.0` → `1.53.0`:** The new `goose-acp` lean executable is distinct from
  the standard `goose acp` command selected here. Goose adjusts internal
  provider tool-loop, usage, recipe-storage, and session-context code. Its
  streamable-HTTP client now refuses redirects. This provider-side security
  hardening is outside the still-unqualified live-MCP capability;
  `client_mcp_servers` remains `No`. A wasm-only network-error helper change
  does not affect the qualified Darwin ARM64 runtime.

The exact full-tree manifests and selected path sets are retained in the
fixture directory. Existing decoder, permission, cancellation, cleanup,
prepared-route, and consumer-projection fixtures remain part of the targeted
validation. No adapter operation was added, and no existing route capability
was narrowed.

## Claim and retained boundaries

- Maintain four exact points: `1.50.1`, `1.51.0`, `1.52.0`, and `1.53.0`.
- Keep baseline `1.50.1`, claim id `goose.acp.release-window-1`, and behavior
  revision `goose.acp.stdio-v2.auth-required`.
- Keep unpublished interstitial versions `1.50.2`, `1.51.1`, and `1.52.1`
  outside all segments. Later stable versions may be classified
  `UnverifiedNewer`; they are not qualified by this evidence.
- Keep typed `auth_required` mapping on `session/new` and `session/prompt`.
  Local provider configuration stays host-owned; no credential lease is
  introduced.
- Keep Goose `serve`, `--with-builtin`, `--enable-scheduler`, provider
  management, session load/list/delete/close, ACP tool methods, usage, and
  live-voice methods unmapped.
- Keep one consumer streamable-HTTP declaration as emission-only.
  `client_mcp_servers` remains `No` pending its exact live gate; this research
  does not prove remote MCP honouring. SSE remains unsupported.
- Preserve the older currentness fixture and historical research records,
  including their recorded claim-id discrepancy; production claim id and new
  evidence follow the current `selection.rs` claim without rewriting history.

## Sources and limits

- [Goose v1.50.1 release](https://github.com/aaif-goose/goose/releases/tag/v1.50.1)
- [Goose v1.51.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.51.0)
- [Goose v1.52.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.52.0)
- [Goose v1.53.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.53.0)
- Official GitHub stable tag refs and recursive source trees for all four
  points, frozen under `crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.53.0/`.

No provider prompt, authenticated catalogue, live ACP session, MCP server,
credential, Goose installation, host update, or host permission change was
used. Static source evidence does not replace a live proof.
