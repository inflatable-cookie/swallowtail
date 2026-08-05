# 051 Portable Failure Classification

Status: accepted
Owner: Tom
Updated: 2026-08-05

## Purpose

Give consumers one provider-neutral failure classification without erasing
route identity, exact lifecycle truth, safe diagnostics, or provider
differences.

## Boundary

`SafeDiagnostic` remains the stable safe code and message carried by runtime,
preparation, terminal, cleanup, lifecycle, and catalogue failures. It may also
carry one portable `FailureClassification`:

- `FailureOrigin` identifies the layer that reported or caused the failure
- `FailureKind` identifies the strongest machine-supported meaning
- `FailureRecovery` records bounded recovery evidence, not consumer policy

The exact diagnostic code remains available for logs, support, and
route-specific inspection. A consumer may handle ordinary failures through
the portable classification without switching on adapter-owned code strings.

## Origin

Portable origins are:

- provider
- harness
- host
- transport
- protocol
- Swallowtail runtime
- unknown

Terminal provider, host, and runtime status remains distinct under Contract
009. Classification origin does not replace that status. A provider failure
may have transport origin; a runtime terminal may have protocol origin.

## Kind

The portable kind set covers:

- authentication required or rejected
- authorization denied
- entitlement unavailable
- model unavailable
- rate limited or quota exhausted
- provider unavailable
- invalid request
- input or context limit exceeded
- resource missing or stale
- harness unavailable or incompatible
- transport interrupted
- protocol incompatible or malformed data
- host service unavailable
- runtime invariant failure
- unknown

Kinds stay coarse. Provider-native codes, status values, and prose do not
become a second portable vocabulary.

## Recovery Evidence

Portable recovery evidence is limited to:

- unknown
- retry may succeed
- reauthentication required
- configuration change required
- input change required
- harness update required
- the same request is not retryable

This is adapter evidence. Consumers still own retry timing, backoff, user
wording, provider selection, fallback, and product workflow under Contract
004. Exact rate reset evidence remains in the existing rate-limit records; it
is not duplicated into failure classification.

## Mapping Rules

- map only documented status, typed provider codes, typed harness events,
  exact process exits, or qualified interface behavior
- never parse provider prose, stderr, assistant output, prompts, or raw bodies
- unknown or incomplete evidence remains `Unknown`
- route-specific safe codes remain stable when classification is added
- arbitrary JSON values cannot create a portable class
- malformed translation data remains protocol or runtime failure, not a
  fabricated provider failure
- normal assistant refusal remains output unless the provider reports a typed
  policy failure
- permission and typed-question callbacks remain callback lifecycle, not
  failures
- cleanup remains orthogonal to the primary terminal result

## Non-Terminal Error Activity

Warnings, provider errors, retry notices, and failed tools may remain activity
under Contract 044 without ending the operation. If an activity carries a
safe diagnostic, it uses this same classification. Display content is not
parsed into a diagnostic or classification.

## Compatibility

Unclassified `SafeDiagnostic::new` construction remains valid and yields an
all-unknown classification. This gives every route the portable shape while
allowing evidence-backed mappings to land without guessed semantics.

Adding classification must not change:

- safe diagnostic code or message
- terminal status
- preparation or provider-session failure stage
- cleanup outcome
- callback, cancellation, timeout, or detachment behavior
- route, model, credential, access, or authority selection

## Acceptance

- core records prove safe default and classified construction
- runtime terminal helpers expose one borrowed failure view without
  route-specific matching
- preparation and cleanup retain existing distinct stages and outcomes
- representative direct-provider classes normalize across adapters
- representative harness failures retain harness origin or honest unknown
- non-terminal activity cannot acquire diagnostic evidence outside the
  warning-or-error kind
- raw provider and harness payloads remain absent from stable diagnostics
- focused core, runtime, and mapped-adapter validation passes without live
  provider work
