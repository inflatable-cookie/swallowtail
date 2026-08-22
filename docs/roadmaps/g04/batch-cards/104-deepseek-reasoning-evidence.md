# 104 DeepSeek Reasoning Evidence

Status: ready
Owner: Tom
Created: 2026-08-22
Milestone: [g04.038 DeepSeek Continuation Reasoning Controls](../038-deepseek-continuation-reasoning-controls.md)
Depends on: Research 023, 067, and 169

## Goal

Freeze exact current DeepSeek V4 reasoning-control evidence and define the
smallest effort and thinking-mode subset that preserves exact structured-run
and private direct-continuation behavior.

## Method

1. Recheck official V4 Pro/OpenAI-facade documentation for
   `reasoning_effort`, `thinking.type`, tools, `reasoning_content`, replay, and
   `max_tokens`.
2. Freeze exact URLs, retrieval date, stable specimens, digests, and the
   existing facade/model identity without live provider work.
3. Classify exact `low`, `high`, and `max` values. Record `medium`, `xhigh`,
   unknown values, provider aliases, defaults, and mappings separately; never
   silently normalize them.
4. Classify `thinking.type=enabled|disabled` separately from effort and
   separately for structured runs and tool-continuation sessions.
5. Revalidate every attempt shape: initial tool request, tool-result replay,
   final stream, later user turn, cancellation/failure, and fresh restoration.
6. Decide whether the current facade revision and Contract 030 remain valid for
   each deliver-now combination. Stop on a needed revision or contract change.
7. Decide exact portable `ReasoningSelection` mappings and whether thinking
   disable has any safe typed representation. Do not invent a generic boolean.
8. Freeze deterministic corpus changes and write/index promoted Research 186
   with per-control and per-profile dispositions.

## Acceptance Criteria

- exact current official evidence is frozen without secrets
- values, aliases, thinking modes, models, profiles, and attempt shapes have
  explicit dispositions
- the current facade/contract decision is explicit
- private continuation, tool, cache, and claim bounds stay intact
- Research 186 is promoted and indexed
- production code, claims, matrices, architecture, and changelog are unchanged
- `effigy validate:focused swallowtail-adapter-deepseek` passes
- `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- `git diff --check` passes

Auto-continue to card 105 only when at least one useful exact control extends
the current facade without a contract or compatibility-segment change.

## Stop Conditions

- current official behavior requires a new facade revision or contract change
- no useful exact value/profile combination survives
- selection cannot remain fixed through every continuation attempt
- safe binding needs aliasing, raw strings, or a generic thinking control

## Out Of Scope

- production binding or dispatch
- V4 Flash, retired aliases, other facades, tools expansion, or output changes
- live provider, account, balance, or currentness work

