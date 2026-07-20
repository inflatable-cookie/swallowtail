# Runtime Identity, Access, And Requirements

Status: completed
Owner: Tom
Roadmap: 004 Runtime Records and Preflight
Updated: 2026-07-19

## Goal

Add the pure Contract 008 records that bind driver, instance, route, access,
ownership, topology, host-service, and capability requirements.

## Scope

- integration and transport family identity
- configured-instance and execution-host identity
- execution layer and operation shape
- external, owned-ephemeral, and owned-persistent ownership modes
- safe access profile and dimensional access status
- model-route identity separate from `ModelId` display metadata
- host-service kinds
- named capability requirements with the first parameterized constraints
- driver descriptor and safe discovery outcome records
- focused core validation and unit tests

## Out Of Scope

- async traits or runtime crate
- executable, endpoint, credential, attachment, or schema resolution
- provider and consumer records
- serialization framework selection
- compatibility aliases for pre-1.0 names

## Acceptance Criteria

- invalid blank identities fail with safe diagnostics
- records preserve all Contract 008 identity dimensions
- access status cannot collapse to one ready boolean
- no record contains secret values or raw host paths
- unsupported or unknown constraints remain explicit
- `swallowtail-core` retains no external dependencies

## Validation

- focused core tests
- `effigy qa`
- `git diff --check`

## Stop Condition

Stop before adding behavior that resolves a host reference or contacts a
provider.

## Closeout

- `swallowtail-core` now separates integration, driver, transport, configured
  instance, execution host, model route, access profile, and protocol-facade
  identity.
- Access profiles and status preserve credential, entitlement, endpoint,
  readiness, and support-authority dimensions without an aggregate boolean.
- Driver descriptors, configured instances, model routes, operation
  requirements, host-service kinds, and parameterized capability profiles are
  pure records with no external dependencies.
- Host target references are opaque in default formatting; public records
  contain no credential values or raw paths.
