# 2026-07-25 OpenCode Attached Prepared Facade

Status: complete

## Changed

`swallowtail-adapter-opencode` now exposes an adapter-local prepared path for
its attached HTTP/SSE harness.

Preparation binds one configured-instance identity and revision, execution
host, opaque endpoint target, delegated-auth access profile and evidence, and
bounded probe. It authorizes only that target, acquires and releases one
scoped delegated credential lease, and observes exact `/global/health`
version evidence. It does not start, stop, configure, authenticate, or recover
the attached service.

Separate prepared catalogue and read-only interactive-session values derive
immutable plans and matching requests. Catalogue selects no provider or model
route. Session preparation requires an explicit provider, route, model, and
working resource.

## Current Evidence

Current maintained OpenCode server documentation still exposes the headless
HTTP server, optional Basic Auth configuration, exact global health response,
provider catalogue, sessions, asynchronous prompts, abort, and SSE events:

- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)

This revalidation does not widen the frozen qualified range. Guaranteed
support remains the closed `1.14.48..=1.18.4` corpus with its exact gaps and
behavior milestones. Later exact stable releases remain visibly unverified.
No research or contract delta was required.

OpenCode HTTP/SSE remains a provider-specific harness interface, not ACP.
Remote ACP remains an explicitly selected reusable transport for compatible
provider adapters. There is no probe, upgrade, fallback, or recovery path
between them.

## Native Boundaries

- configured service ownership remains `ExternalAttached`
- harness configuration and isolation remain ambient
- provider session identity and directory affinity remain operation-scoped
- SSE, interruption, deadline, disconnect, and cleanup behavior remains on the
  unchanged low-level driver and handles
- resume remains unsupported
- catalogue observations do not imply access, entitlement, availability, or
  route selection
- no server lifecycle or authentication-discovery authority was added

## Validation

- five prepared-facade tests pass under local and remote-authoritative host
  identities
- qualified, incompatible, and unverified-newer health evidence stays
  distinct
- cancellation and host drift fail before endpoint effects
- credential cleanup succeeds before preparation returns; cleanup failure
  remains visible
- all 40 deterministic OpenCode tests pass
- public example and guide compile
- full Effigy QA passes
- 23-crate public API declaration gate passes
- the live installed endpoint probe remains separately ignored
- Doctor remains at the known 19 oversized-file findings: 7 errors and 12
  warnings

## Next

Card 027 starts g02.010 with separate Kimi Platform and DeepSeek
direct-inference facades. Cards 027-036 remain in the provider-wide facade,
package-proof, and replacement-candidate runway.
