Close verified correctness, safety, and lifecycle defects in core, idioms, and
runtime.

Core: reject non-canonical integer (`+123`, `0123`) and calendar (`2026-+1-+2`)
version text, and compare exclusions at the scheme's version point so a
build-metadata spelling cannot bypass one. Preflight now validates remote ACP
requirements (interactive harness role, task/blocking-work/time/network
services, unauthenticated first proof) under a new
`PreflightDimension::RemoteAcp`; session-access and provider-state checks read
every matching capability declaration instead of only the first;
`ModelManifestDigest` normalizes hex to lowercase.

Idioms: a registry merge keeps fresh incoming `as_of` and provenance on an
unchanged confidence outcome; `BoundedTextError`, `ConfidenceErrorKind`, and
`LintIssue` are re-exported and implement `Display` and `Error`.

Runtime: `SecretLease` zeroizes through `Zeroize`; the registered-tool kernel
releases its call slot through an RAII guard so a dropped `issue` future cannot
jam it, while a dispatch the host had already started stays counted as an
abandoned, unjoined call so teardown still reports `TeardownFailed` instead of
a clean close; duration bounds convert through the one documented monotonic-
tick boundary (`MonotonicInstant::saturating_add_duration`); event-channel,
terminal-outcome, and cancellation notifications wake outside their locks,
single-consumer waits hold one waker, and drop paths recover poisoned locks;
late-event quarantine evidence is bounded; installed-executable discovery
bounds its process-exit wait with the request deadline and cancellation; the
idioms fold never exceeds its byte bound and adds no leading newline on a
first-line overflow, with new `resolve_idiom_instructions` and discovery
coverage.

Public API: additive `PreflightDimension::RemoteAcp`,
`MonotonicInstant::saturating_add_duration`, `MAX_QUARANTINED_LATE_EVENTS`, and
the two re-exported error enums; no existing item changed shape.
