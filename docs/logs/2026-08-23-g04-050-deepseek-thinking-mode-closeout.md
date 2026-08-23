# 2026-08-23 g04.050 DeepSeek Thinking Mode Closeout

Status: worker-complete
Owner: Tom
Milestone: g04.050

## Result

Research 197 was promoted with one non-empty deliver-now row: exact
`deepseek-v4-pro` structured runs may use adapter-local
`DeepSeekThinkingMode::disabled()` on the existing
`deepseek-openai-chat-2026-07-22` facade. The request emits
`thinking.type=disabled`, omits `reasoning_effort`, carries no portable
`ReasoningSelection`, and remains distinct from the existing enabled
`low|high|max` path.

Cards 140-141 implemented and accepted that row. Prepared input, immutable
evidence, plan, configured driver, request encoding, structured response
parsing, and deterministic fixtures agree. A non-null private
`reasoning_content` field under disabled mode fails closed; ordinary disabled
responses do not synthesize private continuation. Sessions, tool results,
later turns, restoration, and private replay remain enabled-only and unchanged.
The facade, private behavior revision, model route, cache acceptance, and
Contract 029 currentness claim did not change.

Validation passed: focused package validation (45 tests), affected-package
verification, examples, route QA, Northstar QA, research/log/roadmap index
QA, next-action QA, package API, and `git diff --check`. No credentials,
account state, live provider request, paid work, or merge was used.

PR and final pushed head are recorded in the worker handoff after publication.

## Shared Closeout Delta

Not applied by this worker. The orchestrator must decide any changes to shared
architecture, Contract 029, route/feature matrices, programme/front-door
indexes, changelog, the g04 generation boundary, and the sole Next Task
pointer after review and merge.
