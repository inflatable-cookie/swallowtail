# Codex exec and app-server 0.165.0 identity

Status: promoted.
Date: 2026-10-28.
Scope: `codex.exec` and `codex.app-server` on `codex.cli`. Research 463.

## Question

Can the existing AllowUnverified ceilings move from `0.164.0` to official
stable `0.165.0` without a new behavior revision?

## Decision

Compatible extension. Raise both ceilings to `0.165.0`. Keep the claim ids,
baselines, older segments, exclusions, and the separate exec and app-server
behavior revisions. The lifecycle claim follows the app-server ceiling on
`codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. No new public
operation.

`thread/delete` now calls `ensure_no_attached_threads` before
`prepare_thread_for_delete`. That rejects only when an experimental attached
child exists. Swallowtail does not send `thread/fork` or `attachToSource`, so
threads it creates have no attached child. The ordinary success path stays:
`prepare_thread_for_delete`, descendants-then-root delete, empty
`ThreadDeleteResponse` plus `ThreadDeleted`.

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
classified every changed feeding file. Handler traces follow selected call
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

- `with_expanded_deny_globs` stays at `protocol/src/permissions.rs:1843`. The only caller remains unselected exec-server bubblewrap. Selected exec has zero exec-server refs.
- `exec/src/lib.rs` sets `attached_to_thread_id: None` only inside `resolve_resume_thread_id`. Selected one-shot exec does not call it. The json_mode-gated bwrap warning stays skipped because selected uses `--json`.
- Linux sandbox source is byte-identical. Shipped `codex-resources/bwrap` is a rebuild: `3a81c19ad10a70436529c58e77898e2a447abe80167f813fc20ec34bd9eafdeb` at `0.164.0` and `755c8d6a9b893905a4aabab058bd6c02c453e5e8653adb84fc08e570607f3f36` at `0.165.0`. Linux `codex-path/rg` stays byte-identical. `.aws` remains in `PROTECTED_METADATA_PATH_NAMES`. The create-monitor stays removed. Selected exec read-only still has no workspace writable roots.

App-server dispatch:

- `message_processor.rs` adds experimental `ThreadMemoryRead`. Selected initialize, config, model/list, thread/start, turn/start, archive, delete, and unarchive arms remain and still call `thread_processor` for archive/delete/unarchive.
- `protocol/common.rs` declares `thread/memory/read`. The adapter does not send it.
- `thread/fork` gains optional `attachToSource`, default false. True skips `ThreadStarted`. `thread/list` gains optional `attachedToThreadId`; omitting it keeps ordinary lists and hides attached threads. Archive, unarchive, and delete handler bodies are not in that diff. The adapter sends none of the new fields.
- `finalize_thread_teardown` stays cancel, remove_thread_state/watch, pending_unloads.remove. The listener skip of in-process `PrivateSessionEvent` is unselected. `PrivateSessionEvent` is `serde(skip)` and is not a JSONL variant.
- `map_network_requirements_to_api` copies `review_domains`. The adapter sends no managed application requirements. Model and turn schemas stay byte-identical.

Unpublished interior patches, including the former synthetic `0.164.1`, stay
inside the inclusive range and are not new exclusions. Kept gaps stay:
`0.82.0`, `0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, `0.151.1`,
`0.152.2`, `0.154.1`, and `0.155.2`. `0.165.1` is the synthetic
`UnverifiedNewer` point.

## Claim shape

- `codex.exec.cli-window-2` maintained `0.156.0..=0.165.0` on `codex.exec.jsonl-v1`
- `codex.app-server.cli-window-2` maintained `0.159.0..=0.165.0` on `codex.app-server.v2.protected-aws-workspace-roots`
- lifecycle maintained `0.140.0..=0.165.0` on the existing hard-delete revision

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.165.0` source tag](https://github.com/openai/codex/tree/rust-v0.165.0)
- [Research 452](./452-codex-exec-app-server-0-164-0-identity.md)
- [Research 410](./410-codex-exec-0-161-0-identity-and-policy-qualification.md)
- [Research 388](./388-codex-app-server-currentness-qualification.md)
