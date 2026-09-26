# Generation-Control Application And Enforcement

Status: active
Owner: Tom
Created: 2026-07-28

## Purpose

Apply portable output-token, reasoning, and structured-output controls without
claiming more provider enforcement than the exact route proves.

This contract does not create a generic generation-parameter map.

## Independent Controls

The three portable controls remain independent:

- `OutputTokenLimit` carries one positive caller maximum
- `ReasoningSelection` carries one exact `ReasoningMode`
- `StructuredOutput` carries one bounded `StructuredOutputDescriptor`

Support for one does not imply either other control. Catalogue limits,
reasoning output, JSON envelopes, prompt conventions, client truncation, tool
budgets, and context windows do not imply these capabilities.

Each requested control must appear as an exact capability and constraint in
the immutable plan. Request, prepared operation, plan, and dispatch must agree
before provider work.

## Application States

Adapters keep these states separate:

1. requested — the consumer supplied the portable control
2. planned — preflight authorized the exact control and constraint
3. dispatched — the adapter encoded the qualified provider or harness field
4. accepted — the selected surface did not reject that field
5. effective — the provider or harness explicitly confirmed the applied value
6. observed — later output or usage supplied relevant evidence

Dispatch does not prove acceptance. Acceptance does not prove an effective
value unless the exact interface confirms it. Output shape, token count, or
reasoning text cannot be reverse-engineered into confirmation.

When an interface has no effective-value confirmation, Swallowtail may claim
qualified dispatch and provider acceptance only. It must not synthesize an
effective-value observation.

## Output Maximum

The portable maximum is a positive generation bound. The adapter maps it to
one exact provider or harness field whose selected behavior limits generated
output.

A route does not qualify when it only:

- reports a catalogue maximum
- stops reading after a client byte or token threshold
- truncates completed output
- limits context, turns, tools, cost, or wall-clock time
- permits the harness to raise or retry beyond the requested bound

Provider numeric range conversion is checked before effects. Overflow, zero,
unsupported model combinations, ignored fields, or automatic escalation fail
closed.

## Reasoning Selection

`ReasoningMode` is an exact portable selection, not a request for the adapter
to choose a close provider value.

Provider booleans, effort names, numeric budgets, model variants, and
harness-option ids remain adapter-private mappings. Every mapping belongs to
an exact driver behavior revision and, where required, exact model capability
evidence.

An adapter cannot:

- clamp to the nearest supported value
- replace an unsupported value with provider default
- infer support from emitted reasoning
- apply a value to another model, route, operation shape, or version segment
- treat an accepted request as confirmed effective selection

Harness negotiation that confirms an effective value remains under Contract
034. Request-body and operation-private configuration mappings follow this
contract and report only the evidence their surface returns.

## Structured Output

The portable descriptor retains document, media type, dialect, digest, and
transport bounds. A driver accepts only its qualified dialect and supported
schema subset.

Structured-output enforcement has one of these exact sources:

- `ProviderNative` — the inference provider accepts and enforces the schema
- `HarnessValidated` — the selected harness owns a schema tool, validation,
  and any bounded retry

The source is part of the prepared capability claim. It is not inferred from
transport family or provider name.

Prompt instructions, JSON mode without schema, CLI result envelopes, and
consumer post-validation do not qualify. Swallowtail does not inject a prompt
to emulate support.

Provider or harness enforcement does not make consumer acceptance implicit.
The consumer still owns schema meaning, validation policy, repair, ranking,
and use of the result under Contract 010.

Harness validation does not grant implicit retry authority. A non-zero retry
count requires an exact preflight-bound attempt budget. The first OpenCode
mapping uses zero schema retries.

## Model And Version Qualification

Some controls depend on the selected model even when their wire field is
stable. A prepared operation advertises only the exact modes, limits,
dialects, and enforcement source qualified for its model route and interface
version.

Contract 029 milestones may add, change, or remove a control inside one
provider route. The configured instance remains one identity. Guaranteed
support follows the qualified baseline, milestones, exclusions, and latest
point. An allowed unverified-newer attempt uses the latest qualified mapping
without extending the guarantee.

Another executable or distribution is not a newer segment merely because it
shares a command name or provider branding. It requires its own identity,
version axis, preparation evidence, and route qualification.

Unknown model capability never becomes support. Unsupported combinations fail
before provider work when knowable at preflight. Provider rejection after
dispatch remains a provider failure, not a fallback trigger.

## Harness Configuration

A typed portable control may map to one exact operation-private command,
request field, or child-process environment value without exposing a generic
configuration API.

Ambient configuration cannot override the planned portable value. A mapping
that needs a synthetic file tree or isolated configuration root remains gated
by Contract 033's host-scoped lease. Adapters cannot mutate user configuration
or create a temporary home under this contract.

## Failures And Diagnostics

Stable diagnostics distinguish:

- unsupported portable capability or value
- request, plan, or dispatch mismatch
- numeric or schema transport invalidity
- provider rejection
- harness rejection or missing confirmation
- interface drift

Diagnostics expose no schema body, prompt, output, raw provider payload,
credential, endpoint, model-private detail, or host path.

No control failure authorizes retry, fallback, another model, another route,
another provider, or weaker enforcement.

## Selected First Tranche

The first implementation covers:

- OpenAI background reasoning and provider-native structured output
- OpenAI Realtime output maximum
- Ollama attached reasoning and provider-native structured output
- OpenCode HTTP reasoning and harness-validated structured output

All use existing route identities and prepared facades. xAI remains
operator-held.

## Conformance

Deterministic fixtures prove:

- exact request, plan, and wire agreement
- no effect before capability and constraint validation
- exact reasoning value with no clamp or default
- positive bounded output maximum with no escalation
- bounded schema transport and exact dialect
- provider-native versus harness-validated enforcement source
- unsupported, malformed, ignored, drifted, and unverified-newer behavior
- redacted failures and unchanged topology, access, cancellation, and cleanup

Live credentials and provider requests are separately gated and unnecessary
for default QA.

## Acceptance

- no generic parameter map enters the public API
- every control is independent and exact
- dispatch, acceptance, effectiveness, and observation remain distinct
- model and version capability cannot be inferred
- schema enforcement source remains visible
- prompt conventions and post-validation remain non-capabilities
- unsupported behavior fails without fallback
