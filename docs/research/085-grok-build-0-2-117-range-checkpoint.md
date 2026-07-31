# 085 Grok Build 0.2.117 Range Checkpoint

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Does installed and current stable Grok Build `0.2.117` require a new
Swallowtail contract, route, or compatibility segment beyond qualified exact
`0.2.114`?

## Method

The checkpoint compared official npm metadata, signed macOS platform
artifacts, exact executable version output, selected ACP initialization output,
official changelogs, and current xAI CLI documentation.

It inspected stable `0.2.115`, `0.2.116`, and `0.2.117`. Each exact platform
artifact was downloaded to a temporary directory, checked with strict macOS
code-signature validation, and executed only with `--no-auto-update --version`
or one ACP `initialize` request. The installed `0.2.117` executable received
the same bounded probes.

No authentication, session creation, model prompt, provider request, workspace
write, or durable provider mutation ran.

## Exact Artifact Evidence

| Version | Executable source revision | macOS arm64 tar SHA-256 | Executable SHA-256 |
| --- | --- | --- | --- |
| `0.2.115` | `dd16b5eb7d50` | `11987bffe9980199877e67b9714f37fd360656d0ae0dc2222cea67d24da1a294` | `32e6a2a7beabd480a299db9955f860bd47e46d40ed0c06552fab7347615da4ca` |
| `0.2.116` | `99b387d2cc0e` | `64cb8ef863452a6be8af1ef5ba3a7864288930ff6e16bf7faa14c442d42c0d7e` | `7b54fde201cfe7c753bd6f6f8e9ee4bb7d4d9b5a04568c26a80a622d69f9d463` |
| `0.2.117` | `f1c06093089f` | `c187041aa6d584a50916f268c6b6705928b7a0542e207a19812014578e303726` | `03de738b8ccd40569a18905ddafe004226eff3343b8423a172477b2767e721d2` |

The installed executable resolves to `0.2.117 (f1c06093089f) [stable]` and its
SHA-256 matches the official platform package. All three platform binaries are
signed by X.AI Corporation under team id `5Y6N3AJ54S`.

The fixture must also freeze the separate launcher and platform npm SHA-512
integrity values. Package identity and selected executable identity are not one
field.

## ACP Initialization Evidence

Every exact release returned the same selected ACP initialization facts:

- protocol version `1`
- exact agent version matching the executable
- retained-session load support
- `cached_token` and `grok.com` authentication methods
- `cached_token` as the default method
- model `grok-4.5`
- `high`, `medium`, and `low` effort choices

No stderr was emitted. This is compatibility evidence for Swallowtail's
selected ACP route, not a claim that the releases are source-identical.

## Material Release Delta

`0.2.115` adds session deletion UI and history/auth robustness. `0.2.116`
expands headless streaming with tool, result, and usage records; Swallowtail's
selected route is ACP stdio, so that headless-only change does not widen this
route.

`0.2.117` changes ACP-visible task behavior:

- stopping a run terminates background subagents from the prior loop
- `kill_task` reports a missing task correctly over ACP
- `get_task_output` no longer waits indefinitely over ACP
- plan approval handling is repaired

Swallowtail does not gain a direct task-control operation from those changes.
Provider-emitted tool and child activity stays observational, while existing
cancellation and interruption retain their current authority.

## Decision

Compile roadmap g03.012 for one continuous maintained window from `0.2.114`
through `0.2.117` with two private behavior segments:

- `0.2.114..=0.2.116` —
  `grok-build.acp-v1.cached-token-activation-v1`
- exact `0.2.117` —
  `grok-build.acp-v1.cached-token-task-control-v2`

Versions above `0.2.117` remain visible as unverified newer and inherit the
latest mapped behavior revision for attempted compatibility. Prereleases,
malformed observations, and pre-baseline versions remain incompatible.

## Contract Result

Contracts 011, 015, 023, 029, 032, 037, 039, and 044 already govern the work.
No new operation, capability, credential authority, session authority,
topology, fallback, or public task-control surface is required.

Standalone Claude and Gemini maintenance remains paused. No other installed
harness had a material qualified-route drift during this checkpoint.

## Sources

- [xAI CLI reference](https://docs.x.ai/build/cli/reference)
- [xAI headless and ACP documentation](https://docs.x.ai/build/cli/headless-scripting)
- [xAI Grok Build repository](https://github.com/xai-org/grok-build)
- [Grok Build `0.2.115` changelog](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-shell/changelogs/0.2.115.md)
- [Grok Build `0.2.116` changelog](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-shell/changelogs/0.2.116.md)
- [Grok Build `0.2.117` changelog](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-shell/changelogs/0.2.117.md)
- npm metadata for `@xai-official/grok` and
  `@xai-official/grok-darwin-arm64` at `0.2.115`, `0.2.116`, and `0.2.117`
