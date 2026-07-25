# 026 OpenCode Attached Harness Facade

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../009-remaining-harness-facades.md`

## Objective

Add the prepared normal path for the attached OpenCode HTTP/SSE harness.

## Governing Refs

- Contracts 014, 017, 020, 023, 029, and 037
- OpenCode HTTP/SSE driver fixtures
- card 025

## Scope

1. Prepare one host-approved endpoint, access posture, instance, and exact
   facade compatibility claim.
2. Bind catalogue and interactive harness operations separately.
3. Preserve attached-service ownership and provider session affinity.
4. Keep HTTP/SSE transport behavior private to the adapter.
5. Add no server start, authentication discovery, or recovery fallback.

## Acceptance Criteria

- [x] endpoint selection and service ownership stay explicit
- [x] catalogue does not imply access or route selection
- [x] session affinity and cleanup remain operation-scoped
- [x] no attached service lifecycle authority is gained
- [x] all eight harness routes now have prepared normal paths

## Validation

- OpenCode deterministic HTTP/SSE suite
- supported topology conformance
- drift, disconnect, cancellation, deadline, and cleanup cases

## Auto-Continuation

No. Close g02.009 and advance to card 027.

## Completion Evidence

- Preparation authorizes one opaque host-approved endpoint, acquires and
  releases one delegated credential lease, and observes exact
  `/global/health` output under a bounded cancellation and deadline scope.
- The configured instance retains `ExternalAttached` ownership, ambient
  configuration, the exact `opencode.server` binding, and qualified or visible
  unverified-newer compatibility.
- Prepared catalogue and read-only interactive-session values are separate.
  Catalogue carries no provider or model route. Session preparation requires
  explicit provider, model route, model, and working resource.
- Provider session identity, directory affinity, SSE behavior, interruption,
  deadline, disconnect, and cleanup remain on the unchanged low-level session
  and turn lifecycle. Resume remains unsupported.
- OpenCode HTTP/SSE is not ACP. Remote ACP remains a separate explicitly
  selected transport with no probe, upgrade, fallback, or recovery path
  between the two.
- Five prepared-facade tests cover both host identities, version and target
  drift, unverified-newer execution, cancellation-before-effects, exact
  compatibility rejection, and visible cleanup failure.
- All 40 deterministic OpenCode tests pass. Full Effigy QA and the 23-crate
  public API declaration gate pass. The live installed endpoint probe remains
  separately ignored.
