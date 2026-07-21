# 035 Provider Expansion Evidence And Contract

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../011-hosted-transport-foundations.md`

## Objective

Repair stale lane state, refresh authoritative provider evidence, select the
first cross-adapter tranche, and promote its missing shared contract.

## Governing References

- Research 003 and 004
- Contracts 005-011 and 014
- `docs/roadmaps/long-term-plan.md`

## Scope

- roadmap and front-door currentness repair
- official and maintained-source comparison
- first harness, direct API, ACP, and self-hosted recommendation
- hosted transport and evidence contract
- g01 roadmap compilation

## Out Of Scope

- provider implementation
- live authentication
- consumer repository edits

## Acceptance Criteria

- [x] cards 032-034 and roadmap 010 agree on completion
- [x] front doors name provider expansion as active
- [x] current evidence favors transport diversity over provider count
- [x] Contract 014 closes the shared planning gap
- [x] roadmaps 011-015 remain in g01

## Evidence

- OpenCode server documentation and installed `1.14.48` help expose a default-
  port mismatch; explicit endpoint binding is required.
- Anthropic official docs confirm public-API auth, paginated models, SSE,
  mid-stream errors, usage, request ids, and rate evidence.
- Research 004 records OpenCode HTTP/SSE plus Anthropic Messages as the first
  tranche, then Gemini ACP and attached llama.cpp.

## Stop Conditions

- missing provider authority or contradictory auth terms
- a first route choice that establishes consumer product policy

## Auto-Continuation

Yes. Continue to card 036 after the compiled runway is current.

