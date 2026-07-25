# 023 Representative Facade Shape Review

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../008-representative-cross-shape-facades.md`

## Objective

Review the Kimi, Anthropic, and Ollama proofs before broad adapter rollout.

## Governing Refs

- Contract 037
- cards 018-022
- public API compatibility evidence

## Scope

1. Compare preparation inputs, evidence, typed operations, failures, and
   low-level escape hatches across all three routes.
2. Remove accidental provider-specific concepts from shared records.
3. Record intentional differences rather than normalizing them.
4. Classify public API changes and update facade author guidance.
5. Stop if a missing durable lifecycle or access rule appears.

## Acceptance Criteria

- [x] shared records express only common identity and evidence
- [x] each route retains its native operation semantics
- [x] preparation failure stages are safe and comparable
- [x] adapter authors have one clear construction and conformance pattern
- [x] breadth rollout needs no unresolved contract decision

## Validation

- three-adapter focused suite
- public API diff
- docs and examples
- `effigy doctor` delta review
- `git diff --check`

## Evidence

- `PreparedOperationEvidence` contains only driver, role, layer, shape,
  instance, revision, host, opaque target, facade, access, plan, and interface
  compatibility evidence. Kimi environment and executable state, Anthropic
  endpoint policy, and Ollama runtime inventory remain adapter-local.
- Kimi retains persistent ACP new/load/resume and turn semantics. Anthropic
  retains catalogue plus one explicit Messages attempt. Ollama retains
  inventory scopes plus one native attempt and runtime-managed residency.
- Preparation effects differ deliberately: Kimi probes one executable,
  Anthropic validates pure local bindings, and Ollama observes one approved
  attached endpoint. All use the same safe failure stages where applicable.
- Every prepared operation delegates to its unchanged low-level role and
  exposes plan, request, evidence, and low-level escape hatch. No provider
  router, generic prompt method, or fallback appeared.
- The prepared-facade authoring guide records the two-phase construction
  pattern, shared-record boundary, effect shapes, failure mapping, version
  rules, and conformance checklist.
- The representative review found no missing durable lifecycle or access rule.
  Contract 037 is sufficient for breadth rollout.
- The focused Kimi, Anthropic, and Ollama suite passes 91 deterministic tests
  with two operator-gated live probes ignored.
- Full repository QA passes with 665 deterministic tests and four gated live
  checks ignored. Doctor remains at 19 pre-existing findings.
- Public-API comparison reports expected additive core, runtime, testkit,
  Codex, Kimi, Anthropic, and Ollama drift. The held `0.1.0` baseline remains
  unchanged for card 036.

## Auto-Continuation

No. g02.008 is closed. Card 024 is the sole next task.
