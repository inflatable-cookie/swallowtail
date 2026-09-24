# Research 340: Grok Build ACP 1.0.40 And 1.0.41 Identity

Status: complete; identity evidence only. Production claims were first raised
to `1.0.40`, then extended to `1.0.41` as a compatible extension.

Observed 2026-09-21 on the `grok-build.executable` axis, ACP route only:

- Host `grok` is not on `PATH`. Missing install is not a gap and was not
  installed.
- Official npm: `latest` and `alpha` both `1.0.40` (published
  2026-09-20T23:48:33.620Z; packument modified 2026-09-21T01:58:47.345Z).
  Rechecked immediately before the first identity record; no newer stable
  existed then. First unpublished later stable was `1.0.41`.
- Channel rule (same as Research 294/314): npm `dist-tags` expose only
  current pointers. Every plain version at or below `latest` is a published
  stable; an alpha-only version above `latest` is not. `1.0.31` was
  alpha-only at Research 314 and now sits at or below `latest`, so it
  counts as a published stable. Dist-tag history is not recorded by the
  registry.
- Compared platform is this observation host's `linux-x64`
  (`@xai-official/grok-linux-x64`). Darwin-arm64 was cross-checked at
  every hop so Research 314's ceiling identity can reproduce and so the
  prior corpus platform's selected-literal presence can be compared.

Rechecked 2026-09-24, immediately before the second observation:

- Official npm `latest` and `alpha` both moved to `1.0.41` (published
  2026-09-22T16:39:47.725Z; packument modified 2026-09-22T21:33:00.806Z;
  `gitHead` `4220f3b224a672ff2641e35ba78ef6b0c6fd7069`). No later stable
  exists, so there is no first-unpublished successor to name. `1.0.41` is
  the current official stable and was extended as a compatible-extension
  hop.

## Method

Launcher and `linux-x64` platform tarballs were retrieved from the official
npm registry for the previous ceiling `1.0.30`, every published stable
through `1.0.40`, and the `1.0.41` hop. Darwin-arm64 platform tarballs
were retrieved for the same hops as a cross-check. Each tarball's published
`sha1` and `sha512` integrity were verified before use. `bin/grok.br` was
brotli-decompressed into `/tmp` and hashed; the decompressed binaries were
never executed, and each was discarded after probing. The probe extracted a
fixed selected-surface literal set from the binary, carved the embedded
default-model document, and inventoried the embedded ACP source module
paths. The launcher and platform package file inventories were digested
per hop.

No prompt, inference, ACP initialize, session, login, credential use,
catalogue command, install, host update, or downloaded-artifact execution
occurred. The exact `1.0.30` catalogue operation and the registered-tool
courier stay independently bounded and were not exercised.

## Official hop identities

Executable, source revision, and model-document values are SHA-256
prefixes of the compared `linux-x64` artifacts. Full digests, published
timestamps, wrapper/platform integrity, tarball digests, and the
darwin-arm64 cross-check live in the frozen corpora
[`grok-1.0.40`](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.40/identity.json)
and
[`grok-1.0.41`](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.41/identity.json).

| Version | Published | git head | Linux-x64 executable | Model document |
| --- | --- | --- | --- | --- |
| `1.0.30` | 2026-09-11 | `04b7ffed98c6` | `504dd6546ab9` | `9d6924ec760a` |
| `1.0.31` | 2026-09-13 | `b44e0bc0a8e0` | `d37fa3e50c55` | `9d6924ec760a` |
| `1.0.32` | 2026-09-15 | `e21ee47a3bbf` | `519493ba078d` | `9d6924ec760a` |
| `1.0.33` | 2026-09-15 | `ff317a6753d1` | `47d3c69f9301` | `9d6924ec760a` |
| `1.0.34` | 2026-09-16 | `3736acbc8658` | `be5905e107d2` | `9d6924ec760a` |
| `1.0.35` | 2026-09-16 | `d949da3917a8` | `f57df130e95a` | `9d6924ec760a` |
| `1.0.36` | 2026-09-17 | `9ecab0bec61e` | `90e373f48b0f` | `9d6924ec760a` |
| `1.0.37` | 2026-09-18 | `7bb320867c4f` | `5b18c917d4e3` | `9d6924ec760a` |
| `1.0.38` | 2026-09-19 | `41b9d57a3b9a` | `d09092c50f1b` | `9d6924ec760a` |
| `1.0.39` | 2026-09-20 | `0b340e9ac868` | `576cd799f754` | `9d6924ec760a` |
| `1.0.40` | 2026-09-20 | `eb1a2256660d` | `92c997dfd109` | `9d6924ec760a` |
| `1.0.41` | 2026-09-22 | `4220f3b224a6` | `9ce03ed23e16` | `9d6924ec760a` |

Cross-checks: the `1.0.30` wrapper tarball SHA-256
`c57e7106e1f18e9d41677d06836a0abb3a498ae353763dacab3a1da473351628`,
darwin-arm64 executable
`d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb`,
darwin-arm64 tarball
`7e522683b99268fb44b5834909739bdea743fb8ed9fe9fa3186b6be56938b3ed`,
and darwin-arm64 brotli
`13f3c6cd5145c5d1e39a6ab9ab51bc429655d6f7410938b295011a367db17d1e`
reproduce Research 314. The `1.0.30` linux-x64 integrity
`sha512-yM1q8mRZeAUUhEJn0SY256D2ucz+jvmK8tKo/Ivcr+7EUouy/VtS9bRfrhTfj5i6IemYxH9NIoV4wtJjSzh90w==`
matches the frozen Research 314
`other_platform_integrities.grok-linux-x64` row. `xai-org/grok-build`
releases and tags are empty; public git stays discovery only.

## Selected-surface classification

Every mapped ACP method, callback, and vendor channel
(`initialize`, `authenticate`, `session/new`, `session/load`,
`session/prompt`, `session/cancel`, `session/update`, `session/set_model`,
`session/request_permission`, `fs/read_text_file`, `session/close`, and the
`x.ai/session/*` forms), every initialize and session key the adapter
reads, every activity and terminal key, the one-shot permission ids, the
auth literals, the model and effort literals, and the launch invocation
are present in every compared linux-x64 executable. The linux-x64
presence map is byte-for-byte identical across `1.0.30..=1.0.41` (digest
`4cceb3e6fc78…`). `agentVersion` is already absent on linux-x64 at the
`1.0.30` ceiling and stays absent; that is a platform compile difference,
not a hop change. Darwin-arm64 keeps the Research 314 presence digest
`4a548e4dc768…` including `agentVersion` at every hop, including `1.0.41`.
No selected literal appears or disappears on either platform.

The embedded default-model document is byte-identical at every hop
including `1.0.41` (`9d6924ec760a…`, two copies). The default stays
`grok-4.6`, the ids stay `grok-4.6` and `grok-4.5`, the default effort
stays `high`, and the efforts stay `xhigh`, `high`, `medium`, `low`. The
last document delta remains the unread `show_model_fingerprint` removal at
`1.0.11`.

The executable embeds its ACP implementation module paths. All 62 mapped
core modules from Research 314 stay present in every hop. Unmapped
module churn through `1.0.41`: `memory_control`/`memory_forget` added at
`1.0.33`; `memory_carryover` added at `1.0.34`; `memory_status` removed
at `1.0.35` (internal rename of the already-unmapped memory feature set);
`mcp_file_input` added at `1.0.37`; `prompt_origin` added at `1.0.39`;
`subagent_handoff` added at `1.0.41`. No mapped method, key, auth, model,
permission, or stop-reason literal is affected.

The shipped file inventory is unchanged: wrapper 5 files, platform 4
files. Only `package.json` and `bin/grok.br` change per hop, including the
`1.0.40` -> `1.0.41` hop. Wrapper `README.md`, `bin/grok`,
`bin/grok-bootstrap.js`, and `bin/postinstall.js` and platform
`README.md`/`THIRD_PARTY_NOTICES.md` are byte-identical through `1.0.41`.
The launcher and postinstall bootstrap stay outside the ACP wire.

Release notes and the public `xai-org/grok-build` mirror are discovery
only. The public repo has no releases or tags matching these npm
`gitHead` commits, so the published artifact tree and its digests are the
support authority per Contract 029's Artifact Authority Without Public
Source.

## Contract 029 decision

`compatible-extension` on `grok-build.executable`, ACP route only. Keep
the baseline `0.2.114`, claim id `grok-build.acp.executable-window-2`,
posture `AllowUnverified`, behavior revision
`grok-build.acp-v1.cached-token-model-4-6-v3`, the deprecated
`0.2.114..=0.2.117` segments, gaps `0.2.118..=0.2.121` and
`1.0.0..=1.0.3`, the `1.0.4` floor, and the `grok-4.6` model binding.
Qualify `1.0.31` through `1.0.41` and raise the maintained
latest-qualified boundary to `1.0.41`. `1.0.41` is the current official
stable, so no later stable is left `UnverifiedNewer`; the next published
stable (not yet observed) stays permitted `UnverifiedNewer` per the
existing posture.

The exact `1.0.30` `QualifiedOnly` catalogue claim and the registered-tool
courier bounded to the accepted live capsules at `1.0.4` and `1.0.5` do
not move. Extending the ACP executable window must not widen the
registered-tool qualification or reopen the catalogue exact pin.

No provider prompt, live ACP session, credential, catalogue command,
install, host update, or downloaded-artifact execution was used.

## Sources

- npm `@xai-official/grok` packument and `linux-x64` / `darwin-arm64`
  platform packuments, official tarballs and published `sha1`/`sha512`
  integrity
- frozen
  `crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.40/{identity,protocol,dist-inventory}.json`
- frozen
  `crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.41/{identity,protocol,dist-inventory}.json`
- Research 314 darwin-arm64 `1.0.30` identity, reproduced
- public mirror [`xai-org/grok-build`](https://github.com/xai-org/grok-build),
  discovery only (empty releases/tags)
