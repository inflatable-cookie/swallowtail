# 2026-07-21 Bedrock Catalogue Records And Fixtures

## Outcome

Completed card 071. Added the provider-neutral catalogue observation records
required by Contract 020 and froze the exact generated
`aws-sdk-bedrock = 1.148.0` control-plane corpus. Card 072 is ready.

## Changed

- added optional model catalogue observations for source, provider display
  name, input and output modalities, advertised response streaming, inference
  types, customization types, lifecycle status and transitions, and bounded
  provider-defined values
- preserved absent repeated evidence separately from an observed empty set and
  absent streaming separately from observed `false`
- added a provider-neutral epoch timestamp so generated SDK date types do not
  escape the adapter
- kept provider display name separate from stable provider id and catalogue
  observations separate from model route and runtime capability
- pinned `aws-sdk-bedrock = 1.148.0` with default features disabled for the
  fixture batch
- froze generated unfiltered request, single output collection, model summary,
  lifecycle, enum, typed error, explicit configuration, and retry types
- added a bounded fixture projector for known values, namespaced unknown values,
  optional fields, lifecycle timestamps, error classification, and redaction

## Bounds And Authority

- at most 1,024 model summaries per response
- at most 32 values per repeated observation category
- model id, model name, provider name, and provider-defined code lengths are
  bounded; blank, surrounding-whitespace, control-character, and oversized
  values fail
- overflow fails the whole projection; it is never reported as a complete
  truncated catalogue
- model ARN stays inside the generated fixture and is not projected
- the fixture requests no filters, pagination, runtime inference, route
  creation, Marketplace action, provider onboarding, or Mantle `/models`
- no AWS credential source, account, endpoint, network request, or paid model
  participated in validation

## Validation

- 42 focused core and Bedrock tests pass
- focused warnings-denied clippy passes
- workspace all-target `effigy check:rust` passes
- `git diff --check` passes
- `effigy doctor` retains the inherited 19 oversized-file findings: 12
  warnings and 7 errors

## Remaining Risks

- production endpoint, access, credential, deadline, cancellation, private
  executor, and cleanup mapping remains unproved until card 072
- live IAM, regional catalogue contents, identity refresh, and provider service
  behavior remain separately gated
- generated enum or service-model drift requires a new exact fixture pin
- the fixture projector is test-only until card 072 connects it to the
  production catalogue driver
- the separate Bedrock Mantle catalogue remains outside roadmap 021

## Continuation Record

Card 072 is ready. Promote the frozen projection into the production module,
add the separately registered control-plane driver, prove both supported host
topologies offline, then close roadmap 021. No other card is blocked in this
lane.
