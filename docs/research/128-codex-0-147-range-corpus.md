# 128 Codex 0.147.0 Range Corpus

Status: promoted
Owner: Tom
Date: 2026-08-17
Card: g03 batch 231

## Question

Is official stable Codex CLI `0.147.0` a compatible extension of the current
`0.146.0` claims, a new adapter-private behavior milestone, or a stop?

## Method

Compared npm `@openai/codex@0.147.0`, local `codex --version`, GitHub tag
`rust-v0.147.0`, generated app-server JSON Schema, the frozen `0.146.0`
compatibility corpus, Research 120, and the g03.047/048/199 repairs.

No provider prompt, install, update, or claim edit.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `0.147.0` (published 2026-08-07T01:47:21.081Z) |
| npm integrity | `sha512-EQLEXecAG2ptxI7UpBMo2TR/ga5596/c/OsYF/0LoUDh5JANZ7IoGqlzBEWbuEVQ76JePIbtTW/ihCkp1a7Z3w==` |
| npm shasum | `1792030d147156695a2b86db0ec1a000ab9a83fc` |
| git tag | `rust-v0.147.0` |
| peeled commit | `be6e8eac029b183056b7e4402879f15d2c85f61b` |
| local CLI | `codex-cli 0.147.0` |
| later stable | none; `0.148.0` exists only as alpha |

Generated schema: `codex app-server generate-json-schema` exit 0.
`codex_app_server_protocol.v2.schemas.json` SHA-256
`f3dec1e031d99a420b137b903f02196d4325eece57620c925bb7130b25f168d2`.
That digest differs from the frozen `0.146.0` stable bundle, so the schema
is not byte-identical.

Selected methods remain present: `initialize`, `model/list`, `thread/list`,
`thread/read`, `thread/start`, `thread/resume`, `thread/archive`,
`thread/delete`, `turn/start`, `turn/interrupt`, `item/started`,
`item/completed`, `item/plan/delta`, `subAgentActivity`,
`collabAgentToolCall`.

## Exec versus app-server

Exec has no 0.147-specific defect record. The maintained revision
`codex.exec.jsonl-v1` already covers `0.122.0..=0.146.0` and currently
classifies `0.147.0` as UnverifiedNewer.

App-server live drift already repaired while UnverifiedNewer:

- g03.047 — `item/plan/delta` malformed-inbound diagnostics (method already
  mapped; extra-field warning was provider stderr)
- g03.048 — `item/started` before `item/tool/call`; runtime adopts late
  correlation once
- g03.199 / Research 120 — child `turn/started` can precede spawn
  completion; `subAgentActivity` kind `started` is admission evidence

Those repairs are more-tolerant decode on the existing driver. They are not
0.147-only dispatch. Frozen `0.146.0` fixtures still pass.

Thread-catalogue methods remain on the 0.147 schema. The current
`supports_thread_catalogue_version` ceiling at `0.146.0` is the qualified
bound, not a protocol absence.

Lifecycle hard-delete from `0.140.0` has no 0.147-specific break.

## Segment decision for card 232

Compatible extension. Same behavior revisions. Raise `codex.cli` through
exact `0.147.0` on exec, app-server, lifecycle, and thread-catalogue
ceilings. Do not add a new milestone. After qualification, later stables
remain UnverifiedNewer; there is no stable `0.148.0`, so the synthetic
later point becomes `0.148.0`.

No new public operation. No contract beyond currentness of the existing
Codex window.
