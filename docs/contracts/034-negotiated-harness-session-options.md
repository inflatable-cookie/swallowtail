# 034 Negotiated Harness Session Options

Status: active
Owner: Tom
Updated: 2026-07-24

## Purpose

Apply one consumer-selected portable session option through a harness-
advertised configuration channel without exposing a generic provider
configuration API or changing the preflight-bound model route.

## Portable Selection

The consumer continues to use the typed `SessionOptions` boundary from
Contract 012. A driver may translate only an option already represented by a
portable capability and exact preflight constraint.

The first negotiated option is reasoning mode:

- the immutable plan contains `ReasoningSelection` and the exact requested
  `ReasoningMode`
- the configured model route remains unchanged
- provider option ids, labels, categories, ordering, and raw values remain
  adapter-private
- provider model, agent mode, permission mode, tool policy, and arbitrary
  configuration are not implied

No common string-to-string option map is introduced. A provider option without
one exact portable mapping is observation only.

## Capability And Version Gate

Negotiated option support belongs to one exact driver behavior revision and
interface-version assessment. A stable wire protocol does not prove that every
harness release advertises or applies the same options.

Before provider work, the driver validates:

- exact configured-instance, request, and immutable-plan agreement
- the portable capability and requested value
- the lifecycle operation on which selection is supported
- the exact qualified or permitted unverified-newer behavior revision

An unverified-newer attempt uses the latest qualified private mapping under
Contract 029. Missing, renamed, structurally changed, or rejected provider
options remain runtime drift. They do not trigger another value, model, route,
driver, or provider.

## Negotiation Sequence

Some harness protocols advertise session options only after creating a
provider session. For a lifecycle that claims negotiated selection:

1. create or attach through the already-authorized operation
2. receive one bounded option snapshot
3. identify the one adapter-private option mapped to the requested portable
   value
4. require the exact requested value to be selectable
5. send one correlated provider option-selection request
6. require a response or update that confirms the effective value
7. return a ready session only after confirmation

The snapshot is mutable provider evidence. It cannot widen the model route,
capability claim, access profile, or support window. Duplicate option ids,
ambiguous portable mappings, invalid option shapes, missing confirmation, or
an effective-value mismatch fail the attachment.

A driver may support selection for new sessions while rejecting it for load or
resume. Unsupported lifecycle and option combinations reject before provider
work. Empty options retain the previously qualified lifecycle behavior.

## Provider State And Failure

Provider session allocation may precede option drift. Failure after allocation
returns no ready runtime session and joins the attachment, callback, process,
resource, and credential work. It does not claim provider-session deletion or
rollback unless a separately authorized lifecycle proves it.

Durable retention, load, resume, replay, deletion, and consumer persistence
remain governed by Contract 017. Negotiated reasoning does not authorize
mutation of an arbitrary previously persisted session.

## Observation And Updates

The runtime may expose the effective portable reasoning mode as typed evidence.
It does not expose raw provider option snapshots.

Later provider option updates:

- may confirm the already selected portable value
- may report provider drift
- cannot silently change the consumer-selected value
- cannot trigger model switching or capability widening

Changing reasoning after readiness needs a later explicit runtime operation.
The first boundary configures setup only.

## Isolation And Access

Negotiated options do not imply sandboxing, filesystem containment, permission
approval, credential authority, provider sign-in, configuration-file
authority, or model access. Harness isolation remains the separately selected
posture from Contracts 017 and 023.

## Conformance

Deterministic fixtures prove:

- exact portable selection and preflight agreement
- version-specific boolean and multi-level provider mappings
- option omission, ambiguity, unsupported value, malformed shape, rejected
  selection, missing confirmation, and effective-value mismatch
- no fallback from a requested level to a default, boolean alias, model, or
  route
- new-session selection does not imply load or resume mutation
- empty options preserve existing session behavior
- qualified and unverified-newer assessments remain visible and distinct
- provider allocation followed by drift still joins all owned work
- provider option ids, labels, payloads, and credentials stay out of stable
  diagnostics

## Acceptance

- portable typed options remain the public boundary
- provider configuration channels remain adapter-private
- exact version behavior gates every negotiated mapping
- the selected model route cannot change
- readiness requires exact effective-value confirmation
- unsupported lifecycle combinations fail before provider work
- option drift never causes fallback
- isolation, access, persistence, and deletion remain independent
