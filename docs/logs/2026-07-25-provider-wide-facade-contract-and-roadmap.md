# 2026-07-25 Provider-Wide Facade Contract And Roadmap

Status: complete

## Decision

Prepared integration is the normal path for every production driver, not a
Codex-only convenience. Each adapter owns typed constructors and bound
operations for its existing runtime roles. Low-level roles remain public.

No umbrella crate, central provider constructor, implicit routing, universal
prompt API, or mandatory sandbox was added.

## Inventory

The repository has 22 production routes:

- seven installed harness routes
- one attached harness network route
- six hosted direct or provider-owned-state routes
- three realtime routes
- two embedded SDK routes
- three local-runtime routes

Remote ACP remains a composable transport, not another provider route.

## Contract Promotion

Archived Spec 006 promotes:

- provider-wide route coverage into Contract 037
- typed bound execution through existing low-level roles
- adapter-local ownership of private behavior
- provider-neutral shared evidence and conformance only
- guaranteed range and unverified-newer separation
- provider-wide packaged evidence before candidate replacement

System architecture records Codex as the first realized proof and the other 20
routes as the active gap.

## Sequence

Roadmaps g02.007-g02.012 and cards 017-036 now own:

1. shared facade evidence and Codex bound operations
2. representative Kimi Code ACP, Anthropic Messages, and Ollama native proofs
3. remaining harness routes
4. remaining hosted direct and provider-state routes
5. realtime, Bedrock SDK, and llama.cpp routes
6. route guidance, packaged proof, and replacement candidate return

Card 016 was not executed. Card 036 supersedes it after provider-wide
acceptance. Publication remains unauthorized.

## Validation

This batch changes authority and planning documents only.

- production descriptor inventory: 22
- `effigy qa:docs`: pass
- focused roadmap, contract, spec, and log link check: pass
- `effigy qa:northstar`: pass
- `git diff --check`: pass
- `effigy doctor`: known 19 oversized-file findings, seven errors and 12
  warnings; count and category unchanged

## Next

Card 018 adds the smallest shared facade evidence and conformance foundation.
