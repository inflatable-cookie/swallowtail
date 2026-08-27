# g04.079 Claude Code Headless Maximum Turns

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Depends on: g04.055; g04.065 closeout; g04.078 closeout; per-route feature completion programme
Vision tags: explicit behavior, bounded harness work, route-local controls
Contract refs: 010, 011, 029, 037, 039, 040, 041, 052
Research: 121, 202, 212, 226

## Problem

Production route `claude-code.headless` owns one read-only Plan-mode Claude
Code child and one structured run, but it does not expose the native
`--max-turns` limit. Current official documentation describes the flag as a
positive print-mode maximum over agentic tool-use turns, with
`error_max_turns` when the limit is reached. It also names
`CLAUDE_CODE_MAX_TURNS` as an ambient equivalent that explicit argv overrides.

That current documentation cannot backport behavior onto the exact qualified
`2.1.220..=2.1.241` window. The frozen `2.1.241` help specimen does not claim
the flag, and Claude Code documentation warns that help is not exhaustive.
Parser presence, numeric acceptance, native enforcement, stream terminal
shape, environment precedence, and support membership therefore need exact
artifact evidence before any binding.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a caller-decreasing maximum
agentic-turn selection on `claude-code.headless`. Keep it Claude Code-local,
preserve omission as the current no-flag command and approved environment, and
do not relabel turns as output tokens, tool calls, cost, wall time, or portable
generation control.

## Goals

- [x] freeze exact parser, support range, numeric domain, aliases, repetition,
      precedence, diagnostics, and exit behavior for `--max-turns`
- [x] freeze `CLAUDE_CODE_MAX_TURNS` interaction and prove explicit argv wins
      without inspecting or rewriting the approved environment
- [x] freeze the exact definition of one counted turn and the limit-reached
      stream/result/usage/exit shape
- [x] promote Research 226 with an exact deliver-now table or honest empty set
- [x] conditionally bind only positive, exactly enforced values through a
      closed adapter-local type and immutable prepared/driver evidence
- [x] preserve the current no-flag command, selected model and reasoning,
      Plan/read-only posture, tool set, provider configuration, deadline,
      activity, terminal, retention, and joined cleanup
- [x] keep response-only, ACP, output-token, cost, tool-call, and wall-time
      limits unchanged

## Non-Goals

- a provider-neutral maximum-turns, agent-budget, cost, or generation-control
  contract
- `--max-budget-usd`, `--autocompact`, Fast mode, Ultracode, structured output,
  advisor, agents, teams, fallback, writable permission modes, or tool changes
- changing `claude-code.response-only` or `claude-agent.acp`
- inspecting, clearing, or rewriting the operator-approved environment
- live provider prompts, login/account work, paid work, currentness, release,
  merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `claude-code.headless`, driver
`swallowtail.claude-code.headless`, compatibility axis
`claude-code.headless-stream-json`, qualified window `2.1.220..=2.1.241`, and
private behavior `claude-code.headless.stream-json.v1` unless exact evidence
requires a route-private revision.

Card 219 must freeze exact official packages and native artifacts rather than
rely on mutable current documentation. It must identify the first exact point
that accepts and enforces `--max-turns`, then classify every qualified point as
deliver now, evidence-gated, intentionally withheld, or not applicable. A
range may be admitted only when exact change boundaries and artifact evidence
justify it; otherwise admit named exact points only.

Research 226 must distinguish parser acceptance from actual loop enforcement.
It must trace positive values through CLI parsing, options, the agent loop,
tool-use round trips, result subtype, `num_turns`, usage, stderr, exit status,
and stream ordering. Zero, negative, signed, padded, fractional, exponential,
empty, missing, repeated, overflow, and trailing-junk inputs need exact
dispositions. Current official documentation and the exact implementation must
also settle CLI versus `CLAUDE_CODE_MAX_TURNS` precedence.

Any public control stays adapter-local and closed. Omission emits no
`--max-turns` and preserves the approved environment exactly; no claim of
unlimited execution follows because ambient `CLAUDE_CODE_MAX_TURNS` may remain
present. Explicit selection is eligible only if exact evidence proves argv
overrides that ambient value and the native loop enforces the selected bound.

## Execution Plan

### Batch 79.1 — Exact Maximum-Turn Evidence

- [x] Execute card 219.
- [x] freeze exact parser, precedence, counted-turn, enforcement, and terminal
      truth
- [x] promote Research 226 with a non-empty exact table: one closed positive
      selection across the whole published window, plus unchanged omission

### Batch 79.2 — Conditional Adapter-Local Binding

- [x] Execute card 220. Research 226 admitted a non-empty exact set.
- [x] bind only admitted values through typed preparation and canonical argv

### Batch 79.3 — Route-Local Acceptance

- [x] Execute card 221 after card 220.
- [x] prove dispatch, omission, rejection, native terminal truth, and unchanged
      structured-run lifecycle

## Acceptance Criteria

- [x] only Research 226 deliver-now rows prepare a maximum-turn selection
- [x] every admitted value is positive, closed, Claude Code-local, immutable,
      and exactly dispatched
- [x] omission retains the exact current argv with no `--max-turns` and does
      not mutate or overclaim ambient environment behavior
- [x] docs separate requested, dispatched, parser-accepted, enforced, reached,
      and observed turn-limit state
- [x] model/reasoning selection, Plan/read-only authority, fixed tools,
      configuration, provider state, activity, deadline, terminal mapping,
      retention, and cleanup do not widen
- [x] default QA performs no provider prompt, login/account work, paid work,
      ambient configuration mutation, installation, or update

## Lane Runway

- predecessor: g04.078 llama.cpp owned reasoning delivery
- this milestone: complete; evidence admitted a non-empty set, so binding and
  acceptance both ran
- execution topology: one serial worker lane, cards 219-221
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

No gate fired.

- Exact qualified artifacts proved flag support and loop enforcement with no
  live provider work: every published `2.1.220..=2.1.241` version carries the
  hidden option declaration, the argv-precedence resolver, the loop guard, the
  `error_max_turns` result, and the nonzero-exit expression.
- The numeric domain, counted-turn definition, environment precedence,
  limit-reached result, and exit behavior are all exact. The parser's domain is
  wider than the documented one, so the adapter closes it to positive integers
  rather than trusting the native check.
- A positive selected value cannot be ignored, clamped, replaced, or shadowed.
  Only a resolved `0` is inert, and the closed type makes it unselectable.
- Delivery needed no environment inspection or mutation, no portable budget
  vocabulary, no sibling-route change, no currentness movement, and no breaking
  API: the surface is additive and adapter-local.

## Batch Cards

- [219-claude-code-headless-maximum-turns-evidence.md](batch-cards/219-claude-code-headless-maximum-turns-evidence.md)
- [220-claude-code-headless-maximum-turns-binding.md](batch-cards/220-claude-code-headless-maximum-turns-binding.md)
- [221-claude-code-headless-maximum-turns-acceptance.md](batch-cards/221-claude-code-headless-maximum-turns-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 202 Claude Code 2.1.241 Identity](../../research/202-claude-code-2-1-241-identity.md)
- [Research 212 Claude Code Headless Ultracode](../../research/212-claude-code-headless-ultracode-evidence.md)
- [Research 226 Claude Code Headless Maximum Turns](../../research/226-claude-code-headless-maximum-turns-evidence.md)
- [Claude Code CLI Reference](https://code.claude.com/docs/en/cli-reference)
- [Claude Agent Loop](https://code.claude.com/docs/en/agent-sdk/agent-loop)
- [Claude Code Environment Variables](https://code.claude.com/docs/en/env-vars)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 039 Bounded Structured Run](../../contracts/039-bounded-single-turn-structured-run-projection.md)
- [Contract 040 Generation Control](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 041 Input And Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Claude Agent Prepared Integration](../../guides/claude-agent-prepared-integration.md)
