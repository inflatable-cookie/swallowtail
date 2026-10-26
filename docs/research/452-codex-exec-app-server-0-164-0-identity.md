# Codex exec and app-server 0.164.0 identity

Status: promoted.
Date: 2026-10-26.
Scope: `codex.exec` and `codex.app-server` on `codex.cli`. Research 452.

## Question

Can the existing AllowUnverified ceilings move from `0.161.0` to official
stable `0.164.0` without a new behavior revision?

## Decision

Compatible extension. Raise both ceilings to `0.164.0`. Keep the claim ids,
baselines, older segments, exclusions, and the separate exec and app-server
behavior revisions. The lifecycle claim follows the app-server ceiling on
`codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. No new public
operation.

`finalize_thread_teardown` reorders cancel before `pending_unloads.remove` at
`0.163.0`. That is pending idle-unload race hardening. Selected
descendant-before-root persist, empty archive/delete responses, notifications,
and `wait_for_thread_shutdown` join stay.

## Method

Re-probed npm `@openai/codex` `latest` and the GitHub latest non-prerelease
release on 2026-10-26. Both are `0.164.0` / `rust-v0.164.0`. Ignored
`0.165.0-alpha.1`. Walked every published stable after `0.161.0`: `0.162.0`,
`0.162.1`, `0.163.0`, `0.164.0`.

Hashed wrapper tarballs and the native `optionalDependencies` aliases
`@openai/codex@VER-darwin-arm64` and `@openai/codex@VER-linux-x64`. Built
complete per-file SHA-256 inventories. Did not execute any downloaded vendor
binary. `0.161.0` natives reuse Research 370. Compared recursive public-tag
source trees (SHA-256 over sorted path/mode/git-blob) and classified every
changed feeding file. Handler traces follow selected call paths, not request
struct identity. No host `codex` binary was required. No prompt, login, or
live session.

## Identity

| Version | npm published | GitHub tag | Commit | Source files |
| --- | --- | --- | --- | --- |
| `0.161.0` | 2026-10-07T16:04:02.844Z | `rust-v0.161.0` | `979011409de0a60b52f179721948e65531d26144` | 8899 |
| `0.162.0` | 2026-10-08T19:01:18.070Z | `rust-v0.162.0` | `c1382380de69521303b416720a52f42d51af6248` | 9017 |
| `0.162.1` | 2026-10-09T19:50:30.512Z | `rust-v0.162.1` | `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d` | 9018 |
| `0.163.0` | 2026-10-15T21:37:11.083Z | `rust-v0.163.0` | `83086188ae0614bd882818ff1aa23f3194962012` | 9176 |
| `0.164.0` | 2026-10-23T04:52:29.240Z | `rust-v0.164.0` | `6aaac694803004a63c6f393d153c96d22521a6b9` | 9438 |

Native tarball SHA-256:

| Version | darwin-arm64 | linux-x64 | files darwin/linux |
| --- | --- | --- | --- |
| `0.161.0` | `21c99b848b25ed91b92afadabe3751cc33878d2684f3e2d01cc1782e6d0abc7a` | `d50e02c2755be551b576527b2b13c6b6a4af3340149afdf792d5491e5d899978` | 44 / 46 |
| `0.162.0` | `e627d6403ce31bda57d57629a0086a565d5f6b57b154aba92e1c9db2463874eb` | `dbd342e7ad896d53dbb505e926f1b82314b79a9d33e209b0b7616ced6b06c983` | 44 / 46 |
| `0.162.1` | `fe0370027a05237adc0152b482ad8502321f6f9aff1baca3bfa56588923ba9ad` | `7417711678b464eee70664126e39e88bb3c84c2928da0e71f14b56e04ff5c09f` | 44 / 46 |
| `0.163.0` | `1193ff4ce72387a5d6c34820b54a9dc57d007c67a958e8acb4715924facd3c31` | `30efc97738b975940d78b59a9bea115fa34040b732807d54ec35ae6b7528de94` | 43 / 45 |
| `0.164.0` | `9b5a35be1975ceb332eaf274032f375b017ab644ff0165737ead03277fae77cd` | `62659fefd9a8efa21a496402fde3c11c025bbeee735a582968f8ffd14cb5f79c` | 43 / 45 |

`0.163.0` drops bundled `zsh`. Wrapper `bin/codex.js` stays
`61b0194f3bb6534439c8d26a3ed57d0805f84b884588b761795323eeb92fcf70`.
The `0.161.0` tag object `7e21416b38834816c224ea0dfd135c3de94b2f15` matches
Research 410. Full identities, inventories, and hop TSV are in the adapter
fixture.

## Selected protocol

Byte-identical from `0.161.0` through `0.164.0`:

- exec argv definitions (`exec --json --sandbox read-only`), JSONL processor, and exec event types
- `thread/delete` processor
- app-server `SandboxPolicy`, `AskForApproval`, and v1 initialize
- managed application loader and `application.network`

Archive / unarchive / delete handlers:

- `thread/delete` enumerates the spawn subtree, `prepare_thread_for_delete`, descendants-then-root `delete_threads`, empty `ThreadDeleteResponse` plus `ThreadDeleted` notifications. `prepare_thread_for_delete` calls `prepare_thread_for_removal` then `log_db.flush`. `prepare_thread_for_removal` is `remove_thread_for_client`, `wait_for_thread_shutdown`, `finalize_thread_teardown`.
- `thread/archive` lives in `thread_processor.rs`: persist non-ephemeral loaded thread, reverse descendants, `prepare_thread_for_archive` (same removal helper), `archive_threads`, `ThreadArchiveResponse` then `ThreadArchived`.
- `thread/unarchive` calls `unarchive_thread(StoreArchiveThreadParams)` then `ThreadUnarchiveResponse` plus `ThreadUnarchived`.
- `finalize_thread_teardown` at `0.161.0` is pending_unloads.remove, cancel, remove_thread_state/watch. From `0.163.0` it is cancel, remove_thread_state/watch, pending_unloads.remove. Selected join/cancel/persist stays.
- `try_evict_v2_thread` is idle-unload only in `thread_lifecycle.rs` (`0.163.0`+). Explicit archive/delete RPCs do not call it.

Exec deny-glob / sandbox:

- `with_expanded_deny_globs` is defined in `protocol/src/permissions.rs:1843` at `0.163.0`.
- Only caller is `exec-server/src/sandbox_integrity/bubblewrap.rs:119` after linux-sandbox `expand_unreadable_globs_in_environment`.
- `exec/src/lib.rs` has zero exec-server refs. The json_mode-gated bwrap warning at line 1179 is skipped because selected uses `--json`.
- Selected sandbox is `exec --sandbox read-only`. exec-server bubblewrap/mxc stay unselected. `.aws` protection remains.
- Linux helper is `linux-sandbox` bwrap, used by selected exec `--sandbox read-only` and app-server workspace-write. Selected exec read-only has no workspace writable roots; the empty-writable-roots fallback requires `has_full_disk_write_access`, which read-only does not. The `0.164.0` removal of `ProtectedCreateTarget` / `ProtectedCreateMonitor` is unused-path cleanup: `0.163.0` tests already required that list empty because missing `.git` / `.agents` / `.codex` / `.aws` are mount-protected before exec via synthetic empty RO tmpfs. Existing protected metadata stays `--ro-bind`. `.aws` remains in `PROTECTED_METADATA_PATH_NAMES`.

App-server dispatch:

- `message_processor.rs` is the JSON-RPC dispatcher. Selected initialize, config, model/list, thread/start, turn/start, archive, delete, and unarchive arms remain and still call `thread_processor` for archive/delete/unarchive.
- New unmapped arms: `ThreadAttachmentOwnerList` at `0.162.0`; `ThreadReadStateUpdate` and `ThreadItemsRead` at `0.163.0`; `ThreadSample`, `ThreadSampleCancel`, and `FeedbackSave` at `0.164.0`. Adapter does not send them.

Additive and unselected:

- optional `parentTurnId`, `rootTurnId`, and responses metadata on `turn/start`; adapter omits them
- optional `excludedThreadIds` and read-state fields on thread list and read
- optional model and reasoning effort on sub-agent activity
- `imageGeneration` payload reshape, which the adapter already treats as an unknown item
- removal of `namespace_tools` from the unselected provider-capabilities read
- optional Desktop storage requirements; adapter sends no managed application requirements
- listener, idle unload, and v2-child eviction on the unselected idle-unload path

`0.162.0` to `0.162.1` does not change those selected files. Unknown
notifications stay progress snapshots. Unpublished gaps, including `0.155.2`,
stay incompatible. `0.164.1` is the synthetic `UnverifiedNewer` point.

## Claim shape

- `codex.exec.cli-window-2` maintained `0.156.0..=0.164.0` on `codex.exec.jsonl-v1`
- `codex.app-server.cli-window-2` maintained `0.159.0..=0.164.0` on `codex.app-server.v2.protected-aws-workspace-roots`
- lifecycle maintained `0.140.0..=0.164.0` on the existing hard-delete revision

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.164.0` source tag](https://github.com/openai/codex/tree/rust-v0.164.0)
- [Research 410](./410-codex-exec-0-161-0-identity-and-policy-qualification.md)
- [Research 388](./388-codex-app-server-currentness-qualification.md)
- [Research 370](./370-codex-exec-currentness/)
