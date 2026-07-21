# 2026-07-21 Optional Harness Isolation Rebaseline

## Changed

- promoted Research 012 from current T3 Code and maintained harness evidence
- made local harness isolation explicit as `AmbientHost`,
  `ProviderEnforced`, or `HostEnforced`
- made generic resource-bound harness requests ambient by default
- kept direct inference free of a fake harness-isolation posture
- recorded ambient process and network authority separately from denied or
  host-approved provider-side network
- retained Codex read-only and bounded-workspace provider sandbox mappings
- reclassified Gemini ACP and attached OpenCode as ambient without changing
  their Plan Mode, permission, callback, transport, or cleanup behavior
- closed card 057 as a completed negative capability proof: the pinned Kimi
  artifact cannot support the tested host-enforced App Sandbox profile
- reopened roadmap 018 and made card 058 ready for explicit ambient Kimi ACP

## Current State

Swallowtail now matches the relay-first control-plane boundary used by T3 Code
and similar orchestrators. A local harness does not need an outer process
sandbox to communicate through Swallowtail. Enforced isolation remains an
optional exact capability and never falls back to ambient execution.

Kimi Code `0.28.1` may proceed with its pinned executable, isolated
`KIMI_CODE_HOME`, delegated authentication, working-resource location, bounded
ACP callbacks, persistent lifecycle, and joined cleanup. The route will make
no bounded filesystem, descendant, or provider-tool network claim.

Remaining risk is explicit: an ambient Kimi process and its descendants retain
the execution host user's authority. Consumers own whether to expose that
route and its user-facing risk posture. The later Kimi driver must bind
isolation into new/load/resume state and reject posture drift before provider
work.

## Validation

- focused core, testkit, Codex, Gemini, and OpenCode tests pass
- full repository QA passes with 262 tests; installed/live probes remain gated
- warnings-denied clippy, all-target compile, formatting, docs, Northstar, and
  diff checks pass
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors

## Continuation

- card 058 is ready: ambient Kimi ACP production driver
- card 059 remains blocked only by card 058: cross-agent conformance and
  roadmap closeout
- optional provider- or host-enforced Kimi isolation remains outside this lane
