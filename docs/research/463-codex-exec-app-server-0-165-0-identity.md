# Codex exec and app-server 0.165.0 identity

Status: promoted.
Date: 2026-10-28.
Scope: `codex.exec` and `codex.app-server` on `codex.cli`. Research 463.

## Question

Can the existing AllowUnverified ceilings move from `0.164.0` to official
stable `0.165.0` without a new behavior revision?

## Decision

Stop. Keep the 0.164.0 ceilings and existing claim shapes. Two gates remain:

- Linux shipped bwrap bytes differ without a demonstrated build cause or
  behavior identity. Qualify the shipped helper and selected sandbox path.
- An externally forked or imported parent can have an attached child. The
  ordinary adapter list hides that child, and the new guard refuses deletion
  before effect. Qualify or adapt this lifecycle behavior before raising the
  app-server and lifecycle bounds.

These are planner-owned adaptation follow-ups under Contract 029. No new
public operation or authority is granted.

## Method

Re-probed npm `@openai/codex` `latest` and the GitHub latest non-prerelease
release on 2026-10-28. Both are `0.165.0` / `rust-v0.165.0`. Ignored
`0.166.0-alpha.5` and the alpha, beta, and native dist-tags. One published
stable after `0.164.0`. No npm `0.164.1` or `0.165.1`.

Hashed the wrapper tarball and the native `optionalDependencies` aliases
`@openai/codex@0.165.0-darwin-arm64` and `@openai/codex@0.165.0-linux-x64`.
Built complete per-file SHA-256 inventories. Did not execute any downloaded
vendor binary. `0.164.0` natives reuse Research 452. Compared recursive
public-tag source trees (SHA-256 over sorted path/mode/git-blob) and
classified selected feeding files, including the attached-child guard closure. Handler traces follow selected call
paths, not request struct identity. No host `codex` binary was required. No
prompt, login, or live session.

## Identity

| Version | npm published | GitHub tag | Commit | Source files |
| --- | --- | --- | --- | --- |
| `0.164.0` | 2026-10-23T04:52:29.240Z | `rust-v0.164.0` | `6aaac694803004a63c6f393d153c96d22521a6b9` | 9438 |
| `0.165.0` | 2026-10-27T20:14:25.378Z | `rust-v0.165.0` | `360319d8dee62d87e2d59fc0c21805d79519cfed` | 9507 |

The `0.165.0` annotated tag object
`e374836b2728d9b7aa981c2902649e640a2edaa8` points at that commit. GitHub
published the release at 2026-10-27T20:09:38Z. The previous ceiling tag object
`fe3b29455488c2e189e43ad5526a107900c5e1b7` still matches Research 452.

Native tarball SHA-256:

| Version | darwin-arm64 | linux-x64 | files darwin/linux |
| --- | --- | --- | --- |
| `0.164.0` | `9b5a35be1975ceb332eaf274032f375b017ab644ff0165737ead03277fae77cd` | `62659fefd9a8efa21a496402fde3c11c025bbeee735a582968f8ffd14cb5f79c` | 43 / 45 |
| `0.165.0` | `a991b0abcb6b6d2ae4d0221b55dd74afe8b87847044483ec68d290908a5a0f97` | `f997cd6ef5987126e19cd3c82e847a4a6461270f32f97340c5e99a65fb6f9b8a` | 43 / 45 |

`0.165.0` CLI binaries, hashed and not executed:
darwin-arm64 `a45099154d7fb4a5ed8a56f40966a95deb938e508bab23e6985035c709e9a85a`,
linux-x64 `c06a67fd8ac76508dc78bd99ce20c9f317b8d568beda59485f3f4134df9962d5`.
Wrapper `bin/codex.js` stays
`61b0194f3bb6534439c8d26a3ed57d0805f84b884588b761795323eeb92fcf70`.
The wrapper hop changes only `package.json`. Source inventory digest for
`0.165.0` is
`c39481bcf4c96bb514d1fe54047c78e451a66067d0e833874287f6b07f8a3eb0`
(9507 files). The hop adds 80, removes 11, changes 594, and leaves 8833
identical. Full identities, inventories, and the hop TSV are in the adapter
fixture.

## Selected protocol

Byte-identical from `0.164.0` through `0.165.0`:

- exec argv definitions (`exec --json --sandbox read-only`), JSONL processor, and exec event types
- app-server `SandboxPolicy`, `AskForApproval`, and v1 initialize
- turn, thread data, item, model, and application protocol files
- managed application loader and `application.network`
- `protocol/src/permissions.rs`, including `with_expanded_deny_globs`
- linux-sandbox `bwrap.rs` and `linux_run_main.rs`

`thread/delete`:

- `LocalThreadStore::ensure_no_attached_threads` returns `Ok` when no attached child exists, and `InvalidRequest` when an experimental attached child exists.
- The success path stays `prepare_thread_for_delete` (`prepare_thread_for_removal` then `log_db.flush`), descendants-then-root `delete_threads`, empty `ThreadDeleteResponse` plus `ThreadDeleted`.
- `ThreadDeleteResponse.json` and `ThreadArchiveResponse.json` stay byte-identical. `ThreadStartResponse`, `TurnStartResponse`, and `ThreadUnarchiveResponse` gain only the error enum variant `modelNotFound`. The adapter does not match `CodexErrorInfo`.

Exec deny-glob / sandbox:

- `with_expanded_deny_globs` stays at `protocol/src/permissions.rs:1843`. Callers are exec-server bubblewrap and mxc-sandbox; both are outside selected exec, which has zero exec-server refs.
- `exec/src/lib.rs` sets `attached_to_thread_id: None` only inside `resolve_resume_thread_id`. Selected one-shot exec does not call it. The json_mode-gated bwrap warning stays skipped because selected uses `--json`.
- Linux sandbox source is byte-identical. Shipped `codex-resources/bwrap` has an unproven byte difference: `3a81c19ad10a70436529c58e77898e2a447abe80167f813fc20ec34bd9eafdeb` at `0.164.0` and `755c8d6a9b893905a4aabab058bd6c02c453e5e8653adb84fc08e570607f3f36` at `0.165.0`. Linux `codex-path/rg` stays byte-identical. `.aws` remains in `PROTECTED_METADATA_PATH_NAMES`. The create-monitor stays removed. Selected exec read-only still has no workspace writable roots.

App-server dispatch:

- `message_processor.rs` adds experimental `ThreadMemoryRead`. Selected initialize, config, model/list, thread/start, turn/start, archive, delete, and unarchive arms remain and still call `thread_processor` for archive/delete/unarchive.
- `protocol/common.rs` declares `thread/memory/read`. The adapter does not send it.
- `thread/fork` gains optional `attachToSource`, default false. True skips `ThreadStarted`. `thread/list` gains optional `attachedToThreadId`; omitting it keeps ordinary lists and hides attached threads. Archive, unarchive, and delete handler bodies are not in that diff. The adapter sends none of the new fields, but externally created or imported parents may still have attached children. The guard fails before effect while ordinary adapter listings hide those children.
- `finalize_thread_teardown` stays cancel, remove_thread_state/watch, pending_unloads.remove. The listener skip of in-process `PrivateSessionEvent` is unselected. `PrivateSessionEvent` is `serde(skip)` and is not a JSONL variant.
- `map_network_requirements_to_api` copies `review_domains`. The adapter sends no managed application requirements. Model and turn schemas stay byte-identical.

Unpublished interior patches, including the former synthetic `0.164.1`, stay
inside the inclusive range and are not new exclusions. Kept gaps stay:
`0.82.0`, `0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, `0.151.1`,
`0.152.2`, `0.154.1`, and `0.155.2`. `0.165.0` and `0.165.1` remain `UnverifiedNewer`.

Selected `thread/resume` also reaches the paginated history path for ordinary
non-ephemeral local threads. In `thread_processor.rs`, after config loading
releases the locks, 0.165.0 retains `PreparedResumeConfig.model_context` and
rechecks the thread. `load_resume_initial_history_from_stored_thread` reuses
it only for a paginated stored thread with matching conversation id and rollout
path, a revision, and `is_model_context_current` returning true. A mismatch,
missing revision, false or error reloads the latest model context. Explicit
request history bypasses this stored-history path; non-paginated history reads
the full history. The adapter sends `thread/resume`, so this is selected, not
an unmapped cache. Its effect on restoration and concurrent history remains a
qualification follow-up alongside the delete guard; no equivalence is claimed.

## Claim shape

All three ceilings remain `0.164.0`; `0.165.0` is `UnverifiedNewer`.
The existing baselines, behavior revisions, exclusions, and claim ids remain.
`0.165.1` is also unverified.

The bwrap crate, vendored C, build action, toolchain, release profile and
lock dependencies are identical across tags; that does not explain the shipped
byte change or establish behavior identity. The `rust-release.yml` change is
Python-wheel only. The attached-child guard closure includes
`thread-store/src/local/delete_thread.rs`, `rollout/src/rollout_reference_index.rs`,
`thread-store/src/store.rs`, `thread-store/src/local/list_threads.rs`, and
`state/migrations/0062_threads_attached_parent.sql`. The reference index checks
active and archived rollouts. A guard error maps to `failed_before_effect`.

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.165.0` source tag](https://github.com/openai/codex/tree/rust-v0.165.0)
- [Research 452](./452-codex-exec-app-server-0-164-0-identity.md)
- [Research 410](./410-codex-exec-0-161-0-identity-and-policy-qualification.md)
- [Research 388](./388-codex-app-server-currentness-qualification.md)
