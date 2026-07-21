# 2026-07-21 Bedrock Runtime Driver And Conformance

## Outcome

Completed Swallowtail's first production SDK-native route: Amazon Bedrock
Runtime `ConverseStream` through the official AWS SDK for Rust.

## Changed

- added a separately identified hosted-direct structured-run descriptor for
  the `amazon-bedrock` family over `rust-sdk-eventstream`
- bound one exact configured instance, execution host, access profile,
  credential reference, AWS region, delegated credential provider, endpoint
  grant, model route, model, and positive output-token limit
- configured the SDK directly with no default AWS credential chain, region or
  endpoint discovery, and one total attempt
- ran the SDK inside host-approved blocking work on an operation-private Tokio
  runtime; no global executor or detached task was added
- projected typed EventStream text and usage through a bounded runtime channel
- joined cancellation, complete-stream deadline, SDK work, scoped task, and
  delegated credential release
- added deterministic success, binding, cancellation, deadline, topology, SDK
  type, failure, drift, redaction, and provider-neutral hosted-profile tests

## Access And Authority

The runtime credential lease is delegated and exposes no secret. The adapter
uses only the credential-provider object supplied in the exact driver binding.
IAM permission, credential refresh, model access, regional availability,
quota, rate, billing, and runtime readiness remain account-specific evidence.
No model, region, endpoint, provider, account, or fallback is selected by
Swallowtail.

No AWS credential, ambient configuration, metadata endpoint, provider account,
or paid inference request participated in default validation.

## Validation

- 11 focused Bedrock tests pass: five driver/runtime, five typed SDK fixtures,
  and one provider-neutral hosted-direct profile
- focused warnings-denied clippy passes
- full `effigy qa` passes with 283 tests; three installed or live probes remain
  separately gated and ignored by default
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and 7 errors

## Remaining Risks

- live AWS identity refresh, IAM, model access, regional availability, rate,
  quota, billing, and provider-side request acceptance are unproved and remain
  separately gated
- local cancellation cannot prove that AWS did not already receive a signed
  request
- generated SDK and Bedrock EventStream drift require a new frozen corpus
- Bedrock model discovery uses a separate control-plane client, endpoint
  audience, permissions, and evidence posture

## Continuation Record

Roadmap 021 and card 070 own the separate Bedrock control-plane catalogue
recheck. Cards 071-072 remain in bounds for exact generated fixtures and a
production catalogue driver. Kimi Rust Wire and the ACP Rust SDK rechecks stay
behind that bounded lane.
