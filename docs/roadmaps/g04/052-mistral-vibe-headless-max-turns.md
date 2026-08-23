# g04.052 Mistral Vibe Headless Maximum Turns

Status: ready
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Depends on: per-route feature completion programme; Research 150
Vision tags: explicit selection, bounded execution, route-local controls
Contract refs: 008, 011, 029, 033, 037, 040, 052
Research: 150; 199 reserved by card 145

## Problem

Production route `mistral-vibe.headless` fixes `--max-turns 8` on every
structured run. Callers cannot select a smaller conversation envelope even
when one or two assistant turns are enough.

Current official documentation and exact Mistral Vibe `2.24.2` source expose a
native `--max-turns` limit. Exact source also accepts values broader than a
useful Swallowtail API: zero stops before the first assistant turn, negative
values appear parser-valid, and omitting the flag is unbounded. Those states
must not become public behavior by inference.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a typed caller-decreasing
positive maximum-turn selection for exact Mistral Vibe `2.24.2`. Omission
preserves the current `--max-turns 8` command.

## Goals

- [x] freeze current official and exact `2.24.2` parsing, counting,
      enforcement, terminal, and cleanup evidence
- [x] classify candidate values `1..=8`, zero, negative, fractional, values
      above eight, and upstream unbounded omission
- [x] settle the exact assistant-turn definition and pre-turn enforcement
- [x] distinguish native limit truth from host deadline, output-token, tool,
      cost, provider-completion, and quality claims
- [x] promote Research 199 with an exact deliver-now table or honest stop
- [x] preserve caller omission as exact current `--max-turns 8` argv and
      behavior
- [x] bind only admitted values through typed adapter-local input, immutable
      plan/evidence, driver validation, and command construction
- [x] reject invalid or mismatched values before process start or prompt
- [x] publish deterministic dispatch and terminal truth without claiming
      provider acceptance, effective work, latency, cost, or billing

## Non-Goals

- zero, negative, fractional, raised, or unbounded public selections unless
  card 145 proves a useful exact subset and the roadmap is explicitly revised
- a portable output-token limit or shared generic execution-budget control
- `--max-price`, `--max-tokens`, tool budgets, agent selection, approval,
  trust, prompt, output mode, working-resource, deadline, or cancellation changes
- Vibe ACP, TUI, continue, resume, teleport, setup, model selection, or auth
- earlier or later Vibe releases, currentness work, or sibling routes
- live login, prompt, provider request, paid work, release, publication, or merge

## Named Scope

The lane is restricted to route `mistral-vibe.headless`, driver
`swallowtail.mistral-vibe.headless`, axis `mistral-vibe.release`, exact release
`2.24.2`, and behavior `mistral-vibe.headless.stdio-streaming-v1` unless card
145 proves a feature-local revision is required.

The candidate Swallowtail domain is positive caller-decreasing maximum turns
`1..=8`. This is an evidence candidate, not a prequalified public range. A
caller that omits the new selection must still dispatch `--max-turns 8`;
upstream omission of the flag remains forbidden because it is unbounded.

Card 145 must distinguish argparse acceptance from useful route truth. It must
freeze the exact counter definition, increment and check points, one-child
lifetime, process exit, stderr, stream records, partial public events, portable
terminal classification, cancellation, deadline, and joined cleanup. It must
also decide whether selected values need a feature-local behavior or evidence
revision while leaving the exact Contract 029 release claim unchanged.

The control remains adapter-local. Contract 040 explicitly excludes turn
limits from portable `OutputTokenLimit`; no shared capability or generic
provider-options map is planned. The existing streaming output, plan agent,
trust flag, workdir, read-only posture, local unauthenticated access, required
host deadline, failure mapping, cancellation, and cleanup remain fixed. An
empty Research 199 deliver-now set is an honest stop.

## Execution Plan

### Batch 52.1 — Exact Maximum-Turn Evidence

- [x] Execute card 145.
- [x] freeze official and exact-release parsing, counting, enforcement, and
      terminal evidence
- [x] promote Research 199 with domain and terminal dispositions

### Batch 52.2 — Conditional Maximum-Turn Binding

- [x] Execute card 146 only when card 145 admits a non-empty deliver-now set.
- [x] add the smallest typed adapter-local caller-decreasing selection
- [x] preserve omission and every fixed route boundary
- [x] bind admitted values through preparation, plan/evidence, driver, and argv

### Batch 52.3 — Route-Local Acceptance

- [x] Execute card 147 only after card 146.
- [x] prove admitted, omitted, rejected, terminal, and lifecycle truth
- [x] update route-local guidance and reserve the shared closeout delta

## Acceptance Criteria

- [x] only Research 199 deliver-now values prepare
- [x] omission preserves exact current `--max-turns 8` argv and behavior
- [x] input, plan/evidence, driver, and command agree exactly
- [x] invalid values and knowable mismatches reject before effects
- [x] streaming, agent, trust, workdir, deadline, access, cancellation,
      failure, and cleanup remain unchanged
- [x] terminal claims do not exceed exact source and deterministic fixture truth
- [x] no portable output-token, generic budget, effective-work, quality,
      latency, price, or billing claim is introduced
- [x] default QA performs no install, login, credential, catalogue, prompt, or
      paid work
- [x] g04.052 closes only this route-local family; g04 remains active until the
      operator directs otherwise

## Lane Runway

- predecessor: g04.051 Qwen headless turn and tool budgets
- this milestone: Mistral Vibe headless caller-decreasing maximum-turn evidence
  and conditional adapter-local binding
- execution topology: one serial worker lane, cards 145-147
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact counting, enforcement, terminal, or partial-event truth needs a
  live provider prompt.
- Stop if a positive selection cannot remain exact across input, immutable
  plan/evidence, driver, and command.
- Stop if a native limit would be misrepresented as successful completion,
  effective work, or a Contract 040 output-token limit.
- Stop if delivery needs a shared capability, contract change, currentness
  change, sibling route, or breaking public API.
- Stop if output mode, plan agent, trust, workdir, deadline, access,
  cancellation, failure, or cleanup changes.

## Batch Cards

- [145-mistral-vibe-headless-max-turns-evidence.md](batch-cards/145-mistral-vibe-headless-max-turns-evidence.md) — ready
- [146-mistral-vibe-headless-max-turns-binding.md](batch-cards/146-mistral-vibe-headless-max-turns-binding.md) — conditional
- [147-mistral-vibe-headless-max-turns-acceptance.md](batch-cards/147-mistral-vibe-headless-max-turns-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 150 Mistral Vibe Headless 2.24.2 Identity](../../research/150-mistral-vibe-headless-2-24-2-identity.md)
- [Research 199 Mistral Vibe Headless Maximum-Turn Evidence](../../research/199-mistral-vibe-headless-max-turns-evidence.md)
- [Contract 008 Runtime Registration And Preflight](../../contracts/008-runtime-registration-and-preflight.md)
- [Contract 011 Runtime Conformance Profiles](../../contracts/011-runtime-conformance-profiles.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 052 Consumer And Operator Integration Documentation](../../contracts/052-consumer-and-operator-integration-documentation.md)
- [Mistral Vibe Headless Prepared Integration](../../guides/mistral-vibe-headless-prepared-integration.md)
- [Mistral Vibe README](https://github.com/mistralai/mistral-vibe/blob/v2.24.2/README.md)
- [Mistral Vibe v2.24.2](https://github.com/mistralai/mistral-vibe/tree/v2.24.2)

## Closeout

Reserved. The worker closes only route-local Research 199, cards 145-147,
guide, fixtures, example/API evidence, and this milestone. Shared architecture,
Contract 029 disposition, route/feature matrices, programme/front-door
indexes, changelog, sole Next Task, and merge truth remain for the orchestrator
after review and merge. g04 closure is not authorized.
