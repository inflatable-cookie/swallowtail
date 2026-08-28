# 258 Qwen Headless 0.22.3 Identity

Status: promoted
Owner: Tom
Date: 2026-08-28
Card: g05 batch 012

## Question

Is official npm `@qwen-code/qwen-code` `latest` = `0.22.3` (via published
`0.22.2`) a compatible extension of exact Maintained `0.21.15`
`qwen-code.headless.v0.21.15-reasoning-control`, a private milestone,
a new driver/facade, or a stop? Current `main` ceiling is `0.22.1`.
Ignore dist-tags `preview` and `nightly`.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Qwen headless | not installed | `0.19.11..=0.20.1`, `0.21.0..=0.21.14`, exact `0.21.15`, `0.22.0..=0.22.1` | operator-named family; official npm `latest` is `0.22.3` |

Gemini stays deferred. Do not flatten this family onto Model Studio or
ACP. Do not reopen unpublished `0.20.2` or `0.21.16`. Do not extend
reasoning or budgets past exact `0.21.15`.

## Method

Compared npm `@qwen-code/qwen-code@0.22.3` and `@0.22.2`, GitHub tags
`v0.22.3` and `v0.22.2`, public commit `09825973` (`v0.22.3`), and
selected git blobs
`packages/cli/src/nonInteractive/types.ts`,
`packages/cli/src/nonInteractive/control/controllers/systemController.ts`,
`packages/cli/src/config/config.ts`,
`packages/cli/src/nonInteractive/session.ts`,
`packages/core/src/core/reasoning-effort.ts`,
`packages/core/src/core/openaiContentGenerator/provider/dashscope.ts`,
`packages/core/src/config/approval-mode.ts`,
`packages/core/src/core/permissionFlow.ts`,
`packages/core/src/core/plan-mode-shell-policy.ts`, and
`packages/core/src/tools/exitPlanMode.ts`
against the frozen `0.22.1` / `0.22.2` corpora. Selected mapped flags
were checked in `config.ts`. Official artifacts stayed in `/tmp`.

No provider prompt. No live catalogue. No live headless session. Host
install was not present and was not replaced. Dist-tags `preview` and
`nightly` were not installed.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `qwen` absent from PATH; missing install is not a gap |
| Official npm latest | `0.22.3` | published 2026-08-28T17:30:35.909Z; integrity `sha512-8Ngy/ZEn+idOyN3k52K9TNu/XSkNfS2hyzsikeSDe79kRd2/eMYbWLOZq6LHSGVYXVNpY6ktpfZLthxY5AHWeA==`; gitHead `09825973e7d3c3fd07e17909c396aa62f48ce51f`; tarball SHA-256 `2521d3ef3a1ffc21f6c876218922f628ea8bce4ea290d8d2a752e7085089ea9a` |
| Published intermediate | `0.22.2` | published 2026-08-26T12:55:44.532Z; GitHub `v0.22.2` 2026-08-26T12:57:21Z; gitHead/tag `0d573e45275fdc800ebc6b458fd019ccc6e7b7bf` |

Published stables after previous ceiling `0.22.1`: `0.22.2` then
`0.22.3`. npm still has no `0.21.16` and no `0.22.4`. Dist-tag `preview`
is `0.22.2-preview.1` and is ignored. Nightly is ignored. `cli-entry.js`
digest `68cb29eb7ccc936d78ece5564ef55cae41a55b630e6657dc417c1f2e561cf4c9`
is identical from `0.21.15` through `0.22.3`. GitHub release `v0.22.3`
published 2026-08-28T17:16:38Z on the same commit.

## Selected protocol

`0.22.2` stream types, catalogue controller, session, dashscope,
`reasoning-effort.ts`, and Plan blobs are byte-identical to `0.22.1`.
`0.22.2` `config.ts` adds unmapped `hostPolicy.provisionalWorkspace`.

`0.22.3` mapped-subset deltas:

- `types.ts` (`9cf8d998…`) and `systemController.ts` (`737d581b…`) match
  `0.22.2`
- `reasoning-effort.ts` (`aac10131…`) and `dashscope.ts` (`a3da0cd4…`)
  match `0.22.2`
- Plan blobs `approval-mode` (`103535c5…`), `permissionFlow`
  (`f2cb3dbb…`), `plan-mode-shell-policy` (`799ce4b8…`), and
  `exitPlanMode` (`1174d6fa…`) match `0.22.2`
- `config.ts` (`a724ecb2…`, was `d1473780…`): GeminiMd-to-Memory rename
  plus unmapped `tools.eager`. All selected `.option(...)` names stay
- `session.ts` (`a61977d8…`, was `192bb3fe…`): comments `gemini.tsx` to
  `llm.tsx` and `getGeminiClient` to `getLlmClient`. Unmapped rename,
  not a public operation

Selected mapped flags remain:
`--safe-mode`, `--approval-mode`, `--core-tools`, `--exclude-tools`,
`--max-wall-time`, `--max-tool-calls`, `--max-session-turns`,
`--include-partial-messages`, `--input-format`, `--output-format`, and
exact `--resume`. Ambient `--continue` stays unselected. Catalogue still
filters `imageOnly`. Nested `goal_state` stays unmapped.

`0.22.3` release notes: no known breaking changes. Channels named
sessions, Web Shell extras, daemon session APIs, and OTel stay unmapped.
`--core-tools` still names `list_directory`.

Decoder specimen remains `qwen-code-v0.19.11`.

No new mapped public operation. No live session required.

## Segment decision for card 013

Compatible extension of existing
`qwen-code.headless.v0.21.15-reasoning-control`. Same axis. Keep
AllowUnverified. Keep Deprecated `0.19.11..=0.20.1`. Keep unpublished
stables `0.20.2` and `0.21.16` incompatible.

Keep exact `0.21.15`. Extend same-revision `0.22.0..=0.22.3`. Raise
latest qualified to `0.22.3`. Qualify published intermediate `0.22.2`.
After qualification, synthetic unpublished later stable is `0.22.4`.
Preview `0.22.2-preview.1` stays a prerelease reject.

Do not extend reasoning or budgets past exact `0.21.15`. No new public
operation. No provider prompt.

## Sources

- npm `@qwen-code/qwen-code@0.22.3` and `@0.22.2`
- [GitHub `v0.22.3`](https://github.com/QwenLM/qwen-code/releases/tag/v0.22.3)
- [GitHub `v0.22.2`](https://github.com/QwenLM/qwen-code/releases/tag/v0.22.2)
- commit [`09825973`](https://github.com/QwenLM/qwen-code/commit/09825973e7d3c3fd07e17909c396aa62f48ce51f)
- frozen `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.22.1/`
- frozen `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.22.2/`
