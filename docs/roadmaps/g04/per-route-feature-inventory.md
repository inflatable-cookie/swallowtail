# Per-Route Feature Inventory

Status: active ledger
Owner: Tom
Created: 2026-08-27
Source: [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
Programme: [Per-Route Feature Completion](./per-route-feature-completion.md)

## Purpose

Keep one live disposition for the original 85-item advanced-feature inventory.
The triage note remains the source assessment. This ledger owns counts and
sequence after g04.082.

## Count

| Disposition | Items | Meaning |
| --- | ---: | --- |
| Closed by a numbered lane | 45 | Delivered, evidence-stopped, corrected, or explicitly withheld by g04.035-g04.082. |
| Active qualification backlog | 30 | A bounded route-local evidence question remains. |
| No active lane | 10 | Current policy, exact-route evidence, or a contract boundary prevents useful qualification now. |
| **Total** | **85** | Original inventory, exactly once. |

Closed original item ids:
`1, 5, 7, 16, 19, 23-27, 32-33, 35-38, 40, 43-45, 50, 52-53, 55-60,
62-63, 65-68, 72-74, 77-78, 81-85`.

The programme progress section owns the detailed outcome and evidence link for
each closed lane. `Closed` does not mean every feature shipped. Honest empty
sets and durable withholds count as dispositions and do not remain in the live
queue.

## Active Qualification Backlog

| Original ids | Route | Control family | Current posture |
| --- | --- | --- | --- |
| 2, 4, 6, 8-9 | `claude-code.headless` | Fast, compaction, spend, advisor, permission modes | qualify separately; never widen permissions by default |
| 11-12 | `claude-code.response-only` | Fast, compaction | route-local proof must preserve tool-free response-only behavior |
| 15 | `codex.exec` | Fast / service tier | exact config, model, and returned-state evidence |
| 18, 20-21 | `codex.app-server` | Fast, personality, Plan effort | three distinct controls; no promotion from exec |
| 28 | `cursor-agent.acp` | model parameters | exact ACP option and selected-value confirmation required |
| 29-30 | `gemini-cli.acp` | sandbox, thinking | qualify independently on exact enterprise API-key route |
| 31 | `gemini-cli.headless` | sandbox | qualify independently on exact enterprise API-key route |
| 34 | `grok-build.acp` | web-search disable | exact ACP spawn and applied-state evidence |
| 41-42 | `cline.acp` | Plan, model | no promotion from headless evidence |
| 46-47 | `goose.acp` | builtins, mode | host extension authority and session configuration stay distinct |
| 48-49 | `kiro.acp` | effort, agent | ACP only; deferred Kiro headless work does not own these rows |
| 54 | `mistral-vibe.headless` | agent beyond Plan | qualify only non-bypass profiles |
| 61 | `opencode.http` | task/subagent permission | attached-server configuration and permission truth required |
| 64 | `kimi-platform.chat` | thinking disabled outside K3 | exact model-family and request semantics required |
| 69-71 | `anthropic.managed-agent` | effort, Fast, tools/MCP | three independent control families |
| 76 | `openai.realtime` | reasoning effort | exact realtime model and lifecycle evidence |
| 79 | `bedrock.runtime` | model-specific thinking | per-model schema only; never a generic effort string |

## No Active Lane

| Original id | Route / feature | Disposition | Reopen trigger |
| ---: | --- | --- | --- |
| 3 | Claude Code headless Agent teams | excluded process topology | operator requests a bounded topology feature |
| 10 | Claude response-only UltraCode | evidence-blocked on the exact binary and response-only posture | official visible support composes with tool-free response-only execution |
| 13 | Claude Agent ACP UltraCode | unconfirmed ACP control | exact ACP selection and confirmation surface appears |
| 14 | Claude Agent ACP Agent teams | excluded process topology | operator requests a bounded topology feature |
| 17 | Codex exec multi-agent | excluded process topology | operator requests a bounded topology feature |
| 22 | Codex app-server multi-agent | excluded process topology | operator requests a bounded topology feature |
| 39 | Kimi headless Plan | exact selected prompt command rejects composition | selected route/version changes with exact composable evidence |
| 51 | Deep Agents skills / memory | host-path authority, not a bounded generation control | a consumer-owned path and lifetime contract exists |
| 75 | xAI WebSocket multi-agent model | catalogue/model choice, not a generic run toggle | exact production model route is separately qualified |
| 80 | Bedrock tools / guardrails | current runtime guide excludes both | contract and guide explicitly admit a bounded surface |

## Completed Parallel Qualification Wave

g04.082 completed four route-distinct evidence lanes:

1. item 19 — `codex.app-server` model verbosity — g04.082 card 228
2. item 32 — `gemini-cli.headless` thinking configuration — card 229
3. item 81 — `bedrock.runtime` latency / service tier — card 230
4. item 83 — `ollama.attached` think `max` — card 231

Research 229-232 each admit an honest empty deliver-now set. Item 19 has no
typed confirmable app-server verbosity seam. Item 32 lacks an adapter-bound
isolated settings seam and effective-value confirmation. Item 81 cannot close
Bedrock model, region, inference-profile, account, or returned-state truth at
preparation. Item 83 cannot prove selected-model `max` membership from generic
Ollama thinking capability. No production binding or acceptance lane follows.

[g04.082](082-parallel-per-route-feature-qualification.md) owns the completed
wave. The next wave must be compiled from the remaining 30 active candidates
with the same disjoint worker ownership and serial shared-state promotion.

## Update Rule

After a numbered lane lands, move its original id from the active backlog to
the closed set, update all three counts, and link the outcome from the
programme. Do not leave a stopped evidence lane in the active count. Do not
recount from the append-only triage history.
