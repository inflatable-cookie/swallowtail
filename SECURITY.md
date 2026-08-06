# Security Policy

## Supported Versions

The tagged `0.1.x` source line is supported once released. Unreleased branches,
local modifications, unqualified provider versions, and historical candidate
bundles receive best-effort review but are not supported release identities.

Swallowtail's initial verified target is Apple Silicon macOS. A report on an
unverified target is still welcome; target support will be stated honestly.

## Report A Vulnerability

Report vulnerabilities privately through
[GitHub Security Advisories](https://github.com/inflatable-cookie/swallowtail/security/advisories/new).
Do not open a public issue for an undisclosed vulnerability.

Include only the evidence needed to reproduce and classify the issue:

- exact Swallowtail tag or commit
- affected package and route ID
- provider or harness version when relevant
- target and Rust version
- safe diagnostic codes and redacted steps
- security impact and the boundary crossed

Do not include credentials, tokens, authentication state, private prompts,
consumer data, raw provider payloads, or private endpoints. If sensitive test
material is required, agree on a safe exchange before sending it.

No response-time or embargo deadline is promised. The maintainers will
acknowledge, validate, classify, and coordinate disclosure as capacity allows.

## Scope

Security-relevant Swallowtail boundaries include:

- credential and secret redaction
- executable, environment, endpoint, and working-resource admission
- callback correlation and exactly-once answers
- provider-session and child-operation ownership
- cancellation, terminal truth, cleanup, and retained provider state
- protocol and event bounds
- dependency advisories, licenses, and sources

Provider outages, account disputes, pricing, model behavior, prompt injection
inside consumer-owned content, and vulnerabilities in an unmodified external
harness should normally be reported to that provider. Report them here when a
Swallowtail boundary makes the impact possible or materially worse.

## Fix And Disclosure Posture

Before 1.0, compatible security fixes advance the patch version when documented
behavior remains intact. A required breaking fix advances the minor version
unless an operator-approved urgent exception records compatibility loss,
rollback, and upgrade guidance.

A failed release tag is never moved. The fix lands in a later version.
