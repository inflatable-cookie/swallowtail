# 2026-07-25 Representative Facade Shape Review

Status: complete

## Decision

The Kimi ACP, Anthropic direct, and Ollama native proofs are sufficient to
start breadth rollout. They need no new shared lifecycle or access contract.

`PreparedOperationEvidence` remains provider-neutral. It carries only common
identity, plan, access, and interface-compatibility evidence. Executable
observation, environment, endpoint policy, native inventory, artifact
identity, and residency evidence stay in adapter-local records.

## Retained Differences

- Kimi probes an installed executable and retains persistent ACP new, load,
  resume, prompt, replay, and interruption semantics.
- Anthropic preparation is pure local validation and retains catalogue plus
  one explicit hosted inference attempt.
- Ollama observes an attached endpoint and retains installed, running,
  selected-detail, and runtime-managed residency truth.

Preparation is async only when bounded observation requires it. Operation
methods keep native names and delegate to unchanged low-level roles. No generic
prompt API, provider router, route selection, fallback, or mandatory sandbox
was introduced.

## Author Pattern

The prepared-facade authoring guide now fixes:

- two-phase integration and operation preparation
- shared versus adapter-local evidence
- installed, hosted-direct, and attached-runtime preparation effects
- safe failure-stage mapping
- guaranteed versus visibly unverified-newer version handling
- low-level escape-hatch and conformance requirements

## Validation

- focused Kimi, Anthropic, and Ollama suite: 91 deterministic tests pass; two
  operator-gated live probes ignored
- full repository QA: 665 deterministic tests pass; four gated live checks
  ignored
- Doctor: unchanged at 19 pre-existing findings
- docs checks and `git diff --check`: pass
- public-API comparison: expected additive held-baseline drift in core,
  runtime, testkit, Codex, Kimi, Anthropic, and Ollama

## Next

Card 024 applies the accepted pattern to Claude Agent ACP and Gemini CLI ACP
without borrowing Kimi-specific persistence or access semantics.
