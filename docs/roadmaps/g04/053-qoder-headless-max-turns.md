# g04.053 Qoder Headless Maximum Turns

Status: stopped after evidence
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Depends on: per-route feature completion programme; Research 151
Vision tags: explicit selection, bounded execution, route-local controls
Contract refs: 008, 011, 029, 033, 037, 040, 052
Research: 151; 200 promoted; empty deliver-now

## Problem

Production route `qoder.headless` fixes `--max-turns 8` on every structured
run. Callers cannot select a smaller conversation envelope even when fewer
assistant turns are enough.

Research 151 proves that exact Qoder CLI `1.1.25` requires a positive native
bound, treats flag omission as unbounded, and emits a distinct
`error_max_turns` result. It does not yet prove the exact counter semantics,
off-by-one behavior, parser breadth, or smallest useful public domain. Those
facts must remain evidence gates.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a typed caller-decreasing
positive maximum-turn selection for exact Qoder `1.1.25`. Omission preserves
the current `--max-turns 8` command.

## Goals

- [x] freeze current official and exact `1.1.25` parsing, counting,
      enforcement, result, exit, and cleanup evidence
- [x] classify candidate values `1..=8`, zero, negative, fractional, values
      above eight, overflow, and upstream unbounded omission
- [x] settle the exact turn definition, check/increment order, one-child
      lifetime, and `num_turns` relationship
- [x] distinguish native limit truth from host deadline, output-token, tool,
      cost, provider-completion, and quality claims
- [x] promote Research 200 with an exact deliver-now table or honest stop
- [ ] preserve caller omission as exact current `--max-turns 8` argv
- [ ] bind only admitted values through typed adapter-local input, immutable
      plan/evidence, driver validation, and command construction
- [ ] reject invalid or mismatched values before process start or prompt
- [ ] publish deterministic dispatch and terminal truth without live work

## Non-Goals

- zero, negative, fractional, raised, or unbounded public selections unless
  card 148 proves and explicitly records them
- a portable output-token limit or shared generic execution-budget control
- permission modes, yolo, safe mode, tools, model, prompt, output format,
  session persistence, workdir, deadline, cancellation, or auth changes
- Qoder ACP, SDK stdio, TUI, continuation, resume, teleport, IDE, or login
- another Qoder release, currentness work, sibling routes, live provider work,
  release, publication, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `qoder.headless`, driver
`swallowtail.qoder.headless`, axis `qoder.package`, exact package `1.1.25`, and
the selected stream-json print behavior. Candidate caller values are `1..=8`.
Omission must still dispatch `--max-turns 8`; upstream flag omission remains
forbidden because it is unbounded.

Card 148 must distinguish parser acceptance from useful route truth. It owns
the counter source, increment/check order, off-by-one behavior, child-local
lifetime, process exit, result envelope, `num_turns`, partial public events,
portable terminal classification, cancellation, deadline, and joined cleanup.
An empty Research 200 deliver-now set is an honest stop.

The control remains adapter-local. Contract 040 excludes turn limits from
portable `OutputTokenLimit`; no shared capability or generic provider-options
map is planned. `dont_ask`, stream-json output, no-session-persistence,
working-resource binding, local config access, required host deadline,
cancellation, failure mapping, and cleanup remain fixed.

## Execution Plan

### Batch 53.1 — Exact Maximum-Turn Evidence

- [x] Execute card 148.
- [x] promote Research 200 with exact domain, counting, and terminal truth

Card 148 stopped the lane. Exact `1.1.25` copies CLI `--max-turns` onto Config
`maxSessionTurns`, but the print QueryEngine path hardcodes AgentLoop
`maxTurns: kN` (`1000`). Caller-decreasing selection is not exact dispatch.

### Batch 53.2 — Conditional Maximum-Turn Binding

- [ ] Execute card 149 only when card 148 admits a non-empty deliver-now set.
- [ ] bind the smallest typed adapter-local caller-decreasing selection

Card 149 is blocked because Research 200 admits no deliver-now row.

### Batch 53.3 — Route-Local Acceptance

- [ ] Execute card 150 only after card 149.
- [ ] prove admitted, omitted, rejected, terminal, and lifecycle truth

Card 150 is blocked because card 149 did not execute. The guide stays accurate
without a caller maximum-turn capability claim.

## Acceptance Criteria

- [x] only Research 200 deliver-now values prepare; the set is empty
- [x] omission preserves exact current `--max-turns 8` argv and behavior
- [ ] input, plan/evidence, driver, and command agree exactly
- [ ] invalid values and knowable mismatches reject before effects
- [x] fixed output, permission, session, workdir, deadline, access,
      cancellation, failure, and cleanup boundaries remain unchanged
- [x] terminal claims do not exceed exact source and deterministic fixtures
- [x] no portable output-token, generic budget, effective-work, quality,
      latency, price, or billing claim is introduced
- [x] default QA performs no install, login, credential, prompt, or paid work
- [x] g04.053 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.052 Mistral Vibe headless maximum turns
- this milestone: Qoder headless caller-decreasing maximum-turn evidence and
  conditional adapter-local binding
- execution topology: one serial worker lane, cards 148-150
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact counting, enforcement, result, exit, or partial-event truth
  needs a live provider prompt.
- Stop if the selected bound cannot remain exact across input, immutable
  plan/evidence, driver, and command.
- Stop if native limit would be misrepresented as success, effective work, or
  a Contract 040 output-token limit.
- Stop if delivery needs a shared capability, contract/currentness change,
  sibling route, or breaking public API.
- Stop if any fixed permission, output, session, workdir, deadline, access,
  cancellation, failure, or cleanup boundary changes.

## Batch Cards

- [148-qoder-headless-max-turns-evidence.md](batch-cards/148-qoder-headless-max-turns-evidence.md) — complete; evidence stop
- [149-qoder-headless-max-turns-binding.md](batch-cards/149-qoder-headless-max-turns-binding.md) — blocked
- [150-qoder-headless-max-turns-acceptance.md](batch-cards/150-qoder-headless-max-turns-acceptance.md) — blocked

## Evidence Stop

Research 200 has no deliver-now row. Exact npm `@qoder-ai/qodercli@1.1.25`
registers `--max-turns <count>` and stores it as Config `maxSessionTurns`. The
selected stream-json print path constructs QueryEngine with `maxTurns: kN`
(`1000`) and never feeds `getMaxSessionTurns()` into AgentLoop. Binding
caller-decreasing `1..=8` would overstate enforcement. Existing fixed argv
`8`, `error_max_turns` fixture mapping, and joined one-child lifecycle remain
the claim.

Cards 149 and 150 are blocked. No production adapter, prepared input, driver,
guide capability, behavior revision, compatibility range, matrix, or release
claim changes.

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 151 Qoder Headless 1.1.25 Identity](../../research/151-qoder-headless-1-1-25-identity.md)
- [Research 200 Qoder Headless Maximum-Turn Evidence](../../research/200-qoder-headless-max-turns-evidence.md)
- [Contract 008 Runtime Registration And Preflight](../../contracts/008-runtime-registration-and-preflight.md)
- [Contract 011 Runtime Conformance Profiles](../../contracts/011-runtime-conformance-profiles.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 052 Consumer And Operator Integration Documentation](../../contracts/052-consumer-and-operator-integration-documentation.md)
- [Qoder Headless Prepared Integration](../../guides/qoder-headless-prepared-integration.md)

## Closeout

Route-local Research 200, cards 148-150, and this milestone record the evidence
stop. Shared architecture, Contract 029 disposition, route/feature matrices,
programme/front-door indexes, changelog, sole Next Task, and merge truth remain
for the orchestrator after review and merge. g04 closure is not authorized.
