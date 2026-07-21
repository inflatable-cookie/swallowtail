# 086 Kimi Platform Conformance And Closeout

Status: planned
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

- [ ] the hosted-direct profile passes without provider-specific branches
- [ ] both topologies preserve exact endpoint, access, route, model, and host
      identity
- [ ] one operation produces no retry, reconnect, fallback, or detached work
- [ ] catalogue discovery cannot imply K3 entitlement or invocation readiness
- [ ] existing llama.cpp behavior and all ten conformance profiles remain
      unchanged
- [ ] one next provider-breadth checkpoint remains after closeout

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
