# 272 Claude Agent ACP 0.73.0 Identity

Status: promoted
Owner: Tom
Date: 2026-09-01
Card: g05 batch 044

## Question

After Research 271 selected Claude Agent ACP `0.72.0` as the next one-family
Upgrade Workflow, official npm `latest` moved to `0.73.0` during the unmerged
`0.72.0` PR. Operator restart: is official
`@agentclientprotocol/claude-agent-acp` `latest` = `0.73.0` a compatible
extension of `claude-agent.acp-adapter` through `0.70.0`, a new private
milestone, or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Agent ACP | installed `0.63.0` | `0.53.0..=0.70.0` excluding `0.58.0` | operator restart of the same family; Research 271 selected this family alone when latest was `0.72.0`; current official npm `latest` is `0.73.0` |

Do not flatten this family onto Claude Code headless, response-only, or
watcher. Do not reopen `kimi-code.acp`. Gemini stays deferred.

## Method

Compared npm `@agentclientprotocol/claude-agent-acp@0.70.0`, `@0.71.0`,
`@0.72.0`, and `@0.73.0` to the frozen `claude-agent-acp-0.70.0` corpus,
GitHub tags `v0.70.0` through `v0.73.0`, and a complete extracted-package
file inventory for `0.70.0` (33 files) → `0.71.0` (96) → `0.72.0` (96) →
`0.73.0` (96), frozen in `dist-inventory.json`. That inventory is an exact
changed/identical/added oracle, not a complete semantic changelog of every
internal line. Remaining named files stay unmapped with reason. `0.71.0` and
`0.72.0` stay intermediate supporting evidence, not a standalone ceiling.

Host `claude-agent-acp --version` `0.63.0` was observed and not replaced.
No provider prompt. No live ACP initialize, session, or catalogue. Official
artifacts stayed in `/tmp` and were not executed.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.63.0` | `dist/index.js` SHA-256 `260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`; size 3200; matches frozen 0.70 host digest; gitHead `15979bba7907484ee22111cdc33b79b0bdcd452d`; ACP SDK `1.3.0`; Agent SDK `0.3.220` |
| Official npm/GitHub latest | `0.73.0` | published 2026-09-01T20:27:53.428Z; GitHub 2026-09-01T20:25:53Z; integrity `sha512-xKnGIntdBbr2dDS2NEsVGdjoLH62EaWjfYlp/U7TYdxUJzERlApe2gliYW3rVFTeWGjG0dUyPszhG9TWhsqGlA==`; shasum `596abd41c6e2f86d7a3d9c89b30dc494a5b4897e`; tarball SHA-256 `eb03d0c6c1934726535d5c6b8defd3b37c2c2d77f5d9d037d3a97d624ea733c3`; gitHead `ea7076c0bc324603e65d8c124b7573f158749969`; ACP registry `claude-acp` `0.73.0` |
| Published intermediate | `0.72.0` | not a standalone ceiling; published 2026-09-01T18:53:49.840Z; gitHead `d3eff191576abcaa7592bb3ac55ff7534e4fe35d`; ACP SDK `1.4.0`; Agent SDK `0.3.252` |
| Published intermediate | `0.71.0` | published 2026-09-01T17:33:55.522Z; gitHead `889346fcf5ff546f7c07e546dbc42de37ce0992d`; ACP SDK `1.3.0`; Agent SDK `0.3.238` |
| Frozen previous ceiling | `0.70.0` | tarball SHA-256 `da2cf1b5f66981578313126a49002c4ae7e0c7e71d92b545f2b834835c6db465` matches the frozen corpus |

Published stables after previous ceiling `0.70.0`: exactly `0.71.0`,
`0.72.0`, and `0.73.0`. Unpublished: `0.58.0`, `0.69.1`, `0.70.1`,
`0.71.1`, `0.72.1`, `0.73.1`, `0.74.0`. First later unpublished stable is
`0.74.0`. Do not exclude `0.70.1` / `0.71.1` / `0.72.1` / `0.73.1`; family
exclusion remains `0.58.0` only.

npm gitHead matches GitHub tags for `0.70.0` through `0.73.0`.

## Selected protocol

Mapped routes compared to frozen `0.70.0`: initialize, session/new, prompt,
cancel, load, resume, close, delete, request_permission, elicitation/create,
set_config_option (`model`/`effort`/`mode`), and set_mode (`acceptEdits`).

Unchanged mapped subset:

- `dist/index.js` identical `0.70.0` through `0.73.0`
  (`9d73d1f0f121fb96cc8badb28c22d5bff02d8582eb2e40360a81c189e1b9422a`)
- `dist/elicitation.js` identical `0.64.0` through `0.73.0`
  (`d40be7d05a0ca9f65621a54fd61aa1a903875d5afcd95051c1f86b561abdd095`)
- `dist/lib.js`, `dist/settings.js`, and `dist/utils.js` identical
  `0.70.0` through `0.73.0`
- every `dist/**` file byte-identical `0.72.0` to `0.73.0`
- initialize: protocol v1; promptCapabilities image+embeddedContext;
  mcpCapabilities http+sse; loadSession true; mapped sessionCapabilities
  still additionalDirectories/close/delete/fork/list/resume
- effort still `id: "effort"`, `category: "thought_level"`
- mode still `id: "mode"`, `category: "mode"`; `plan` and `acceptEdits`
  still advertised
- permission kinds still `allow_once` / `allow_always` / `reject_once`;
  Swallowtail still skips `allow_always` and `reject_always`
- prompt `usage` still `inputTokens` / `outputTokens` /
  `cachedReadTokens` / `cachedWriteTokens` / `totalTokens`
- `stopReason` domain still `end_turn` → Completed, `cancelled` →
  Cancelled, `max_tokens`/`max_turn_requests`/`refusal` → ProviderFailed,
  unknown → `stop_reason_unsupported` RuntimeFailed
- cancel still mapped `session/cancel`
- load/resume/close/delete mapped surface unchanged

## 0.71.0 deltas (intermediate)

GitHub `#1004` keeps mapped mode `id: "mode"`, `category: "mode"`; `plan`
and `acceptEdits` still advertised. `dontAsk` drops from advertised
`availableModes` (already unmapped). Clear-context / ExitPlanMode stay
unmapped.

GitHub `#1045` defers steering while user input is pending. Steering
`_meta` stays unmapped. Permission callback observable contract stays
`session/request_permission` with kinds `allow_once` / `allow_always` /
`reject_once`.

Newly emitted update kinds, all unmapped: `subagent_spawned`,
`subagent_state_update`, `async_task_spawned`, `async_task_progress`,
`async_task_state_update`.

## 0.72.0 deltas (intermediate)

Effort: session/new no longer calls `applyFlagSettings`; picker
`currentValue` comes from per-model `settingsEffortForModel`;
`effortPinnedByUser` gates later switches. Swallowtail confirms effort
after model confirm via explicit `set_config_option(effort)` +
`confirm_reasoning`.

`user_message_uuid` result attribution, queued/cancelled turn ownership,
and stamped-unmatched empty-interruption fallback stay provider-internal.
PostModelSwitch mirrors CLI `/model` into `config_option_update` /
`current_mode_update`. PreModelSwitch veto at session/new logs and falls
back to `models[0]` instead of throwing. Changed failure point: session/new
no longer fails closed on a spawn-pin veto; Swallowtail still fails closed
on the following explicit `set_config_option(model)` + `confirm_model`
exact match.

## 0.73.0 deltas

GitHub `v0.72.0...v0.73.0` is ahead 2, behind 0: `#1066` Agent SDK pin
`0.3.252` → `0.3.257` and `#1067` release. Source files besides
`package.json`, lock, changelog, and release-please manifest are
unchanged. Complete dist inventory: added 0, removed 0, changed exactly
`package.json`, identical 95. `package.json` changes are version
`0.72.0` → `0.73.0` and Agent SDK pin. ACP SDK stays `1.4.0`. Every
`dist/**` file is byte-identical to `0.72.0`, including `acp-agent.js`
`e41014b49c5ac096b5e18a89f990ee0ec64452e440666b59dcf4e087f632e370` and
`tools.js` `d8053c7880d61a0ffa25c8bd367eb2d5adc78ee7c0a3eb65b4d33be3a6c07ab4`.

The Agent SDK pin is unmapped with reason: Swallowtail maps ACP stdio, not
the Agent SDK package. No new public mapped operation. No mapped behavior,
lifecycle, failure, support, capability, permission, usage, session
update, config/mode, or load/resume/close/delete delta versus `0.72.0`.

## Unmapped extras

Do not map:

- `0.71.0` additive `sessionCapabilities.subagents: {}`
- session titles; native subagent and async-task `sessionUpdate` kinds
- `#1004` clear-context / ExitPlanMode
- `#1045` steering deferral
- `reject_always`; Providers API; goal/Air/file-change `_meta`
- `0.72.0` effort seed / pin gating, result-attribution, PostModelSwitch
  mirroring, and PreModelSwitch session/new fallback, as classified above
- `0.73.0` Agent SDK pin `0.3.252` → `0.3.257` and `package.json` version
  bump; adapter dist is byte-identical to `0.72.0`

Wire `protocolVersion` remains `1`. ACP SDK is `1.3.0` on `0.70.0` and
`0.71.0`, `1.4.0` on `0.72.0` and `0.73.0`. Agent SDK is `0.3.232` /
`0.3.238` / `0.3.252` / `0.3.257`.

## Decision

Compatible extension of the mapped v1 subset on existing
`claude-agent.acp.initialize-meta-extensions-v7`.

Keep claim id `claude-agent.acp.window-2`, baseline `0.53.0`, exclusion
`0.58.0`, and `AllowUnverified`. Qualify published intermediates `0.71.0`,
`0.72.0`, and `0.73.0`. Extend Maintained v7 to `0.66.0..=0.73.0`. Raise
latest qualified to `0.73.0`. Synthetic later `UnverifiedNewer` is
unpublished `0.74.0`. No new milestone. Decoder specimens stay
`claude-agent-acp-v0.53.0-v0.61.0` and
`claude-agent-acp-v0.62.0-v0.64.0`.

Host `0.63.0` stays observation-only Qualified Deprecated. Claude Code and
the watcher stay untouched.

## Claim at observation

Production claims are unchanged in this research:

- latest qualified `0.70.0`
- `0.71.0`, `0.72.0`, and `0.73.0` `UnverifiedNewer`
- `0.58.0` incompatible
- `0.63.0` qualified Deprecated

Card 045 continues only because this identity admits a compatible
extension.
