# Codex App-Server 0.171.0 Identity and Currentness Qualification

Status: promoted.
Date: 2026-11-23.
Scope: `codex.app-server` and `codex.app-server` lifecycle on `codex.cli` (Contract 029 one-family currentness). Research 488.

## Question

Can `codex.app-server` and its thread-lifecycle claim extend their maintained
`AllowUnverified` ceilings from `0.164.0` through official stable `0.171.0`
(`0.165.0`–`0.171.0`, ten published stable hops), closing the Research 463/467
Linux `bwrap` provenance and attached-child `thread/delete` / paginated-resume
gates and classifying every selected RPC, lifecycle, permission, sandbox, and
managed-configuration change across the ten hops?

## Decision

- **`codex.app-server.cli-window-2` (compatible extension):** Raise
  `CODEX_APP_SERVER_LATEST_QUALIFIED_VERSION` from `0.164.0` to `0.171.0`,
  extending maintained `0.159.0..=0.164.0` to `0.159.0..=0.171.0` on
  `codex.app-server.v2.protected-aws-workspace-roots` (`Maintained`,
  `AllowUnverified`). Preserve claim id, baseline `0.80.0`, older segments,
  thread-catalogue floor `0.105.0`, and all retained exclusions (`0.82.0`,
  `0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, `0.151.1`, `0.152.2`,
  `0.154.1`, `0.155.2`). `0.171.1` is the synthetic `UnverifiedNewer` point.
- **`codex.app-server.lifecycle-window-1` (private milestone):** Keep
  `0.140.0..=0.165.0` on
  `codex.app-server.lifecycle.v1.strict-descendant-hard-delete` (`Maintained`)
  and add `0.166.0..=0.171.0` on
  `codex.app-server.lifecycle.v1.best-effort-descendant-restore` (`Maintained`,
  `AllowUnverified`) to record `0.166.0`'s shift from target-only
  `unarchive_thread` to best-effort spawned-descendant `unarchive_threads`
  (mirroring Research 037's `0.123.0` `best-effort-descendant-archive`). No
  public operation, capability, affected scope, or diagnostic code changes:
  `Restore` remains `TargetOnly`, `Delete` remains `ProviderHardDeleted` with
  `ProviderDefinedDescendants`.

## Method

Re-probed npm `@openai/codex` `latest` and GitHub `openai/codex` non-prerelease
releases on 2026-11-23. Both agree on `0.171.0` / `rust-v0.171.0`. Reused the
verified eleven-version (`0.164.0`–`0.171.0`) wrapper/native tarball, local
host binary (`codex-cli 0.171.0`, Darwin ARM64), and Sigstore Public Good TUF
`bwrap` provenance ledger from [Research 487](./487-codex-exec-0-171-0-identity.md)
(`crates/swallowtail-adapter-codex/tests/fixtures/codex-exec-0.171.0/`).

Fetched all eleven official tags (`rust-v0.164.0`–`rust-v0.171.0`) into run
scratch, verified every tag commit against Research 487, computed per-version
SHA-256 digests and per-hop changed-file lists for all 37 selected
`codex.app-server` feeding files across `app-server`, `app-server-protocol`,
`thread-store`, `rollout`, `state`, `protocol/permissions`, `linux-sandbox`,
and `sandboxing`, and classified every delta in
`crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.171.0/`.
No downloaded vendor binary was executed; no prompt, login, live session, or
host mutation occurred.

## Gate Resolution Across `0.165.0`–`0.171.0`

1. **Linux `bwrap` provenance (Research 463/467):** Closed across all eleven
   releases (`0.164.0`–`0.171.0`) via Research 487's trusted Sigstore TUF
   verification (Fulcio CA chain at Rekor `integratedTime`, trusted Rekor SET
   key, OIDC workflow/commit extensions), embedded `CODEX_BWRAP_SHA256`, and
   byte-identical `bwrap`/vendored C sources. `PROTECTED_METADATA_PATH_NAMES`
   remains `[".git", ".agents", ".codex", ".aws"]` through `0.171.0` with no
   new workspace-write narrowing.
2. **Attached-child `thread/delete` and hidden-child listing (Research 467):**
   - `codex-rs/app-server/src/request_processors/thread_delete.rs`,
     `codex-rs/rollout/src/rollout_reference_index.rs`, and
     `codex-rs/state/migrations/0062_threads_attached_parent.sql` are
     **byte-identical** from `0.165.0` through `0.171.0`.
   - `codex-rs/thread-store/src/local/delete_thread.rs` adds only
     `ensure_diagnostic_logs_available`, which fails closed before destructive
     cleanup if the diagnostic logs DB reports an error.
   - `codex-rs/state/src/runtime/threads.rs` retains `EXCLUDE_ATTACHED_THREADS`
     for all ordinary listings (`relation_filter` is `None`). Swallowtail never
     sends `attachToSource`, `attachedToThreadId`, or `includeAttachedThreads`.
     Refusal continues to map to `failed_before_effect` with
     `swallowtail.codex.app_server.request_failed`, and the successful path
     deletes the strict spawn-edge subtree (`ProviderDefinedDescendants`).
3. **Best-effort descendant restore (`0.166.0` milestone):**
   - Starting in `0.166.0`, `thread_processor.rs` (`thread_unarchive_response`)
     and `codex-rs/thread-store/src/local/unarchive_thread.rs` switch from
     target-only `unarchive_thread` to `unarchive_threads`, attempting to
     restore spawned descendants best-effort (parent restore stays strict;
     descendant failures log warnings and do not fail parent restore) and
     emitting one `thread/unarchived` notification per restored thread id.
   - Following Research 037's precedent for `0.123.0`
     `best-effort-descendant-archive`, the portable guarantee for `Restore`
     stays `ProviderSessionAffectedScope::TargetOnly` while the provider side
     effect is recorded under a distinct maintained private behavior revision:
     `codex.app-server.lifecycle.v1.best-effort-descendant-restore` for
     `0.166.0..=0.171.0`.
4. **Paginated `thread/resume` and CLI account check:**
   - `codex-rs/thread-store/src/local/history_revision.rs` is **byte-identical**
     across all ten hops (`0.164.0`–`0.171.0`), and `live_writer.rs` preserves
     writer-locked `history_revision` revalidation.
   - `thread_resume_account::check` (added later in the window) gates only
     `clientInfo.name` in `["codex-tui", "codex_exec"]`; Swallowtail identifies
     itself as `"swallowtail"` and is unaffected.
5. **Managed configuration (`config_server`) and first-turn provider check:**
   - `startup_config.rs`, `in_process_bootstrap.rs`, `application_network.rs`,
     and `config_manager.rs` adopt administrator-enrolled `[config_server]`
     from host/MDM `requirements.toml` only, with non-interactive OAuth and
     fail-closed errors.
   - In `0.166.0`–`0.167.2`, `turn_processor.rs` checks effective-config model
     provider parity (`check_thread_model_provider_for_input`) on `turn/start`
     (reverted to managed-only `check_thread_model_provider` in `0.168.0`). Any
     mismatch returns a JSON-RPC error before turn work, which
     `CodexSessionHandle::start_turn` maps cleanly to
     `swallowtail.codex.app_server.request_failed` with active-turn and
     registered-tool cleanup.

## Selected `codex.app-server` Protocol Classification

Selected invocation: `codex app-server --listen stdio://`.
Selected RPCs: `initialize`, `initialized`, `model/list`, `thread/start`,
`thread/resume`, `thread/list`, `thread/read`, `thread/archive`,
`thread/unarchive`, `thread/delete`, `turn/start`, `turn/interrupt`, plus
server requests `item/tool/call`, `item/*/requestApproval`,
`item/tool/requestUserInput`, and selected turn/item/hook/lifecycle
notifications.

- `v1.rs`, `v2/model.rs`, `v2/thread.rs`, `v2/thread_data.rs`, `v2/turn.rs`,
  `v2/item.rs`, `v2/shared.rs`, `v2/hook.rs`: all changes are additive optional
  fields, unmapped experimental capabilities (`requiredHumanApproval`,
  `attachToSource`, `includeAttachedThreads`, `currentAccountOnly`,
  `environments`, `deferOutput`), or flat wire wrappers (`ThreadItem` flattens
  `ThreadItemPayload` with optional `startedAtMs`/`completedAtMs`). All mapped
  `ThreadItem` type tags, `TurnStatus` values, `ThreadStatus` values,
  `ModelListParams`/`Model` fields, and `SandboxPolicy` fields stay unchanged.
- `outgoing_message.rs` and `thread_lifecycle.rs`: `resolve_server_request`
  continues to emit `serverRequest/resolved` with `{ threadId, requestId }`.

## Claim Shape

- `codex.app-server.cli-window-2`: maintained `0.159.0..=0.171.0` on
  `codex.app-server.v2.protected-aws-workspace-roots`, `AllowUnverified`;
  `0.171.1` is `UnverifiedNewer`.
- `codex.app-server.lifecycle-window-1`: maintained `0.140.0..=0.165.0` on
  `codex.app-server.lifecycle.v1.strict-descendant-hard-delete` and maintained
  `0.166.0..=0.171.0` on
  `codex.app-server.lifecycle.v1.best-effort-descendant-restore`,
  `AllowUnverified`; `0.171.1` is `UnverifiedNewer`.

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.171.0` source tag](https://github.com/openai/codex/tree/rust-v0.171.0)
- [Research 487](./487-codex-exec-0-171-0-identity.md)
- [Research 467](./467-codex-attached-child-lifecycle-and-paginated-resume.md)
- [Research 463](./463-codex-exec-app-server-0-165-0-identity.md)
- [Research 452](./452-codex-exec-app-server-0-164-0-identity.md)
- [Research 388](./388-codex-app-server-currentness-qualification.md)
- [Research 037](./037-codex-app-server-lifecycle-range-evidence.md)
