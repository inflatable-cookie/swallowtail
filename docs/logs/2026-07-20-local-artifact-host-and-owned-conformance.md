# 2026-07-20 Local Artifact Host And Owned Conformance

## Changed

- bound local model-artifact and serving-endpoint services to one explicit
  execution-host identity
- added exact artifact approvals with full descriptor, regular-file, and
  lowercase SHA-256 checks before lease issue
- kept artifact material consumer-owned; lease release drops tracked authority
  and never deletes the file
- added per-host dynamic endpoint publication for exact nonzero IPv4 and IPv6
  loopback HTTP sockets
- connected published endpoint references to scoped network authorization and
  invalidated them on awaited release
- expanded the provider-neutral owned profile with process and lease cleanup
  ordering plus local and remote-authoritative topology fixtures

## Current State

Card 062 is complete. Card 063 is ready to implement the exact llama.cpp
`b10069` owned-child lifecycle through the existing bounded protocol facade.
The local host owns resolution and publication mechanics; provider startup,
readiness, and health interpretation remain adapter concerns.

Kimi remains independently paused. Unpausing it requires selecting and proving
one deployment-owned containment platform and mechanism under card 057 before
cards 058-059 can become ready.

## Validation

- 50 focused local-host and testkit tests pass
- focused warnings-denied clippy and all-target compile pass
- full repository QA passes with 236 tests; two installed/live probes remain
  separately gated and ignored
- docs QA, Northstar QA, formatting, whitespace, and diff checks pass
- doctor remains at the inherited 19 findings, including 7 errors

## Risks

- artifact verification establishes content at lease acquisition; the
  deployment still owns stability of the approved source path for the lease
  lifetime
- dynamic publication authorizes only an observed loopback socket; adapter
  readiness, build, and route checks must complete before returning a handle
- remote-authoritative fixtures prove public identity and lifecycle semantics,
  not an unimplemented remote launcher
- Kimi callback mediation still cannot substitute for descendant filesystem
  containment

## Next

Execute card 063. Add deterministic b10069 startup and failure fixtures, then
implement owned start, readiness, and joined teardown without a live model.
