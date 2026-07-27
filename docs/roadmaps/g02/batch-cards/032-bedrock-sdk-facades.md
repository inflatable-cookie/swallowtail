# 032 Bedrock SDK Facades

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../011-specialized-runtime-facades.md`

## Objective

Add prepared facades for the separate SDK-native Bedrock Runtime and catalogue
drivers.

## Governing Refs

- Contracts 014, 019-020, 029, and 037
- Bedrock SDK fixtures
- card 031

## Scope

1. Prepare explicit region, endpoint, SDK identity, cloud client,
   delegated-credential provider, host, and route where required.
2. Bind Runtime inference and control-plane catalogue operations separately.
3. Preserve operation-private executor ownership and joined SDK work.
4. Keep catalogue availability separate from runtime capability and
   entitlement.
5. Add no ambient AWS configuration or credential-chain discovery.

## Acceptance Criteria

- [x] runtime and control-plane drivers remain separately registered
- [x] SDK and service version axes stay visible
- [x] explicit cloud-client configuration is required
- [x] catalogue results cannot select inference routes
- [x] credentials release after private executor join

## Validation

- both generated SDK fixture suites
- hosted-direct and catalogue conformance
- both host identities
- MSRV and package checks for Bedrock exceptions

## Evidence

- separate Runtime and catalogue preparation inputs bind different drivers,
  configured instances, access profiles, endpoint audiences, interface axes,
  plans, requests, and bound operations
- `BedrockCloudClientConfig` requires one exact region and one explicitly
  selected opaque credential provider
- prepared evidence retains region, SDK crate, SDK version, service API, and
  provider-neutral operation evidence
- catalogue preflight contains no model route or model identity
- 25 Bedrock tests pass across both generated corpora, prepared facades,
  conformance, low-level drivers, both host identities, and lifecycle cleanup
- warnings-denied lint, examples, Bedrock's `1.94.1` MSRV exception, current
  stable, docs, and public API declarations pass

## Addendum — Composite Provider Facade

Operator-authorized follow-up on 2026-07-27 adds `prepare_bedrock` as the
provider-level normal entry point. It binds only the shared execution host and
explicit `BedrockCloudClientConfig`.

The returned typed facade exposes separate `runtime` and `catalogue` branches.
Each branch still binds its own configured instance, target, access profile,
evidence, descriptor, version axes, plan, and low-level driver. The original
route-specific constructors remain public. No catalogue result selects an
inference route and no route fallback was added.

## Auto-Continuation

Yes. Continue to card 033.
