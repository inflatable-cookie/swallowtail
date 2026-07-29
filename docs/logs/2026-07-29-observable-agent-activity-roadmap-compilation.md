# 2026-07-29 Observable Agent Activity Roadmap Compilation

## Context

Nucleus and other Swallowtail consumers need to render intermediate assistant
messages, provider-visible reasoning summaries, tool calls and results,
commands, file changes, plans, tasks, and related agent work.

The existing runtime already delivers ordered bounded events through prepared
run and turn handles. The audit found that the portable vocabulary is too
thin: several rich Codex and ACP events become empty generic progress before a
consumer can project them.

## Evidence

- current Codex app-server documentation exposes stable item lifecycle, plan,
  reasoning-summary, command-output, file-change, tool, subagent, review,
  compaction, hook, and request events
- T3 Code commit
  `694f8d1c6eaaabafbf5c2861ae524174919ef625` demonstrates the useful split:
  canonical provider-runtime events, separate server projections for messages
  and work activity, and client-owned collapsed grouping
- Swallowtail run and turn handles already expose the required stream seam
- `RuntimeEvent` lacks general activity identity, lifecycle, content ownership,
  and route fidelity
- `StreamingEvents` is only binary
- Codex app-server, Claude Agent ACP, Gemini ACP, Kimi ACP, Kimi headless, and
  Codex exec contain confirmed semantic flattening

No provider effect occurred.

## Decision

Promote one observable-activity boundary:

- reuse the existing run and turn event streams
- add operation-local activity identity and exact lifecycle
- add typed assistant, reasoning-summary, plan, command, file, and tool
  content streams
- add exact disclosure and route activity profiles
- preserve unknown semantic activity without raw provider payloads
- keep hidden reasoning excluded
- keep consumer messages, work-log persistence, grouping, collapse, labels,
  authorization, and UI downstream

Direct inference exposes only the activity its selected API supplies. It does
not inherit harness steps. Realtime media retains its dedicated lifecycle.
Catalogue and serving-only operations remain not applicable.

## Planning

- Research 063 is promoted.
- Contract 044 is active.
- System architecture records the ownership boundary.
- g02 now contains 40 roadmaps, inside its 30-50 range.
- Roadmaps g02.035-g02.040 sequence:
  1. common runtime and prepared profile
  2. Codex
  3. ACP
  4. remaining harnesses
  5. direct-inference truth
  6. provider-wide package evidence and consumer handoff
- Cards 119-137 provide the execution runway.
- Card 119 is the sole ready task.

## Continuation

Cards 059, 097, and 098 remain paused and in bounds. Grok Build remains in the
held backlog. No consumer repository edit, release mutation, live
authentication, provider call, or package publication is authorized by this
planning batch.

