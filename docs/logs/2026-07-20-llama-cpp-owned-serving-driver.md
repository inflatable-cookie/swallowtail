# 2026-07-20 llama.cpp Owned Serving Driver

## Changed

- added a distinct production driver for the pinned llama.cpp `b10069` owned
  child and retained the existing b9910 attached driver unchanged
- mapped only the host-resolved executable and artifact plus exact alias,
  loopback port zero, offline, no-UI, and no-agent launch arguments
- added bounded stderr parsing for the tagged listening record and kept output
  supervision active through health, exact build, and single-route readiness
- published the dynamic endpoint only after exact loopback observation and
  returned no owned handle before readiness completed
- joined graceful or forced child stop before endpoint invalidation and
  artifact-lease release on normal and failed startup paths
- parameterized the bounded llama.cpp protocol facade by observed build so
  b9910 attached and b10069 owned routes share parsing without sharing
  lifecycle authority

## Current State

Card 063 is complete. Card 064 is ready for production-driver conformance,
topology, attached non-regression, full validation, and roadmap 019 closeout.
Kimi cards 057 and 065 remain ready but queued; cards 058-059 remain blocked.

## Validation

- 25 focused adapter tests pass without a llama.cpp binary or model
- all 254 repository tests pass; two installed/live probes remain gated
- focused all-target warnings-denied clippy passes
- doctor remains at the inherited 19 findings, including 7 errors

## Risks

- the tagged startup log is not a stable cross-release API; any llama.cpp
  upgrade needs a new exact fixture and parser review
- `--offline` constrains llama.cpp retrieval behavior but is not a general
  process network sandbox
- default fixtures prove lifecycle semantics with a scripted child and bounded
  loopback HTTP server, not model compatibility or memory sufficiency
- persistent serving, router mode, acquisition, and Monkey lifecycle authority
  remain outside this driver

## Next

Execute card 064. Apply the common owned profile to the production driver,
prove topology and b9910 attached non-regression, then run full repository QA.
