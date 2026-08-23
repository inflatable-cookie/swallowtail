# g04.051 Qwen Headless Turn And Tool Budgets

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; g04.026; g04.041
Vision tags: explicit selection, bounded execution, route-local controls
Contract refs: 008, 011, 029, 033, 037, 052
Research: 017, 173, 189, 198 reserved by card 142

## Problem

Production route `qwen.headless` fixes `--max-session-turns 24` and
`--max-tool-calls 16` on every structured-run and turn child. Callers cannot
choose a smaller execution envelope even when their workload needs fewer
model/tool cycles.

Current official headless documentation and exact Qwen Code `0.21.15` source
describe both native limits. The source also exposes details that must not be
guessed: zero tool calls aborts before the first tool dispatch, turn accounting
must be reconciled with Swallowtail's one-prompt child shape, and a budget
terminal may surface through process status and stderr rather than one stable
semantic stream event.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind typed caller-decreasing
turn and tool-call budgets for exact Qwen Code `0.21.15` across structured
runs and every session child. Omission preserves the current `24` / `16`
command.

## Goals

- [x] freeze current official and exact `0.21.15` parsing, counter, lifetime,
      terminal, and version evidence
- [x] classify candidate turns `1..=24` and tool calls `0..=16` without
      treating those ranges as prequalified
- [x] settle zero-tool usefulness and exact turn accounting for this route's
      one-prompt child shape
- [x] classify ordinary and reasoning-selected structured runs, first turns,
      resumed turns, and fresh replacement children
- [x] promote Research 198 with an exact deliver-now table or honest stop
- [x] preserve omission as exact current `24` / `16` argv and behavior
- [x] bind only admitted values through typed adapter-local input, immutable
      plan/evidence, driver validation, and every spawned child
- [x] reject invalid or mismatched values before process start or user prompt
- [x] publish deterministic dispatch and terminal truth without claiming
      provider acceptance, completed work, quality, latency, or billing

## Non-Goals

- caller-increasing bounds, upstream unlimited `-1`, or values beyond `24` / `16`
- a selectable native wall time or a change to the current 60-second bound
- approval, permission, tool-set, subagent, model, provider, or credential changes
- portable output-token, reasoning, context, billing, or generic budget controls
- earlier or later Qwen Code versions, currentness work, or sibling routes
- live login, catalogue, prompt, provider request, paid work, release, or merge

## Named Scope

The lane is restricted to route `qwen.headless`, driver
`swallowtail.qwen.headless`, axis `qwen-code.package`, exact package
`0.21.15`, and the existing qualified ordinary and reasoning-selected child
transports.

The candidate Swallowtail domains are positive turn limits `1..=24` and tool-
call limits `0..=16`. These are proposed caller-decreasing subsets, not
upstream-domain claims. Card 142 must decide whether either subset is useful
and exact. Omission must retain the existing values and command bytes.

Card 142 must distinguish parser acceptance, counter lifetime, pre-dispatch
enforcement, process exit, stderr, stream events, terminal classification, and
cleanup. It must cover structured runs plus first, resumed, and fresh
replacement session children. It must also prove composition with the exact
`0.21.15` reasoning handshake for `qwen3.8-max` and
`qwen3.8-max-preview` without widening that model-qualified control.

The budgets remain adapter-local execution limits. They do not become shared
generation controls or a generic provider-options map. The current mandatory
host deadline, native 60-second wall bound, read-only allowlist, excluded tools,
safe mode, model route, credentials, cancellation, and cleanup remain fixed.
An empty Research 198 deliver-now set is an honest stop.

## Execution Plan

### Batch 51.1 — Exact Budget Evidence

- [x] Execute card 142.
- [x] freeze official and exact-package parsing, counting, lifetime, and
      terminal evidence
- [x] promote Research 198 with domain/profile/terminal dispositions

### Batch 51.2 — Conditional Budget Binding

- [x] Execute card 143 only when card 142 admits a non-empty deliver-now set.
- [x] add typed adapter-local caller-decreasing selection
- [x] preserve omission and every fixed safety boundary
- [x] bind admitted values across every child shape

### Batch 51.3 — Route-Local Acceptance

- [x] Execute card 144 only after card 143.
- [x] prove admitted, omitted, composed, rejected, terminal, and lifecycle truth
- [x] update route-local guidance and reserve the shared closeout delta

## Acceptance Criteria

- [x] only Research 198 deliver-now values and profiles prepare
- [x] omission preserves current `24` / `16` argv and behavior
- [x] input, plan/evidence, driver, and every child command agree exactly
- [x] reasoning-selected and ordinary children retain their existing transport
- [x] invalid values and knowable mismatches reject before effects
- [x] native wall time, host deadline, tool set, approval mode, and credentials
      are unchanged
- [x] budget exits, stderr, stream, terminal, cancellation, and cleanup are
      represented only to the exact proved boundary
- [x] default QA performs no install, login, credential, catalogue, prompt, or
      paid work
- [x] g04.051 closes only this route-local family; g04 remains active until the
      operator directs otherwise

## Lane Runway

- predecessor: g04.050 DeepSeek structured-run thinking mode
- this milestone: Qwen headless caller-decreasing turn/tool budget evidence and
  conditional adapter-local binding
- execution topology: one serial worker lane, cards 142-144
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if zero-tool or turn-count semantics cannot be closed without a live
  provider prompt or inference.
- Stop if process exit, stderr, stream, and Swallowtail terminal truth cannot be
  represented without hiding a budget failure.
- Stop if selection cannot remain exact across structured, first, resumed, and
  fresh replacement children.
- Stop if the existing adapter-local plan/evidence boundary cannot retain the
  selection without an unplanned shared contract or capability.
- Stop if delivery changes the native wall bound, host deadline, tool set,
  approval posture, reasoning mapping, model route, credentials, or currentness.

## Batch Cards

- [142-qwen-headless-turn-and-tool-budget-evidence.md](batch-cards/142-qwen-headless-turn-and-tool-budget-evidence.md) — complete
- [143-qwen-headless-turn-and-tool-budget-binding.md](batch-cards/143-qwen-headless-turn-and-tool-budget-binding.md) — complete
- [144-qwen-headless-turn-and-tool-budget-acceptance.md](batch-cards/144-qwen-headless-turn-and-tool-budget-acceptance.md) — complete

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 017 Qwen Headless Coverage Evidence](../../research/017-qwen-headless-and-post-managed-harness-coverage-evidence.md)
- [Research 173 Qwen Headless 0.21.15 Identity](../../research/173-qwen-headless-0-21-15-identity.md)
- [Research 189 Qwen Headless Reasoning Effort Evidence](../../research/189-qwen-headless-reasoning-effort-evidence.md)
- [Research 198 Qwen Headless Turn And Tool Budget Evidence](../../research/198-qwen-headless-turn-and-tool-budget-evidence.md)
- [Contract 008 Runtime Registration And Preflight](../../contracts/008-runtime-registration-and-preflight.md)
- [Contract 011 Runtime Conformance Profiles](../../contracts/011-runtime-conformance-profiles.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 052 Consumer And Operator Integration Documentation](../../contracts/052-consumer-and-operator-integration-documentation.md)
- [Qwen Headless Prepared Integration](../../guides/qwen-headless-prepared-integration.md)
- [Qwen Headless Mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Code v0.21.15](https://github.com/QwenLM/qwen-code/tree/v0.21.15)

## Closeout

g04.051 closes the Qwen headless caller-decreasing turn/tool-budget family at
the route-local dispatch boundary. Research 198 admits exact package
`0.21.15` turns `1..=24` and tool calls `0..=16`. Omission keeps `--max-session-turns
24 --max-tool-calls 16`. Wall time stays `60s`. Counters are process-local
and reset on every child. Terminal truth is exit 53 / 55 with plain
`stream-json` stderr; Swallowtail already maps those to
`native_turn_limit` and `native_budget`.

Cards 142-144 bound that subset through adapter-local types, immutable
plan/evidence, driver validation, and every structured-run, first, resumed,
and fresh-replacement child. No shared capability, portable policy field, or
Contract 029 edit landed on the worker branch. g04 stays open.

The reserved closeout names the shared delta for the orchestrator:
architecture, Contract 029, route/feature matrices, programme/front-door
indexes, changelog, the g04 generation boundary, and the sole Next Task
pointer remain untouched by this worker.

