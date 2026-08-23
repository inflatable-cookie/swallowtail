# 2026-08-23 g04.050 DeepSeek Thinking Mode Closeout

Status: merged
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

Worker validation passed: focused package validation (45 tests), affected-
package verification, examples, route QA, Northstar QA, research/log/roadmap
index QA, next-action QA, package API, and `git diff --check`. Exact-head CI
passed after rerunning one unrelated Codex deadline fixture flake. No
credentials, account state, live provider request, or paid work was used.

PR: [#49](https://github.com/inflatable-cookie/swallowtail/pull/49).
Implementation head at PR creation: `ac0378d6d5ce7f1a2cae4463d6606362a6a1e4a6`.
Worker branch: `t3code/review-deepseek-thinking-handoff`. PR 49 merged by exact
fast-forward at `52413da0bcee940d006e800fa36fb111b156b8af`.

## Shared Closeout Delta

Applied after merge. Architecture, route/feature matrices, programme and index
truth, changelog, and the promoted triage disposition now describe disabled
structured runs and enabled-only continuation. Contract 029, the opaque facade,
private behavior revision, model route, and currentness claim remain unchanged.
g04 remains active at 50 roadmaps. The sole Next Task reassesses the remaining
promoted per-route inventory inside g04; generation closure waits for explicit
operator direction.
