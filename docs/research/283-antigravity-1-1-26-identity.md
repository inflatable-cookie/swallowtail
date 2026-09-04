# 283 Antigravity 1.1.26 Identity

Status: promoted
Owner: Tom
Date: 2026-09-04
Card: g05 batch 071

## Question

Is official GitHub `google-antigravity/antigravity-cli` `1.1.26` a
compatible extension of the catalogue, print, and exact-conversation surfaces
qualified through `1.1.17`, a private milestone, a new revision, or a stop?

## Remaining AllowUnverified rank

Named family only.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity catalogue and headless | installed official `1.1.19` | `1.1.9..=1.1.17` | operator-dispatched family; official GitHub stable is `1.1.26` |

Gemini stays deferred. `antigravity-acp` is another family. `1.1.8` stays
independently incompatible. Card 069's projection code is unrelated.

## Method

Re-probed the official GitHub releases and tags on 2026-09-04. Downloaded the
linux-x64 tarball for every point `1.1.17..=1.1.26` into a fresh `/tmp`
directory, hashed each archive, extracted it, and hashed and inspected the
single ELF binary with `file`, `strings`, and its GNU build-id note. No
downloaded binary was executed.

Scanned each binary for the exact selected flag, value, catalogue, and version
literals. Compared every shipped tree and each adjacent public tag. Every
archive contains only `antigravity`; that binary changes at every hop. Every
public tag hop changes only `CHANGELOG.md`, so the closed binary is support
authority and the changelog is discovery only.

Re-downloaded the official `1.1.16` linux-x64 and `1.1.17` linux-x64 and
mac-arm64 artifacts to recompute every digest in the frozen `1.1.17` corpus.
Recomputed every digest parked on PR 182 from fresh `1.1.17..=1.1.24`
downloads, including the `1.1.24` mac-arm64 tarball. The parked branch was not
merged, cherry-picked, rebased, or used as digest authority.

The host `agy` was not executed. Its 178046224-byte Mach-O is byte-identical
to the official signed `1.1.19` mac-arm64 binary. No install, update, provider
prompt, catalogue call, print run, credential use, or host mutation occurred.

## Identity

Host `agy` is official `1.1.19`: binary SHA-256
`96fae3fccfb444c7fb2c6d8d70426e5c978e4f21cfc4507a541f612a8b8ffeef`,
official mac-arm64 tarball SHA-256
`ad72daf6b255d96e4864fe6bd2f3fa4070fb4c554845ceaf1d6399d8f1092e45`,
tag commit `ee5766c17fce8f27ea85185f97183575058218ec`, and Developer ID
`Google LLC (EQHXZ8M8AV)`.

| Version | Published | Tag commit | linux-x64 tarball SHA-256 / size | Extracted binary SHA-256 / size / Build ID |
| --- | --- | --- | --- | --- |
| `1.1.17` | 2026-08-20T22:13:58Z | `efa16f096dc02fb654b7e86958d268195284d014` | `15443966494cd62938320900acfd16df906cf4da56279e4dd8f4846c09f849df` / 55607296 | `d1ea7370fce2ae229a370d8cc42e91d4eeb971344c5f07918e55ce05a4e19579` / 205574400 / `de253ec6ade81ddac11b45a8558af46a` |
| `1.1.18` | 2026-08-22T01:46:57Z | `f09d6b583d0f902d3f0f63736af23d34f0a5ddbe` | `1aa7e3c1f5ba02372d24ba2f99ed015c7135016becc7dcbb18bf8332f513a818` / 55721724 | `60eb243a68bfbc1bffa3823c7fb90df27a72502550b333c6248fc55f20d02564` / 206024960 / `dd44555625e546bec8bd0befba77123a` |
| `1.1.19` | 2026-08-22T23:30:26Z | `ee5766c17fce8f27ea85185f97183575058218ec` | `a02132a7c6c647ef0ad483ecbe767619adf6b660a5589cba5c937b0c83909b97` / 55763391 | `68d229d37aeabde76d15af0003d4c1ce07b211414e7452fb0309be9714ae7dd4` / 206188800 / `708bc379032a11dfc939a59a9b461991` |
| `1.1.20` | 2026-08-25T02:58:27Z | `ade702a5439c2bc67de2f9cfcb83c5370768f0c6` | `6ceeb0ac91df6dca60a4fa02856807ed2e2fc6d3d70bb734d1ad61a9e44ef4da` / 56295623 | `d743ebe97c822b07d010a5a836804528119f926de9b136c9f5b2c0925fe710cb` / 208085248 / `917b0a8cc7c33aec77e9e26374c01659` |
| `1.1.21` | 2026-08-26T02:21:06Z | `7cc1925c8cbe021699038606ada488618dbda5a2` | `4806a347119d36be6d8ab5cc3f03319bc6aa8407a8d9203de7976a42954cabde` / 56329878 | `ca7ffc496be6c24bb908aab478ec5be2b8fbad76507085b885163475613332c5` / 208183552 / `54865ceacda7fc2f3ff8071b1b8180fb` |
| `1.1.22` | 2026-08-27T04:03:21Z | `556846a4bb94117222f53846896c7eb0d645307e` | `1e1a219a86e75d7c6351f96d182ca2105302d5c34d8fa9c31265dc0adf24145f` / 56399106 | `2822292f90deea4556938a8728fe4ed02a1d66d1525cf75fa07a171e36a38c25` / 208429312 / `a9f978445e9528435a7fcaa6983687aa` |
| `1.1.23` | 2026-09-01T04:47:50Z | `4c150a22f7f68061e8af35412b05b9f8974e4c56` | `379693509ca4d68d74f75def6c95996739aa6c1dc38b120c399035c108f1a39a` / 56593215 | `caf4a5f9ae0f02e0ac3db01600a7dd4a9697354e3f4dc3f0a08b2de30d3aefbc` / 208986368 / `7d25d7790a3dc495ec30cb513b299199` |
| `1.1.24` | 2026-09-02T02:38:18Z | `bf27ce1134b4ead2f7bfa0a4fb3cb5fcbebcaa5a` | `cff1fb7ed735da72c35658645a4f916cf74f020d4cd30ab95ebe8c2a49a4d569` / 56692103 | `22c6ddeb06d2da6049ff861e44954bf232b77bd791986104326e9500f5327193` / 209273088 / `0d87e8b60bfaf0d76a8d5e6f838dddae` |
| `1.1.25` | 2026-09-03T02:30:18Z | `7e1316ca775dc3805aac13b2db5cd37d89d5aae8` | `45ab4a99884de17af76565a4ff8d9762d6e960067bd008fde9b050ec8fc9e421` / 56770237 | `e552463e7cd479e342cfec3487f7b2de048b89548df74c610e3a58d1c2c9735b` / 210436352 / `64216c04e5d62b5257e3e40bc500defd` |
| `1.1.26` | 2026-09-04T03:28:48Z | `3bc5795ff561c9d71bf1ce272f185aec6013e5e4` | `c47c0726266b3513660b7094bceceecbd03d8ae907786aa269c507ceb7e4ee54` / 56691683 | `a0a6a8044d01accd39e6f5926d29648d212a2e519ff14102f09e1c061e6171dd` / 210247936 / `ffbd3e994b91095d2d1ff46e3b54b6c4` |

Every binary contains its own version literal. Published stables after the
previous ceiling are `1.1.18` through `1.1.26`. GitHub has no stable release
or tag `1.1.27`; that is the first unpublished later point.

### Frozen and parked cross-checks

The frozen `1.1.17` corpus matches fresh downloads:

- `1.1.16` linux-x64 tarball `7742953b7835b457e9102f1357a493913657dfd147435584f609d58356ec085a`
- `1.1.16` extracted binary `b233e6a4f38564a06a0d3220aa79f6a7c8f11da2b85fc8f0957f8a14d46e6cc9`
- `1.1.17` linux-x64 tarball and binary as above
- `1.1.17` mac-arm64 tarball `60fe89d3aef472ddf6c7048032f7585fae732d879f3700fc3188c68c46b35cdd`

PR 182 head `562225db6e2a77986e5f1504a70f767ccb3fe82d` recorded the same
freshly recomputed linux-x64 tarball and binary hashes for every point
`1.1.17..=1.1.24`. Its recorded `1.1.24` mac-arm64 tarball hash also matches:
`189af288ed9527f567ab3a53b35a6da2fc0c3812c6245f266c75a2a3604bdec3`.
No digest disagrees.

## Selected protocol and hop ledger

Every binary retains `--print`, `--output-format`, `--model`, `--mode`,
`--sandbox`, `--effort`, `--json-schema`, exact `--conversation`, `models`,
`stream-json`, `plan`, and `request-review` literals. The selected literal set
is stable. Semantic lifecycle is not.

| Hop | Mapped or authority-relevant change | Material unmapped change | Result |
| --- | --- | --- | --- |
| `1.1.17→1.1.18` | dropped print streams now fail non-zero; malformed valueless print arguments reject | project-name widening, picker rename/delete keys, audio formats | compatible repair |
| `1.1.18→1.1.19` | none | Remote Control free-port choice; renderer environment controls | unmapped |
| `1.1.19→1.1.20` | review mode auto-approves reads inside the workspace; benign tool errors and permission denials no longer become cascade failure; embedded ripgrep removes an ambient child dependency | skill icons, customization listing, settings preservation | compatible repairs within selected Read/ReadWrite authority |
| `1.1.20→1.1.21` | invalid-UTF-8 tool results no longer stall the stream; read/write repairs stay under selected resource authority | voice and `mic-serve`; always-proceed MCP/page autoapproval; cost line | compatible repair; always-proceed is unselected |
| `1.1.21→1.1.22` | model-endpoint HTTP 502 now retries instead of ending the run; self-subagents preserve parent setup | interactive `/model`; Gemini API-key effort; headless daemon banner | **authority stop**: selected provider-managed retry has no published bound, disable control, or separate acceptance |
| `1.1.22→1.1.23` | `models` no longer hangs on inherited stdin; history keeps tool-call IDs; cancelled subagent status repairs | MCP dispatcher and Google Cloud onboarding | compatible repairs after the blocking hop |
| `1.1.23→1.1.24` | piped headless stdio closes with `FD_CLOEXEC`; startup schema URIs survive inaccessible cwd | MCP JSON5, conversation annotation cleanup, interactive goal side-questions | compatible repairs after the blocking hop |
| `1.1.24→1.1.25` | permission grants no longer duplicate across reload/subagents; post-trajectory summary panic fixed | Gemini API-key catalogue row; Markdown custom-agent ambient inheritance; MCP OAuth; Remote Control | compatible repairs after the blocking hop; custom `--agent` is unselected |
| `1.1.25→1.1.26` | exit checkpoints the SQLite WAL for trailing continuation metadata | interactive effort default; always-proceed subagent prompt fix; kill/delete worktree cleanup; logout and discovery extras | compatible repair after the blocking hop |

`--input-format`, `mcp`, `mic-serve`, voice, Remote Control, Gemini API-key
sign-in, `--agent`, ambient `--continue`, and dangerous permission bypass stay
unmapped. The adapter selects neither always-proceed nor interactive picker,
kill, delete, logout, or custom-agent operations.

## HTTP 502 retry authority

The `1.1.22` changelog says a model-endpoint HTTP 502 no longer ends the run
and is instead retried. That applies to the selected print and continuation
model request. The public repository has no executable source. Scans of
`1.1.21`, `1.1.22`, and `1.1.26` find no public `--retry`,
`--disable-retry`, `--max-retries`, `--retry-count`, `--retry-backoff`, or
`--max-attempts` control. The changelog publishes no maximum or backoff.

Contract 023 keeps provider-native retry separate from Swallowtail's monotonic
deadline, cancellation, process stop, and joined cleanup. It says
provider-managed retry stays disabled unless separately accepted. No such
acceptance exists for this lane, and the host deadline cannot substitute for
the missing retry policy. The closed binary does not expose enough evidence to
name a deterministic private milestone.

Other authority changes do not rescue or add another stop:

- `1.1.20` workspace read autoapproval stays inside the adapter-selected
  `ResourceAccess::Read`. Contracts 017 and 023 still treat provider approval
  as distinct from filesystem or descendant containment.
- `1.1.25` changes ambient inheritance only for Markdown custom agents. The
  adapter never passes `--agent`; default agents already had that posture.
- `1.1.26` cleans worktrees for provider kill/delete operations that
  Swallowtail does not select. It grants no Swallowtail delete authority and
  proves no descendant containment.
- exact `--conversation` remains private continuation state. Contract 017 does
  not turn it into public load, resume, enumerate, or delete authority.

## Decision

**Stop.** Keep baseline `1.1.9`, latest-qualified `1.1.17`,
`AllowUnverified`, claim ids, and both existing behavior revisions. Do not
admit a segment for card 072. Do not partially qualify `1.1.18..=1.1.21` from
a card targeting the current official stable.

Reopen when official evidence names a finite retry policy plus a deterministic
disable or bound, or the operator separately accepts the exact provider retry
behavior under Contract 023. Decoder specimens stay on
`antigravity-cli-1.1.9`. Production claims are unchanged.

## Sources

- [GitHub `1.1.26`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.26)
- [Changelog at `1.1.26`](https://github.com/google-antigravity/antigravity-cli/blob/1.1.26/CHANGELOG.md)
- official `agy_cli_linux_x64.tar.gz` for `1.1.16..=1.1.26`
- official `agy_cli_mac_arm64.tar.gz` for `1.1.17`, `1.1.19`, and `1.1.24`
- parked PR 182 head `562225db6e2a77986e5f1504a70f767ccb3fe82d`
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.17/`
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
  and [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Research 177](./177-antigravity-1-1-17-identity.md) and
  [Research 276](./276-all-route-version-currentness-checkpoint.md)
