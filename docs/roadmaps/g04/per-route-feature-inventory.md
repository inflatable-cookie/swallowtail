# Per-Route Feature Inventory

Status: active ledger
Owner: Tom
Created: 2026-08-27
Source: [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
Programme: [Per-Route Feature Completion](./per-route-feature-completion.md)

## Purpose

Keep one live disposition for the original 85-item advanced-feature inventory.
The triage note remains the source assessment. This ledger owns counts and
sequence after g04.089.

## Count

| Disposition | Items | Meaning |
| --- | ---: | --- |
| Closed disposition | 81 | Delivered, evidence-stopped, corrected, explicitly withheld, inapplicable, or moved to its owning programme. |
| Active qualification backlog | 2 | A bounded route-local evidence question remains. |
| Active delivery | 0 | No qualified delivery row is waiting for binding or acceptance. |
| Parked | 2 | A named model/schema or contract/guide trigger is required before qualification. |
| **Total** | **85** | Original inventory, exactly once. |

Closed original item ids:
`1-46, 48, 50-78, 81-85`.

The programme progress section and remainder audit own the detailed outcome for
closed rows. `Closed` does not mean every feature shipped. Honest empty sets,
durable withholds, inapplicable rows, and transfers to another owning programme
count as dispositions and do not remain in the live queue.

## Active Qualification Backlog

| Original ids | Route | Control family | Current posture |
| --- | --- | --- | --- |
| 47 | `goose.acp` | mode | g04.090 card 256; ACP membership and confirmation; never auto-approve by default |
| 49 | `kiro.acp` | agent profile | g04.090 card 257; exact ACP membership, failure, and confirmation required |

## Active Delivery

No original item currently has an active delivery lane.

## Completed Qualification Wave VI

g04.089 completed four package-distinct evidence-only lanes:

1. item 9 — `claude-code.headless` permission modes — card 252 / Research 249
2. item 46 — `goose.acp` builtins — card 253 / Research 250
3. item 48 — `kiro.acp` effort — card 254 / Research 251
4. item 54 — `mistral-vibe.headless` agent profiles — card 255 / Research 252

Research 249-252 each admit an honest empty deliver-now set. PRs 109, 107, 106,
and 108 landed fast-forward-only in lane order through `e28979a0`. Original
items 9, 46, 48, and 54 close as evidence stops. No production binding follows.

## Remainder Audit

The 2026-08-28 audit removed sixteen rows from the live per-route queue:

| Original ids | Disposition |
| --- | --- |
| 3, 14, 17, 22 | process topology, not a bounded composer run control |
| 10-13 | dominated by exact Claude UltraCode, Fast, compaction, response-only, and ACP evidence; no new route-local seam |
| 39 | exact selected Kimi headless prompt command rejects Plan composition |
| 51 | host-path skill/memory authority moved to harness skill-discovery and resource-lifetime triage |
| 61 | existing OpenCode permission exchange already relays provider `task` requests; attached-server subagent enablement is not run control |
| 64 | disabled thinking belongs to another Kimi model family; selected K3 requires reasoning |
| 69-71 | effort, speed, builtins, and MCP belong to the operator-owned Managed Agent definition, not per-run input |
| 75 | xAI multi-agent is a distinct model route, not a generic toggle |

These are closed programme dispositions. An upstream or product-boundary
change may justify a new assessment; they do not remain recurring queue items.

## Parked

| Original id | Route / feature | Disposition | Reopen trigger |
| ---: | --- | --- | --- |
| 79 | `bedrock.runtime` model-specific thinking | no selected model/schema row to qualify | exact admitted Bedrock model, region/profile, and official request schema are named |
| 80 | `bedrock.runtime` tools / guardrails | current contract and runtime guide exclude both | contract and guide explicitly admit a bounded surface |

## Active Qualification Wave VII

g04.090 owns two package-distinct evidence-only lanes:

1. item 47 — `goose.acp` mode — card 256 / Research 253
2. item 49 — `kiro.acp` agent profile — card 257 / Research 254

Run both through manual harness handoffs in parallel. Shared closeout remains
serial after Goose then Kiro integration.

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
wave.

## Completed Parallel Qualification Wave II

g04.083 completed four route-distinct evidence lanes from the 30 active items:

1. item 2 — `claude-code.headless` Fast mode — card 232
2. item 15 — `codex.exec` Fast / service tier — card 233
3. item 30 — `gemini-cli.acp` thinking configuration — card 234
4. item 76 — `openai.realtime` reasoning effort — card 235

Research 233-235 admit honest empty deliver-now sets. Research 236 admitted five
future session-scoped `minimal|low|medium|high|xhigh` rows on exact
`gpt-realtime-2.1`, while production was empty. Items 2, 15, and 30 closed as
evidence stops. Item 76 closed through g04.084 and PR 90 at `266ec857`.

[g04.083](083-second-parallel-per-route-feature-qualification.md) owns the
completed lanes and Research 233-236. PRs 86, 88, 87, and 85 landed serially
through `c918d301`.

## Completed Parallel Qualification Wave III

g04.085 completed four route- and package-distinct evidence lanes from the 26
active qualification candidates:

1. item 4 — `claude-code.headless` autocompaction — card 238
2. item 20 — `codex.app-server` personality — card 239
3. item 31 — `gemini-cli.headless` sandbox — card 240
4. item 41 — `cline.acp` Plan mode — card 241

Research 237-239 admit honest empty deliver-now sets. Research 240 admits one
exact `cline.acp` `3.0.55` `HarnessMode::Plan` row through pre-prompt
`session/set_config_option` confirmation. Items 4, 20, and 31 close as evidence
stops. Item 41 moves to active delivery under g04.086. PRs 94, 93, 91, and 92
landed serially through `abdaefd2`.

[g04.085](085-third-parallel-per-route-feature-qualification.md) owns the
completed wave. [g04.086](086-cline-acp-plan-mode.md) owns the promoted delivery
row.

## Completed Cline ACP Plan Delivery

g04.086 delivered original item 41 on exact `cline.acp` `3.0.55`. Cards
242-243 bind portable `HarnessMode::Plan` through one pre-prompt
`session/set_config_option` request and exact selected-value confirmation.
Omission, permission, resource, isolation, lifecycle, and cleanup truth remain
unchanged. PR 95 landed by fast-forward at `3f56aeb4`.

## Completed Parallel Qualification Wave IV

g04.087 completed four package-distinct evidence lanes:

1. item 6 — `claude-code.headless` spend cap — card 244 / Research 241
2. item 18 — `codex.app-server` Fast mode — card 245 / Research 242
3. item 28 — `cursor-agent.acp` model parameters — card 246 / Research 243
4. item 29 — `gemini-cli.acp` sandbox — card 247 / Research 244

All four Research records admit honest empty deliver-now sets. The blockers
are route-specific: billing-unit mismatch, unclosed Fast membership/wire/
persistence truth, account-gated ACP parameter membership, and ambient
sandbox/re-exec/confirmation truth. PRs 99, 97, 98, and 96 landed serially
through `e40a5407`. No production binding follows.

[g04.087](087-fourth-parallel-per-route-feature-qualification.md) owns the
completed wave.

## Completed Parallel Qualification Wave V

g04.088 completed four package-distinct evidence lanes:

1. item 8 — `claude-code.headless` advisor — card 248 / Research 245
2. item 21 — `codex.app-server` Plan-mode effort — card 249 / Research 246
3. item 34 — `grok-build.acp` web-search disable — card 250 / Research 247
4. item 42 — `cline.acp` model selection — card 251 / Research 248

All four Research records admit honest empty deliver-now sets. The blockers
are route-specific: advisor entitlement, ambient authority, attachment, and
spend; no typed app-server Plan-effort seam; unconfirmed Grok ACP search
suppression; and unclosed Cline provider/model membership and pre-effect
rejection. PRs 104, 101, 103, and 102 landed serially through `23278abe`. No
production binding follows.

[g04.088](088-fifth-parallel-per-route-feature-qualification.md) owns the
completed wave.

## Update Rule

After a numbered lane lands, move its original id from the active backlog to
the closed set, update all three counts, and link the outcome from the
programme. Do not leave a stopped evidence lane in the active count. Do not
recount from the append-only triage history.
