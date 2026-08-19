# 160 Codex 0.148.0 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 308

## Question

Is official stable Codex CLI `0.148.0` a compatible extension of the current
`0.147.0` claims, a new adapter-private behavior milestone, or a stop?

## Method

Compared npm `@openai/codex@0.148.0`, host `codex --version` (`0.147.0`),
GitHub tag `rust-v0.148.0`, generated app-server JSON Schema from the
official extracted darwin-arm64 binary, the frozen `0.147.0` compatibility
corpus, and Research 159.

No provider prompt, install, update, live session, or claim edit in the
identity card.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `0.148.0` (published 2026-08-18T22:30:14.842Z) |
| npm integrity | `sha512-bh5kH9+BMrFaHGmLeoSansPdfRksvr4UXzjQInns/KRO7r8VJ+6AAW+SqUsE8XcG3+OW/mI4EEy8Gpo9UDXGvQ==` |
| npm shasum | `069f15c77cf3b26c62c129bc6ca1ff269a226c09` |
| wrapper tarball SHA-256 | `74be73c8a50cc01bce978cc6fec9274564f529a50e9f256b3608a7012437cebe` |
| darwin-arm64 tarball SHA-256 | `b20357817b1b11dab7a6696b051965aaad4311a690f2bac7d87678a92aa8bead` |
| git tag | `rust-v0.148.0` (annotated; peels to `3ba0f711642a888aec92a611a3f3b2211157ff89`) |
| GitHub release | 2026-08-18T22:26:03Z |
| host CLI | `codex-cli 0.147.0` |
| host binary SHA-256 / size | `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37` / 219997536 |
| official extracted CLI | `codex-cli 0.148.0` |
| official binary SHA-256 / size | `b0308517b20543012fa2171aa3d46ce455a7456c4eb2a552ab9468ba4eeb1e50` / 214716336 |
| published from previous ceiling | `0.148.0` only; `0.147.1` unpublished |
| later unpublished | `0.148.1` |
| alpha ignored | `0.149.0-alpha.1` |

Generated schema: `codex app-server generate-json-schema --out` exit 0
from the official extracted `0.148.0` binary.
`codex_app_server_protocol.v2.schemas.json` SHA-256
`e5a20eb7211c21540a2d4e0106479285e13778e9c53d5837cfc735a71316a51e`.
That digest differs from the frozen `0.147.0` stable bundle
`f3dec1e031d99a420b137b903f02196d4325eece57620c925bb7130b25f168d2`, so
the schema is not byte-identical. Same class of drift as `0.146.0` to
`0.147.0`; not a stop.

Selected methods remain present: `initialize`, `model/list`, `thread/list`,
`thread/read`, `thread/start`, `thread/resume`, `thread/archive`,
`thread/delete`, `turn/start`, `turn/interrupt`, `item/started`,
`item/completed`, `item/plan/delta`, `subAgentActivity`,
`collabAgentToolCall`.

## Exec versus app-server

Exec mapped argv remains: `exec`, `--json`, `--ephemeral`,
`--ignore-user-config`, `--ignore-rules`, `--skip-git-repo-check`,
`--sandbox`, `read-only`. Official exec help SHA-256
`23e8b383723998a8ae8427c449ebe77a88ac82b04b4f8a18aad62925b9d3d0ee`.
Host `0.147.0` exec help still matches frozen `444f5b0c9ccbf961a3ba12ad3099074106b5ff757df854dd718f93b4dcd3a174`.

JSONL processor `codex-rs/exec/src/event_processor_with_jsonl_output.rs`
and `codex-rs/exec/src/exec_events.rs` are byte-identical with `0.147.0`
(`d43476319a61c53369055fdbbd7c093100b23bc93f9b01365db0af4c96df3e2c` /
`c404928e0f2a463e19d1b263081c9d5e0380aec9f651a05ee0766f7bb7527f32`).
Keep `codex.exec.jsonl-v1`.

App-server mapped flags remain `--listen` / `stdio://`. Keep
`codex.app-server.v2.workspace-roots` and lifecycle
`codex.app-server.lifecycle.v1.strict-descendant-hard-delete`.
Thread-catalogue methods remain; the `0.147.0` ceiling was the qualified
bound, not a protocol absence.

## Unused surfaces

Do not map:

- `exec fork` (new help line)
- top-level CLI `fork`
- schema `thread/fork` (`ThreadForkParams`)
- changelog extras (TUI `/export`, Bedrock provider, async hooks, credits UI)

## Segment decision for card 309

Compatible extension. Same behavior revisions. Raise `codex.cli` through
exact `0.148.0` on exec, app-server, lifecycle, and thread-catalogue
ceilings. Do not add a new milestone. Keep gaps `0.82.0..=0.83.x`,
`0.108.0`/`0.109.0`. After qualification, later stables remain
UnverifiedNewer; `0.148.1` is unpublished, so the synthetic later point
becomes `0.148.1`.

Host stays on `0.147.0` and remains Qualified. Official `0.148.0` becomes
Qualified Maintained. No new public operation. No contract beyond
currentness of the existing Codex window except the moving thread-catalogue
ceiling named by Contract 048.
