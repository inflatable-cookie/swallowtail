# Grok Build ACP 1.0.70 Identity

Freezes official npm `latest` `1.0.70` identity for `grok-build.acp` and
stops the ceiling at `1.0.46`.

## Channel and hops

The official `@xai-official/grok` packument was re-read on 2026-10-26.
`latest` and `alpha` are both `1.0.70`. Packument `time.modified` is
`2026-10-24T01:48:06.403Z`. `1.0.70` was published `2026-10-23T17:38:01.317Z`.
The published stable hops after `1.0.46` are `1.0.47` through `1.0.70`,
contiguous, with no gaps. `1.0.71` is absent from the registry. The checkpoint
main tip is `f3f72db8b9abd8e8b4c21fada42a5e3414678c8d`.

The hop ledger in
[the identity fixture](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.70-acp/identity.json)
freezes npm publication time, `gitHead` where present, wrapper and platform
integrity and shasum, tarball SHA-256, compressed payload digest, decompressed
runtime digest and size, and the runtime version banner. Linux x64 and Darwin
arm64 payloads were compared at every hop. `gitHead` is present and matches
the embedded 12-character build id for `1.0.47` through `1.0.49`. npm omits
`gitHead` from `1.0.50` through `1.0.70`. Those hops are bound to the exact
tarballs, compressed and decompressed runtime digests, and the embedded banner.
This does not claim source parity for `1.0.50` through `1.0.70`.

The re-download of `1.0.46` reproduces the Research 385 Linux and Darwin
executable SHA-256 and brotli SHA-256, and the banner `1.0.46 (2765805b9442)`.
Wrapper and Darwin arm64 file inventories, and the Darwin runtime SHA-256,
match the Research 443 frozen inventory on every hop `1.0.46` through `1.0.70`.
Linux x64 is outside that catalogue inventory. The match is hash reproduction
of official artifacts. This ACP identity freeze does not adopt the catalogue
formatter mapping, and it does not move `grok-build.catalogue`.

The complete package-tree ledger is in
[dist-inventory.json](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.70-acp/dist-inventory.json).
Each wrapper contains five files. Each Linux x64 and Darwin arm64 package
contains four. The hop ledger records added, removed, changed, and identical
paths plus `package_file_counts` for every side. No file names were added or
removed. On every hop after `1.0.46`, wrapper `package.json` changes, and
each platform package's `package.json` and `bin/grok.br` change. Platform
README and third-party notices stay byte-identical. Wrapper `README.md`
changes only at `1.0.56`: the macOS row moves from `Apple Silicon (arm64)`
to `arm64, x86_64`. That README then stays byte-identical through `1.0.70`.
The README change is a platform table. It is not an ACP wire change.

## Mapped behavior

[protocol.json](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.70-acp/protocol.json)
compares the selected 92-key literal map with the frozen Research 385 corpus.
Each hop stores the measured Linux and Darwin presence digests, not a
self-authored equality boolean. Linux x64 presence and Darwin arm64 presence
are equal to those frozen maps at every hop `1.0.46` through `1.0.70`. Linux
digest `4cceb3e6fc7893dd2e26e8487b5e7266c4040d2fd189ac38ea4569365a10b311`.
Darwin digest
`4a548e4dc7687941e640dc410146a0ed99dc8b95724871317ffe37a14fe3dfb3`.
`agentVersion` stays present on Darwin and absent on Linux.

A separate exact substring sweep of the decompressed executables, on both
platforms and every hop, found no membership change in the curated public
operation set. Present throughout, and already present at the `1.0.46`
ceiling: `initialize`, `authenticate`, `session/new`, `session/load`,
`session/prompt`, `session/cancel`, `session/update`, `session/set_model`,
`session/set_mode`, `session/set_config_option`, `session/request_permission`,
`session/close`, `session/fork`, `session/resume`, `session/list`,
`session/delete`, `session/rename`, `session/export`, `session/import`,
`session/compact`, `fs/read_text_file`, `fs/write_text_file`,
`terminal/create`, `terminal/output`, `terminal/wait_for_exit`,
`terminal/kill`, `terminal/release`, `x.ai/ask_user_question`,
`x.ai/session/upsert`, `x.ai/session/close`, `x.ai/session/update`,
`_x.ai/session/close`, `_x.ai/session/update`, `--no-auto-update`,
`--always-approve`, `--model`, `allow_once`, `reject_once`, `allow_always`,
`reject_always`, `cached_token`, `mcpServers`, `modelState`,
`currentModelId`, `availableModels`, `loadSession`, and `protocolVersion`.
Darwin also keeps `agentVersion`. `session/request_user_input` stays absent
on both platforms. Launch argv stays `--no-auto-update`, `agent`, `stdio`.

`fs/write_text_file`, `session/set_mode`, the terminal methods,
`--always-approve`, `--model`, `x.ai/ask_user_question`, `allow_always`, and
`reject_always` were already present at `1.0.46` and stay unmapped. The
adapter still maps one-shot `allow_once` / `reject_once` only. No new public
operation has to be mapped, and no selected surface moved.

The embedded `default_models.json` digest stays
`9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a`
(2,323 bytes) on every hop and both platforms. Unique document spans were
counted on `1.0.46`, `1.0.58`, and `1.0.70` for both platforms: two
byte-identical copies each time. Default remains `grok-4.6`. Model ids remain
`grok-4.6` and `grok-4.5`. `grok-4.6` effort values remain `xhigh`, `high`,
`medium`, and `low`, with model `reasoning_effort` `high`. `grok-4.5` still
omits `xhigh`. The bytes are the Research 385 document.

ACP source paths are identical on Linux and Darwin at each hop. The set grows
and shrinks only as follows, all provider-internal:

| Hop | Path change | Count |
| --- | --- | --- |
| `1.0.47` | add `xai-grok-pager` `acp/model_state.rs` | 87 |
| `1.0.51` | add `hook_worker.rs` and `notification_hooks.rs`; remove `extensions.rs` | 88 |
| `1.0.52` | add `workflows_listing.rs`, `permission_refusal.rs`, `skill_reads.rs` | 91 |
| `1.0.54` | add `write_card.rs` | 92 |
| `1.0.55` | add `user_question.rs` | 93 |
| `1.0.60` | add `skills_listing.rs` | 94 |
| `1.0.61` | add `outside_links.rs` and `xai-grok-subagent-resolution` `acp.rs` | 96 |
| `1.0.62` | remove `acp_types.rs` | 95 |
| `1.0.67` | add `compaction_headers.rs` | 96 |

`model_state.rs` sits beside the already-mapped `modelState` keys. The hook
split replaces an already-unmapped module. Workflow, skill, and permission
files do not add a session operation; `session/request_permission` and the
four permission kinds stay present, and `session/request_user_input` stays
absent. `user_question.rs` follows `x.ai/ask_user_question`, which was already
present at `1.0.46`. Write-card, outside-link, and compaction-header paths do
not qualify continuation recovery. The subagent path stays with the existing
unmapped subagent handoff. Removing `acp_types.rs` does not remove a selected
literal. These path edits do not flatten `grok-build.acp` onto the catalogue
claim or any other family.

## Selected control flow

[control-flow.json](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.70-acp/control-flow.json)
is the attributable decode of the selected ACP handlers. Recreate it from
official decompressed runtimes with
`scripts/analyze-grok-acp-selected-control-flow.py`. The script hashes each
file, locates interned concatenated JSON-RPC method tables and serde
element-count strings, then maps Darwin compact-unwind ranges through
`ADRP`/`ADD` literal xrefs and Linux `.eh_frame_hdr` ranges through
RIP-relative `LEA` xrefs. No vendor binary is executed.

The selected Swallowtail mapping is `initialize`, `session/new`,
`session/prompt`, `session/update` / `usage_update`,
`session/request_permission` with `allow_once` / `reject_once`, and
`session/cancel`. Join, drain, and release are Swallowtail process lifecycle:
`begin_close` is `close_stdin` plus `request_stop`. Vendor `session/close`
stays unmapped.

On every hop and both platforms the interned stdio 7-tuple
(`initialize`, `authenticate`, `session/new`, `session/load`,
`session/prompt`, `session/cancel`, `session/set_model`), the client 8-tuple
(`session/request_permission`, `session/update`, `fs/write_text_file`,
`fs/read_text_file`, `terminal/create`, `terminal/output`,
`terminal/wait_for_exit`, `terminal/kill`), and the permission 4-tuple
(`allow_once`, `allow_always`, `reject_once`, `reject_always`) stay
byte-identical concatenations. Serde element counts stay
`InitializeRequest` 4, `InitializeResponse` 5, `NewSessionRequest` 4,
`PromptRequest` / `PromptResponse` / `UsageUpdate` 4,
`RequestPermissionRequest` 4, `RequestPermissionResponse` 2,
`SessionModelState` 3, `AgentCapabilities` 8. Field-name packs for
`protocolVersion`/`clientCapabilities`/`clientInfo`,
`agentCapabilities`/`authMethods`/`agentInfo`,
`loadSession`/`promptCapabilities`/`mcpCapabilities`/`sessionCapabilities`,
and `currentModelId`/`availableModels` stay present. StopReason variants
`end_turn`, `cancelled`, `refusal`, `max_tokens`, and `max_turn_requests`
stay present; the intern packing of those variants moved after `1.0.46` and
is not a membership change.

Hops after `1.0.46` add extra interned packs that insert already-present
unmapped methods (`session/set_config_option`, `session/list`,
`session/resume`, `session/close`) beside the selected stdio table. Those
packs are classified `already-present-unmapped`. The selected 7-tuple itself
is never replaced.

Selected handler xref function bodies match `1.0.46` on none of the 48
post-baseline platform-hops. Linux `session/update` stays 1872 bytes from
`1.0.50` through `1.0.70` with 21 distinct body digests.
`session/request_permission` stays 3648 bytes across those 21 hops with 21
distinct digests. Darwin `1.0.70` merges `session/prompt`, `session/cancel`,
and `session/set_model` into one 78624-byte function. Function counts grow
from 128006 to 230887 on Darwin and from 125134 on Linux `1.0.46`. Those
body changes are the stop: interned tables and serde layouts do not prove
the handlers are equivalent.

Every hop, including the twenty with no ACP module-path delta, has this
per-hop decode on both platforms.

`fs/write_text_file` and the terminal methods remain interned in the client
pack and stay unmapped. They have no ADRP/LEA xref to the interior slice;
the adapter does not send those client callbacks.

## Decision

Stop. Keep baseline `0.2.114`, the deprecated `0.2.114..=0.2.117` segments,
maintained floor `1.0.4`, claim id `grok-build.acp.executable-window-2`,
behavior revision `grok-build.acp-v1.cached-token-model-4-6-v3`, and
`AllowUnverified`. The maintained window stays `1.0.4..=1.0.46`. Official
`1.0.47` through `1.0.70` stay `UnverifiedNewer`. Discovery keeps pinning
embedded build ids through the qualified ceiling only. `1.0.71` stays
unpinned and `UnverifiedNewer`.

`grok-build.catalogue` stays `QualifiedOnly` on `1.0.30..=1.0.70` with its
existing behavior revisions. Registered-tool evidence stays exact to `1.0.4`
and `1.0.5`. HTTP MCP evidence stays at its existing exact pin. The shared
`1.0.70` official point is not a flattening: catalogue qualifies it, ACP
does not.

Next adaptation: a decode that can distinguish LTO re-layout from a selected
handler behaviour change on the frozen binaries, or an owner-authorized live
probe of the selected ACP mapping. This record is the identity freeze for
that task.

The host `grok` command was not on `PATH` and was not installed. Public npm
tarballs were statically inspected. No vendor binary was executed. No
credentials, provider prompts, live sessions, catalogue requests, or host
mutation occurred.
