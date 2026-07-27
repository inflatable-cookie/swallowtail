# OpenCode Session Deletion Driver

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/056-opencode-session-deletion-driver-and-facade.md`

## Change

The attached OpenCode HTTP adapter now exposes provider-session deletion
through the shared runtime management role and its prepared facade.

Prepared session open promotes the exact persistent session reference into an
opaque management binding carrying:

- driver, integration, transport, instance, host, and endpoint identity
- exact server version and compatibility assessment
- delegated access evidence
- the bound read-only working resource
- only `ProviderSessionDelete`

The typed prepared delete operation requires caller-asserted inactive state.
Archive and restore remain unsupported.

## Effect Mapping

Before DELETE, the driver validates the immutable management plan, execution
host, approved endpoint, delegated credential, read-only resource lease, and
exact server health version.

The production mapping is:

- HTTP 200 with JSON `true`: applied `ProviderDataDeleted` over
  `ProviderDefinedDescendants`
- 4xx, including missing target: rejected before effect
- malformed 2xx or 5xx: unconfirmed after effect
- transport loss, cancellation, or deadline after dispatch: unconfirmed

Raw provider bodies never enter diagnostics. Transport work joins before
resource and credential release. The adapter gains no start, stop, archive,
restore, retry, fallback, or local-file authority over the attached service.

Stable newer OpenCode versions remain permitted but unverified. Prepared
deletion requires explicit acceptance outside the qualified upper bound.

## Validation

- OpenCode protocol tests: 11 passed
- OpenCode prepared-facade tests: 10 passed
- provider-neutral management conformance: 3 passed
- `effigy check:rust`: passed
- `effigy format:check`: passed after formatting

Initial parallel OpenCode runs hit nondeterministic fixture transport failures
while focused cases passed. Card 057 traced them to inherited nonblocking
accepted sockets, repaired the fixture, and completed the full adapter
regression and topology closeout.

## Next

Card 057 subsequently completed all qualified deletion segments, both host
topologies, pre-dispatch rejection, post-dispatch uncertainty, and unchanged
OpenCode session behavior.
