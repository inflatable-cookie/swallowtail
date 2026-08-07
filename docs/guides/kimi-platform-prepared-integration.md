# Kimi Platform Prepared Integration

Use `swallowtail-adapter-kimi-platform` route `kimi-platform.chat`, driver ID
`swallowtail.kimi-platform.direct-chat`, for the provider-supported Kimi Open
Platform API over HTTPS/SSE. Choose it for account-scoped model observation or
one explicit K3 attempt. Reject it when the application needs Kimi Membership,
Kimi Code, tools, a reusable session, or provider-state management.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Operator Prerequisites

`prepare_kimi_platform_direct` requires configured-instance and host identity,
one approved `api.moonshot.ai` endpoint target, the Platform public API-key
profile, pay-as-you-go metering, provider support authority, opaque credential
reference, and matching access evidence. Operations use the host's endpoint,
credential, HTTP, task, and time services.

Membership, Kimi Code, regional Platform, and compatible bearer credentials
cannot substitute. Swallowtail does not discover accounts, read environment
variables, choose a model or endpoint, select billing, or fall back to another
Kimi route. Secrets and endpoint values do not enter plans or diagnostics.

The route has no ordered version range. It binds exact
`kimi-platform-chat-2026-07-21` facade behavior; compatible syntax or later
documentation does not create an unverified-newer posture.

## Preparation And Catalogue

Preparation is local and acquires no credential or endpoint grant. The
prepared value exposes safe configured-instance, access, service, and
low-level-driver evidence.

`prepare_catalogue` derives a model-catalogue operation without a model route.
`list_models` observes only the selected account and endpoint source. Presence
does not prove top-up, entitlement, balance, capacity, context availability,
or successful inference and cannot select a route automatically.

## One K3 Attempt

`prepare_inference_attempt` requires request identity, exact route and
`kimi-k3` model identity, user content, explicit `low`, `high`, or `max`
reasoning, a positive output-token bound, and optional host-monotonic deadline.

Call `start_run` once. Take and drain streaming events and the terminal outcome
concurrently, then close the run. Assistant output, reasoning, usage, rate and
request correlation, cancellation, provider failure, and cleanup remain
separate. A second request needs a new explicit prepared input and start; no
provider result or error authorizes retry.

The route exposes no attachments, structured output, consumer tools,
callbacks, working resources, provider history, continuation, background
execution, reattachment, reconciliation, archive/restore/delete, or fallback.
Additional compatible-chat features such as tool use need a separately
contracted lifecycle.

## Failures, Promotion, And Validation

Handle failures through portable classification and retain the exact
`swallowtail.kimi_platform.*` diagnostic for support. Never parse HTTP bodies,
SSE frames, provider prose, credentials, or endpoint values in consumer code.

Promotion requires exact platform/facade evidence, immutable plan and access
binding, bounded fixtures, lifecycle tests, and route-matrix coverage.

The compile-tested
[`prepared_kimi_platform_direct` example](../../crates/swallowtail-adapter-kimi-platform/examples/prepared_kimi_platform_direct.rs)
covers normal preparation. Validate deterministically:

```sh
effigy validate:focused swallowtail-adapter-kimi-platform
effigy check:examples
```

No live API call, credential use, or allowance spend is required.
