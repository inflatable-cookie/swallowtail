# Portable Failure Handling

Use portable classification for ordinary application behavior. Keep the exact
safe diagnostic for logs and support.

The boundary applies to every production route. A route may classify only the
failure evidence it can prove. Missing provider detail remains `Unknown`; one
adapter's classification cannot be copied to another route or transport.

## Terminal Failures

`TerminalOutcome::failure` removes the need to match each terminal failure
variant merely to obtain its diagnostic:

```rust
use swallowtail_core::{FailureKind, FailureRecovery};
use swallowtail_runtime::{TerminalFailureSource, TerminalOutcome};

fn handle_failure(outcome: &TerminalOutcome) {
    let Some(failure) = outcome.failure() else {
        return;
    };
    let diagnostic = failure.diagnostic();
    let classification = diagnostic.failure_classification();

    match classification.kind() {
        FailureKind::AuthenticationRequired | FailureKind::AuthenticationRejected => {
            // Offer the consumer's sign-in flow.
        }
        FailureKind::RateLimited | FailureKind::ProviderUnavailable => {
            // Apply consumer-owned retry policy if appropriate.
        }
        FailureKind::ModelUnavailable => {
            // Ask the operator to select or configure a valid model.
        }
        _ => {
            // Present diagnostic.message() and retain diagnostic.code().
        }
    }

    if classification.recovery() == FailureRecovery::ReauthenticationRequired {
        // This is route evidence, not authorization for automatic login.
    }

    match failure.source() {
        TerminalFailureSource::Provider => {}
        TerminalFailureSource::Host => {}
        TerminalFailureSource::Runtime => {}
        _ => {}
    }
}
```

Terminal source and classification origin are independent. A
`ProviderFailed` terminal may carry transport-origin evidence. A
`RuntimeFailed` terminal may carry protocol-origin evidence.

## Unknown Is Supported

Every `SafeDiagnostic::new` has an explicit all-unknown classification. This
is the correct result when a harness reports only an opaque failure or exit.
Do not recover stronger meaning from its message, stderr, display content, or
provider prose.

Consumers should always retain a generic branch that shows the safe message
and records the exact safe code. Route-specific code matching is an optional
support escape hatch, not the normal application integration.

## Preparation And Cleanup

Preparation failures keep their exact `PreparationStage`. Read the safe
classification through `failure.diagnostic().safe()` and keep the stage in
application evidence.

Cleanup remains separate from terminal failure:

```rust
if let Some(cleanup) = outcome.cleanup().diagnostic() {
    // Preserve this alongside the primary result. Do not overwrite it.
}
```

Cancellation, timeout, detachment, provider requests, permission callbacks,
and typed questions are not portable failures.

Failures before a live handle are preparation failures. Failures after start
settle through `TerminalOutcome`. A callback response may itself fail while
the turn remains live or becomes terminal according to the exact route.
Cleanup remains observable after either success or failure. Preserve these
phases instead of collapsing them into one application error string.

## Warning And Error Activity

`ActivityObservation::diagnostic` is available only for
`ActivityKind::WarningOrError`. It may carry the same classification while the
operation continues. Activity display content remains operation data and must
not be parsed into an error class.

## Recovery Evidence

`FailureRecovery` records what the adapter can prove. It never authorizes an
automatic retry, fallback, provider switch, credential change, prompt rewrite,
or harness update. Those decisions remain consumer policy.

## Consumer Boundary

The consumer owns presentation, localization, logging retention, retry and
fallback policy, credential UX, and support escalation. Store the portable
origin, kind, recovery evidence, terminal source or preparation stage, exact
safe diagnostic code, and cleanup truth as separate fields.

Do not persist or display raw provider payloads, stderr, prompts, tool bodies,
credentials, endpoints, or private continuation state as diagnostics. Do not
parse safe messages into control flow. Exact diagnostic-code branches are
route-specific support behavior and must retain a generic fallback.

Swallowtail owns safe classification when qualified, redaction, terminal
source, preparation stage, and cleanup truth. It does not own an application
error taxonomy, retry scheduler, provider chooser, or sign-in flow.

## Examples And Conformance

Every route's compiling example in the
[integration guide map](integration-guide-map.md) uses the same prepared and
terminal failure boundary. Route fixture suites prove known classification,
unknown preservation, safe diagnostics, terminal source, and cleanup
separation without live provider work.

```sh
effigy check:examples
effigy qa:docs
effigy qa:routes
```

Authentication, rate limiting, allowance spend, and provider outages are not
required for deterministic acceptance. Optional live probes remain operator
gated.
