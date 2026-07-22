# 086 Kimi Platform Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../028-kimi-platform-k3-direct-inference-proof.md`

## Objective

Run the production Kimi Platform driver through hosted-direct conformance,
prove topology and cleanup, and close roadmap 028.

## Governing References

- Research 018
- Contracts 011, 014, 020, and 024
- roadmap 028
- cards 084-085 evidence

## Scope

- local and remote-authoritative execution-host identities
- catalogue bounds, successful reasoning and output, terminal usage,
  returned-model mismatch, provider error, unknown event, disconnect,
  cancellation, deadline, redaction, one attempt, and joined cleanup
- common codec cross-corpus regression with llama.cpp
- live catalogue and authenticated inference probes remain separately gated
- architecture, front-door, roadmap, log, and continuation closeout

## Acceptance Criteria

- [x] the hosted-direct profile passes without provider-specific branches
- [x] both topologies preserve exact endpoint, access, route, model, and host
      identity
- [x] one operation produces no retry, reconnect, fallback, or detached work
- [x] catalogue discovery cannot imply K3 entitlement or invocation readiness
- [x] existing llama.cpp behavior and all ten conformance profiles remain
      unchanged
- [x] one next provider-breadth checkpoint remains after closeout

## Validation

- focused Kimi Platform, protocol, llama.cpp, and testkit tests
- focused warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- provider semantics leak into the common codec or conformance profile
- remote-authoritative topology changes access or cleanup semantics
- authenticated evidence contradicts the frozen platform or model boundary

## Auto-Continuation

No. Return to the DeepSeek, Z.AI, and Alibaba provider-breadth checkpoint.

## Completion Evidence

- the unchanged hosted-direct profile passes for the production Kimi Platform
  driver without Kimi-specific testkit branches
- local and remote-authoritative fixtures preserve exact execution host,
  configured instance, endpoint target, public-platform API-key audience,
  catalogue source, K3 route, model, and provider identity
- the exact success transcript contains one inference attempt, two HTTP
  requests, terminal reasoning, output, and usage, and no retry, reconnect,
  fallback, or detached task
- connection work completes before both credential releases; operation close
  leaves no owned work
- catalogue output remains source-scoped observation and grants no entitlement,
  route, or invocation-readiness claim
- 93 focused Kimi Platform, compatible-chat, llama.cpp, and testkit tests pass;
  focused warnings-denied clippy passes
- full repository QA passes with 384 tests; three installed or live probes
  remain separately gated and ignored by default
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  seven errors, with no new finding from this batch
- roadmap 029 and cards 087-089 return the active lane to current DeepSeek,
  Z.AI, and Alibaba Model Studio evidence before another provider is selected
