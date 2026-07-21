# 2026-07-20 Owned Serving Conformance Closeout

## Changed

- applied the complete provider-neutral owned-self-hosted profile beside the
  production llama.cpp b10069 driver
- proved production start, host scope, ownership, endpoint binding, and joined
  cleanup under local and remote-authoritative execution-host identities
- added deterministic readiness-timeout and exact route-mismatch fixtures to
  the existing startup, exit, build, graceful-stop, and forced-stop corpus
- retained separate b9910 attached conformance, including the no-stop boundary
  across success, failure, cancellation, and deadline cleanup
- closed roadmap 019 and activated roadmap 018 with card 065 as the sole next
  task

## Current State

Roadmap 019 and cards 060-064 are complete. The owned b10069 and attached b9910
drivers share bounded protocol parsing but retain separate identity, lifecycle,
and stop authority. Card 065 now freezes the latest maintained Kimi successor;
card 057 follows with the selected native macOS containment helper. Cards
058-059 remain blocked until both proofs pass.

## Validation

- 28 focused llama.cpp tests pass without a binary, model, credential, or live
  route
- all 257 repository tests pass; two installed or live probes remain skipped
  by default
- `effigy lint:rust`, formatting, repository QA, whitespace, and diff checks
  pass
- doctor remains at the inherited 19 findings, including 7 errors

## Risks

- the b10069 startup log remains a release-specific evidence surface rather
  than a stable cross-version API
- deterministic fixtures prove lifecycle behavior, not model compatibility,
  memory sufficiency, or throughput
- persistent serving, model acquisition, router mode, public listeners, and
  Monkey lifecycle authority remain excluded
- Kimi production remains blocked on current successor evidence and complete
  native descendant containment

## Next

Execute card 065. Pin the latest maintained Kimi successor and replace or
retain every legacy ACP fixture assertion from exact current evidence before
building the macOS helper.
