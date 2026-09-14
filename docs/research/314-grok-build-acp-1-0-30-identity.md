# Research 314: Grok Build ACP 1.0.30 Identity

Status: complete; identity evidence only. Production claim changes land in
the g05.064 claim batch after this record.

Observed 2026-09-14 on the `grok-build.executable` axis, ACP route only:

- Host `grok --no-auto-update --version`: `grok 1.0.30 (04b7ffed98c6)
  [stable]`, a symlink to
  `~/.grok/downloads/grok-1.0.30-macos-aarch64`, SHA-256
  `d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb`,
  141869568 bytes. Brotli-decompressing the official
  `@xai-official/grok-darwin-arm64@1.0.30` package's `bin/grok.br` yields a
  byte-identical executable, so the host observation and the official
  artifact are the same identity. The host was read, never installed,
  updated, or replaced.
- Official npm: `latest` `1.0.30` (published 2026-09-11T23:11:16.162Z),
  `alpha` `1.0.31` (published 2026-09-13T15:01:07.004Z). The alpha candidate
  is not the stable target. Rechecked at the identity boundary; no newer
  stable exists.
- Channel rule (same as Research 294): npm `dist-tags` expose only current
  pointers. The newest non-prerelease version carried by the `alpha` tag is a
  candidate, not a stable; every other published non-prerelease version is a
  published stable whether or not it currently sits on `latest`. Under that
  rule the published stables after the `1.0.5` ceiling are exactly
  `1.0.6..=1.0.30`. Research 294 held `latest` `1.0.13` with stable
  `1.0.14..=1.0.23` outside `latest` and `alpha` `1.0.24`; `1.0.24` now sits
  below `latest` and counts as a published stable. Dist-tag history is not
  recorded by the registry, and this record does not claim which promotion
  path `latest` took.
- The publication cadence is continuous: every stable `1.0.6..=1.0.30` was
  published, none is a synthetic gap. There is no unpublished interior
  stable; the first unpublished stable after `latest` is `1.0.32`.

## Method

Launcher and `darwin-arm64` platform tarballs were retrieved from the
official npm registry for the previous ceiling `1.0.5` and every published
stable through `1.0.30`. Each tarball's published `sha1` and `sha512`
integrity were verified before use. `bin/grok.br` was brotli-decompressed
into `/tmp` and hashed; the decompressed binaries were never executed, and
each was discarded after probing. The probe extracted a fixed selected-surface
literal set from the binary, carved the embedded default-model document, and
inventoried the embedded ACP source module paths. The launcher and platform
package file inventories were digested per hop.

No prompt, inference, ACP initialize, session, login, credential use,
catalogue command, install, host update, or downloaded-artifact execution
occurred. The exact `1.0.25` catalogue operation and the registered-tool
courier stay independently bounded and were not exercised.

## Official hop identities

Executable, source revision, and model-document values are SHA-256 prefixes;
the full digests, published timestamps, wrapper/platform integrity, and
tarball digests are frozen in the
[1.0.30 identity corpus](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.30/identity.json).

| Version | Published | git head | Executable | Model document |
| --- | --- | --- | --- | --- |
| `1.0.5` | 2026-08-16 | `5115b46bc909` | `3dfa7f04fbb5` | `264d0f644b0c` |
| `1.0.6` | 2026-08-18 | `24c70bc7ffdd` | `14f76c7164a4` | `264d0f644b0c` |
| `1.0.7` | 2026-08-19 | `325eae35b09b` | `71dad3b49f16` | `264d0f644b0c` |
| `1.0.8` | 2026-08-20 | `95f4d452703b` | `b16e351ea398` | `264d0f644b0c` |
| `1.0.9` | 2026-08-24 | `d5a34fd40c84` | `6010f2c38bc4` | `264d0f644b0c` |
| `1.0.10` | 2026-08-25 | `5992780042ca` | `c66b8b44b670` | `264d0f644b0c` |
| `1.0.11` | 2026-08-26 | `6870d7b2fdb7` | `7fd4681f61a6` | `9d6924ec760a` |
| `1.0.12` | 2026-08-27 | `ece2b556c271` | `a7d2495b9721` | `9d6924ec760a` |
| `1.0.13` | 2026-08-28 | `5e9a58528b76` | `8669e0fdadce` | `9d6924ec760a` |
| `1.0.14` | 2026-08-31 | `dbb9bc1e773c` | `90ef0f656cdd` | `9d6924ec760a` |
| `1.0.15` | 2026-08-31 | `9df779fea880` | `86ffac7a9547` | `9d6924ec760a` |
| `1.0.16` | 2026-09-01 | `a0239a2688c1` | `8d901060ef72` | `9d6924ec760a` |
| `1.0.17` | 2026-09-01 | `a549186d9d39` | `be8b42718132` | `9d6924ec760a` |
| `1.0.18` | 2026-09-02 | `ea950872ad51` | `99b77202286f` | `9d6924ec760a` |
| `1.0.19` | 2026-09-04 | `6b38df55f6b2` | `d07bcf2a5bab` | `9d6924ec760a` |
| `1.0.20` | 2026-09-04 | `df7ef6ad56ed` | `c68e058c3ac0` | `9d6924ec760a` |
| `1.0.21` | 2026-09-04 | `3aedf38d5cfb` | `daac3e1cc567` | `9d6924ec760a` |
| `1.0.22` | 2026-09-07 | `8f40483ca2a5` | `f17b8369d328` | `9d6924ec760a` |
| `1.0.23` | 2026-09-07 | `7fa0ca2c9e6a` | `a697d8e96e93` | `9d6924ec760a` |
| `1.0.24` | 2026-09-07 | `68e414c661e3` | `4291021c1570` | `9d6924ec760a` |
| `1.0.25` | 2026-09-09 | `f7e67d6988e2` | `9ef4a40ad60c` | `9d6924ec760a` |
| `1.0.26` | 2026-09-10 | `fadad3468632` | `081f1d861e99` | `9d6924ec760a` |
| `1.0.27` | 2026-09-10 | `a538938e5720` | `6e85277b0432` | `9d6924ec760a` |
| `1.0.28` | 2026-09-10 | `cae16d2533b6` | `bfaa983f8dd4` | `9d6924ec760a` |
| `1.0.29` | 2026-09-11 | `4c83f16c3e10` | `2b6b44a2e7e7` | `9d6924ec760a` |
| `1.0.30` | 2026-09-11 | `04b7ffed98c6` | `d53b6e543e48` | `9d6924ec760a` |

Cross-checks: the `1.0.5` platform tarball SHA-256
`94b9c4ec7574ef37daabc2fdc5824c5fd86cc84561f2726635fb233f66655192` and
executable `3dfa7f04fbb5…` reproduce the frozen `1.0.5` corpus; the `1.0.25`
executable `9ef4a40ad60c…` reproduces Research 305; the `1.0.13` revision
`5e9a58528b76` reproduces Research 294's host; the `1.0.17` revision
`a549186d9d39` is the public source tree revision cited by Research 289; and
the `1.0.30` executable equals the installed host. `1.0.4` stays in the
existing corpus and was not re-downloaded.

## Selected-surface classification

Every mapped ACP method, callback, and vendor channel
(`initialize`, `authenticate`, `session/new`, `session/load`,
`session/prompt`, `session/cancel`, `session/update`, `session/set_model`,
`session/request_permission`, `fs/read_text_file`, `session/close`, and the
`x.ai/session/*` forms), every initialize and session key the adapter reads
(`protocolVersion`, `clientCapabilities`, `agentCapabilities`, `authMethods`,
`defaultAuthMethodId`, `mcpCapabilities`, `sessionCapabilities`, `loadSession`,
`promptCapabilities`, `embeddedContext`, `sessionId`,
`mcpServers`, `cwd`, `modelState`, `currentModelId`, `availableModels`,
`reasoning_effort`, `reasoning_efforts`), every activity and terminal key
(`sessionUpdate`, `agent_message_chunk`, `agent_thought_chunk`,
`user_message_chunk`, `tool_call`, `tool_call_update`, `toolCallId`,
`stopReason`, `end_turn`, `cancelled`, `refusal`, `_meta`), the one-shot
permission ids (`allow_once`, `reject_once`), the auth literals
(`cached_token`, `grok.com`, `headless`, `XAI_API_KEY`, `oidc`), the model and
effort literals (`grok-4.6`, `grok-4.5`, `xhigh`), and the launch invocation
(`--no-auto-update`, `stdio`) are present in every compared executable. The
presence map is byte-for-byte identical across `1.0.5..=1.0.30`; no selected
literal appears or disappears. The underscore-prefixed vendor spellings the
runtime prefixes at send time do not occur as literals, which is recorded.

The embedded default-model document changes exactly once, at `1.0.11`: both
entries lose the unread `show_model_fingerprint` key. The default stays
`grok-4.6`, the ids stay `grok-4.6` and `grok-4.5`, the default effort stays
`high`, and the efforts stay `xhigh`, `high`, `medium`, `low`. The removed
key is not one of the fields the adapter reads by exact id, so the delta is
bounded unmapped metadata.

The executable embeds its ACP implementation module paths. 62 of them are
present in every hop, including every module that carries the mapped surface
(`xai-acp-lib` gateway and normalizer, `xai-grok-mcp` ACP transport, the pager
`acp_handler` MCP/permission/session-notification modules, and the shell
`acp_session`, `acp_session_impl/{turn,tool_calls,updates,session_setup,
model_switch,spawn,run_loop,sampler_turn,mcp,mcp_snapshot}`, and
`xai-grok-workspace` ACP filesystem modules). The remaining 23 appear and
disappear: added feature modules (status line, sampling events, rate-limit
waits, background tasks, memory capture/status/dream, prompt offload, turn
task, wait-interrupt, context snapshot, parent message, terminal split) and
internal renames or crate splits (`tasks_cancel` → `cancel`/`turn_task`,
`terminal/acp_terminal` → `xai-grok-shell-terminal`, `acp_conversion`,
`laziness_classifier`, `subagent_lifecycle`). No mapped method, key, auth,
model, permission, or stop-reason literal is affected by that churn.

The shipped file inventory is stable: the platform package always ships
`bin/grok.br`, `package.json`, `README.md`, and `THIRD_PARTY_NOTICES.md`, and
only the native payload and its version-bearing `package.json` change per
hop. The launcher package keeps `bin/grok`, `bin/postinstall.js`,
`package.json`, and `README.md`, adds `bin/grok-bootstrap.js` at `1.0.14`,
and otherwise changes only `package.json` until the `1.0.14` bootstrap
refactor, after which the launcher files are stable. The launcher and
postinstall bootstrap are outside the ACP wire; the prepared route launches
the approved executable directly.

Release notes and the public `xai-org/grok-build` mirror (`main`
`37949780c144` at 2026-09-09) are discovery only. The public mirror does not
contain the npm `gitHead` commits, so the published artifact tree and its
digests are the support authority per Contract 029's Artifact Authority
Without Public Source.

## Contract 029 decision

`compatible-extension` on `grok-build.executable`, ACP route only. Keep the
baseline `0.2.114`, claim id `grok-build.acp.executable-window-2`, posture
`AllowUnverified`, behavior revision
`grok-build.acp-v1.cached-token-model-4-6-v3`, the deprecated
`0.2.114..=0.2.117` segments, gaps `0.2.118..=0.2.121` and
`1.0.0..=1.0.3`, and the `grok-4.6` model binding. Qualify `1.0.6` through
`1.0.30` and raise the maintained latest-qualified boundary to `1.0.30`.
Published alpha `1.0.31` and the first unpublished stable `1.0.32` stay
permitted `UnverifiedNewer`.

The exact `1.0.25` `QualifiedOnly` catalogue claim and the registered-tool
courier bounded to the accepted live capsules at `1.0.4` and `1.0.5` do not
move. Extending the ACP executable window must not widen the registered-tool
qualification, which is keyed to the accepted live evidence rather than to
the executable window.

No provider prompt, live ACP session, credential, catalogue command, install,
host update, or downloaded-artifact execution was used.

## Sources

- host `grok 1.0.30 (04b7ffed98c6) [stable]` (`--no-auto-update --version`)
- npm `@xai-official/grok` packument and `darwin-arm64` platform packument,
  official tarballs and published `sha1`/`sha512` integrity
- frozen
  `crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.30/{identity,protocol,dist-inventory}.json`
- public mirror [`xai-org/grok-build`](https://github.com/xai-org/grok-build)
  (`main` `37949780c144` at 2026-09-09), discovery only
