# Kimi Platform Prepared Integration

Use this facade for the provider-supported Kimi Open Platform API. It binds one
host-approved endpoint target, the exact `api.moonshot.ai` audience, the dated
Kimi chat facade, pay-as-you-go Platform access, and either catalogue or K3
inference plans.

## Access Is Platform-Specific

`prepare_kimi_platform_direct` requires:

- one configured-instance identity and revision
- one execution host and approved endpoint target
- one API-key access profile for `api.moonshot.ai`
- pay-as-you-go metering and provider support authority
- observed or caller-asserted access evidence

Kimi Membership, Kimi Code, and regional Platform credentials are not accepted
by this route. A compatible bearer-key shape does not make those audiences,
entitlements, or billing systems interchangeable.

Preparation performs no provider request and acquires no credential. The
prepared value exposes the safe configured instance, access provenance,
available host services, and low-level driver escape hatch.

## Catalogue

`prepare_catalogue` derives a model-catalogue plan without a model route.
`list_models` observes only the selected account and endpoint source. A
returned model does not select a route or prove top-up, entitlement, balance,
capacity, or successful inference.

## One K3 Attempt

`prepare_inference_attempt` requires:

- one exact route identity, route revision, and `kimi-k3` model identity
- request identity and user content
- explicit `low`, `high`, or `max` reasoning
- one positive output-token bound
- an optional host-monotonic deadline

`start_run` makes exactly one streaming Chat Completions attempt. The prepared
route declares no tools, direct continuation, retry, provider history, or
fallback. A second attempt requires another explicit preparation and start.

Kimi now documents additional compatible-chat features, including tool use.
Their JSON shape does not widen this frozen structured-run route. Supporting
them requires a separately contracted operation lifecycle.

`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for diagnostics and advanced use.

See the compile-tested
[`prepared_kimi_platform_direct` example](../../crates/swallowtail-adapter-kimi-platform/examples/prepared_kimi_platform_direct.rs).
