# g04.053 Qoder Headless Maximum Turns

Status: complete; evidence stop and claim correction
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Depends on: per-route feature completion programme; Research 151
Vision tags: explicit selection, bounded execution, route-local controls
Contract refs: 008, 011, 029, 033, 037, 040, 052
Research: 151; 200 promoted; empty deliver-now; claim correction applied

## Problem

Production route `qoder.headless` fixes `--max-turns 8` on every structured
run. Callers cannot select a smaller conversation envelope even when fewer
assistant turns are enough.

Research 200 proves the selected CLI headless QueryEngine factory hardcodes
AgentLoop `maxTurns: kN` (`1000`) and that CLI `--max-turns` only populates
Config `maxSessionTurns` for the text error formatter. Caller-decreasing
binding is not exact. Operator disposition retains historical inert argv `8`,
corrects corpus claims to factory ceiling `1000`, and narrows `error_max_turns`
to decoder-only without removing the flag or adding a caller feature.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a typed caller-decreasing
positive maximum-turn selection for exact Qoder `1.1.25`. Exact evidence does
not permit that binding. The lane closes as an evidence stop plus claim
correction.

## Goals

- [x] freeze current official and exact `1.1.25` parsing, counting,
      enforcement, result, exit, and cleanup evidence
- [x] classify candidate values and route-omit dispositions against exact
      AgentLoop wiring
- [x] settle the exact turn definition, check/increment order, one-child
      lifetime, and `num_turns` relationship
- [x] distinguish native limit truth from host deadline, output-token, tool,
      cost, provider-completion, and quality claims
- [x] promote Research 200 with an exact deliver-now table or honest stop
- [x] reconcile contradicted claims while preserving historical inert argv `8`
- [ ] bind only admitted values through typed adapter-local input, immutable
      plan/evidence, driver validation, and command construction
- [ ] reject invalid or mismatched values before process start or prompt
- [ ] publish deterministic dispatch and terminal truth without live work

## Non-Goals

- caller-selectable maximum turns on this route
- removing historical inert `--max-turns 8`
- zero, negative, fractional, raised, or unbounded public selections
- a portable output-token limit or shared generic execution-budget control
- permission modes, yolo, safe mode, tools, model, prompt, output format,
  session persistence, workdir, deadline, cancellation, or auth changes
- Qoder ACP, SDK stdio, TUI, continuation, resume, teleport, IDE, or login
- another Qoder release, currentness work, sibling routes, live provider work,
  release, publication, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `qoder.headless`, driver
`swallowtail.qoder.headless`, axis `qoder.package`, exact package `1.1.25`, and
the selected stream-json print behavior. Research 200 withholds caller
`1..=8` because CLI values are not wired into AgentLoop on the selected
factory. Route argv still always emits historical inert `--max-turns 8`.

## Execution Plan

### Batch 53.1 — Exact Maximum-Turn Evidence

- [x] Execute card 148 (evidence + claim correction).
- [x] promote Research 200 with exact domain, counting, terminal truth, and
      reconciled claim surfaces

### Batch 53.2 — Conditional Maximum-Turn Binding

- [ ] Execute card 149 only when card 148 admits a non-empty deliver-now set.
- [ ] bind the smallest typed adapter-local caller-decreasing selection

Card 149 is blocked: empty deliver-now; operator forbids a caller feature.

### Batch 53.3 — Route-Local Acceptance

- [ ] Execute card 150 only after card 149.
- [ ] prove admitted, omitted, rejected, terminal, and lifecycle truth

Card 150 is blocked because card 149 did not execute.

## Acceptance Criteria

- [x] only Research 200 deliver-now values prepare; the set is empty
- [x] historical inert argv `--max-turns 8` is retained and correctly claimed
- [ ] input, plan/evidence, driver, and command agree for a caller feature
- [ ] invalid values and knowable mismatches reject before effects
- [x] fixed output, permission, session, workdir, deadline, access,
      cancellation, failure, and cleanup boundaries remain unchanged
- [x] terminal claims distinguish decoder fixture truth from argv enforcement
- [x] no portable output-token, generic budget, effective-work, quality,
      latency, price, or billing claim is introduced
- [x] default QA performs no install, login, credential, prompt, or paid work
- [x] g04.053 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.052 Mistral Vibe headless maximum turns
- this milestone: Qoder headless maximum-turn evidence and claim correction
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

## Batch Cards

- [148-qoder-headless-max-turns-evidence.md](batch-cards/148-qoder-headless-max-turns-evidence.md) — complete; evidence stop and claim correction
- [149-qoder-headless-max-turns-binding.md](batch-cards/149-qoder-headless-max-turns-binding.md) — blocked
- [150-qoder-headless-max-turns-acceptance.md](batch-cards/150-qoder-headless-max-turns-acceptance.md) — blocked

## Evidence Stop

Research 200 has no deliver-now caller-selection row. Exact npm
`@qoder-ai/qodercli@1.1.25` stores CLI `--max-turns` as Config
`maxSessionTurns`. Selected stream-json print constructs QueryEngine with
`maxTurns: kN` (`1000`). Historical inert argv `8` is retained. Synthetic
`error_max_turns` fixtures are decoder-only.

Cards 149 and 150 are blocked. Runtime argv bytes unchanged. Claim surfaces
reconciled under operator direction.

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

Route-local Research 200, cards 148-150, claim correction, and this milestone
close the family. Shared Next Task after merge remains for the orchestrator.
g04 closure is not authorized.
