# Runtime And Stateful Harness Decomposition

Date: 2026-07-30

Card 153 removes all ten runtime, Claude Agent, Gemini, and Kimi error-level
structural findings.

## Changes

- Runtime request and driver roles are split behind unchanged public
  declarations. Private request state, forwarding implementations, and trait
  item macros preserve the same construction, validation, and dispatch paths.
- Provider-session and typed harness-input tests move into private fragments.
- Claude Agent validation, ACP fixture behavior, and headless cases are split
  by operation family.
- Gemini catalogue transport and validation helpers plus headless cases are
  split from their stable entry points.
- Kimi local-server activity projection and WebSocket event decoding are split
  from record definitions and protocol entry points.

## Evidence

- focused tests: 168 passed
- focused warnings-denied clippy: passed
- 24-crate public-API declaration baseline: passed
- doctor: 148 findings, 141 warnings, seven high errors
- no runtime, Claude Agent, Gemini, or Kimi error finding remains

Card 154 now includes the residual provider-route matrix base fragment beside
the six remaining adapter files. This keeps card 155 as a pure zero-error
acceptance gate.

## Next

Card 154 removes the final seven high findings.
