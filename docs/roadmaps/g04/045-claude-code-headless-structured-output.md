# g04.045 Claude Code Headless Structured Output

Status: planned
Owner: Tom
Created: 2026-08-23
Depends on: per-route feature completion programme; g01.039; g04.028
Vision tags: explicit selection, provider truth, structured output
Contract refs: 011, 029, 033, 037, 039, 040, 044, 052
Research: 121, 175, 192

## Problem

`claude-code.headless` is a qualified read-only Plan-mode structured run over
Claude Code stream JSON. It already binds an exact caller-selected model,
optional reasoning, a working resource, fixed `Read,Glob,Grep` tools, no
session persistence, activity, usage, cancellation, and joined cleanup. It
currently rejects `StructuredRunRequest::structured_output()` and does not
pass the official print-mode `--json-schema` flag.

Current official Claude Code documentation describes `--json-schema` as
validated structured output for print mode. Existing response-only Research
121 proves that the same flag can expose a model-visible `StructuredOutput`
tool, retry an unsatisfied schema, and still exit successfully with
`structured_output: null`. That response-only result does not settle the
distinct headless route. It does establish the evidence burden: flag presence
and JSON-shaped terminal text are insufficient.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind one bounded JSON Schema
structured-output subset on `claude-code.headless` without weakening schema,
attempt-budget, plan-mode, tool, model, usage, or terminal-result truth.

## Goals

- [ ] freeze exact `2.1.238` package, source, help, and secret-free protocol
      evidence for `--json-schema` on the selected headless command
- [ ] identify the exact accepted schema dialect and subset; reject aliases,
      unsupported keywords, and inferred draft compatibility
- [ ] classify enforcement as exactly `ProviderNative` or
      `HarnessValidated` under Contract 040
- [ ] prove the exact model-visible tool, turn, attempt, retry, null-result,
      exit-status, usage, and malformed-terminal behavior
- [ ] classify composition with stream JSON, Plan mode, `Read,Glob,Grep`,
      no session persistence, model selection, every qualified reasoning mode,
      working-resource authority, deadline, cancellation, and cleanup
- [ ] decide the exact Contract 029 facade point and behavior revision
- [ ] bind only Research 192 deliver-now schema rows through typed prepared
      input, immutable plan/evidence, request policy, driver, argv, and parser
- [ ] preserve the exact schema-absent command and result behavior
- [ ] prove deterministic acceptance and publish only exact route-local truth

## Non-Goals

- `claude-code.response-only`, `claude-agent.acp`, Anthropic APIs, or another
  Claude route
- prompt JSON emulation, post-hoc JSON claims, or consumer-side parsing as
  structured-output enforcement
- arbitrary tools, MCP, callbacks, permission widening, write access, session
  persistence, resume, fallback model, or search
- UltraCode, Fast mode, autocompact, spend caps, advisor, teams, agents, or
  other Claude Code flags
- live credentials, account inspection, paid prompts, installation, release,
  or compatibility-range widening

## Named Scope

The lane is restricted to production route `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis
`claude-code-headless-stream-json`, exact first evidence point `2.1.238`, and
the existing private behavior `claude-code.headless.stream-json.v1`.

Research 192 must freeze the exact package implementation and deterministic
secret-free specimens. The `2.1.238` identity corpus did not reprobe host help
and does not freeze `--json-schema`; current web documentation cannot amend
that exact claim by itself. Earlier qualified versions do not inherit a new
schema capability.

One model-visible `StructuredOutput` tool may coexist with the route's fixed
provider tools only if exact package evidence proves the boundary and Contract
039 still holds. Any hidden retry must have an exact preflight-bound attempt
budget. `--max-turns` is not assumed to bound schema attempts. A zero exit with
missing or null structured output is failure, not an ordinary successful
structured result.

An empty Research 192 deliver-now set is an honest stop. The worker must not
weaken Contracts 039 or 040 to make the flag fit.

## Execution Plan

### Batch 45.1 — Exact Package And Structured-Result Evidence

- [ ] Execute card 124.
- [ ] freeze current official docs, exact `2.1.238` package source/help, and
      deterministic valid, invalid, unsatisfiable, malformed, and absent-schema
      specimens
- [ ] promote Research 192 with schema, enforcement, retry, result, version,
      and compatibility dispositions

### Batch 45.2 — Conditional Prepared Binding

- [ ] Execute card 125 only if card 124 admits a non-empty deliver-now set.
- [ ] bind only exact admitted schemas through the owning prepared route and
      low-level driver
- [ ] preserve schema absence exactly and reject plan/evidence/driver drift
      before process work

### Batch 45.3 — Route-Local Acceptance

- [ ] Execute card 126 only after card 125.
- [ ] prove exact dispatch, result, failure, usage, cancellation, cleanup, and
      absent-path behavior
- [ ] update route-local guidance and report the deferred shared closeout delta

## Acceptance Criteria

- [ ] only Research 192 deliver-now schema rows prepare
- [ ] descriptor, dialect, enforcement source, attempt budget, plan,
      evidence, policy, driver, argv, and terminal parser agree exactly
- [ ] schema absence preserves the current command and ordinary text result
- [ ] no prompt emulation, raw provider options, unbounded retry, null success,
      or inferred schema draft enters the mapping
- [ ] existing Plan-mode tools, read-only working resource, model, reasoning,
      activity, usage, cancellation, and cleanup truth remain intact
- [ ] every knowable mismatch fails before process work; terminal and provider
      failures remain explicit after dispatch
- [ ] default QA uses no credential, account, install, provider prompt, or paid
      operation
- [ ] docs distinguish requested, planned, dispatched, accepted, effective,
      observed, and returned structured-output truth

## Lane Runway

- predecessor: g04.044 OpenAI background reasoning-vocabulary correction
- this milestone: Claude Code headless JSON Schema evidence and conditional
  route-local binding
- execution topology: one serial worker lane, cards 124-126
- next route family: selected by the orchestrator after evidence, review, and
  merge closeout; no later family is precompiled here

## Decision Gates

- Stop if exact `2.1.238` does not expose and apply `--json-schema` on the
  selected stream-JSON Plan-mode command.
- Stop if the accepted dialect/subset or terminal structured-result shape
  cannot be stated exactly.
- Stop if enforcement is prompt emulation or cannot be classified under
  Contract 040.
- Stop if retries or attempts are non-zero without an exact preflight-bound
  maximum, or if a null/missing structured result can exit as success.
- Stop if schema composition weakens model, reasoning, tool, filesystem,
  activity, usage, cancellation, deadline, process, or cleanup truth.
- Stop if delivery needs a contract change, live provider proof, generic
  settings, a compatibility-range assumption, or a breaking public API.

## Batch Cards

- [124-claude-code-headless-structured-output-evidence.md](batch-cards/124-claude-code-headless-structured-output-evidence.md) — planned
- [125-claude-code-headless-structured-output-binding.md](batch-cards/125-claude-code-headless-structured-output-binding.md) — conditional
- [126-claude-code-headless-structured-output-acceptance.md](batch-cards/126-claude-code-headless-structured-output-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 121 Claude Code Response-Only Structured Route](../../research/121-claude-code-response-only-structured-route.md)
- [Research 175 Claude Code 2.1.238 Identity](../../research/175-claude-code-2-1-238-identity.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 039 Bounded Single-Turn Structured Run](../../contracts/039-bounded-single-turn-structured-run-projection.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Claude Agent Prepared Integration](../../guides/claude-agent-prepared-integration.md)
- [Claude Code CLI Reference](https://code.claude.com/docs/en/cli-reference)
- [Claude Code Headless Mode](https://code.claude.com/docs/en/headless)
