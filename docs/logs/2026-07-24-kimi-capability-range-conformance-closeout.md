# Kimi Capability Range Conformance Closeout

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/131-kimi-capability-range-conformance-and-closeout.md`

## Outcome

Kimi Code now proves two exact qualified behavior points behind one public
interactive-session shape:

- `0.28.1` selects the legacy boolean reasoning option
- `0.29.0` selects declared effort levels and legacy aliases
- later exact stable releases execute through the latest qualified behavior as
  visibly unverified newer
- older, malformed, prerelease, missing, ambiguous, and drifting observations
  reject without fallback

Installed discovery and interactive execution pass under local and
remote-authoritative host identities. Initialization corroborates the
host-observed executable release before session allocation.

The unchanged persistent ACP profile still covers new, load, resume, replay,
bounded writes, delegated authentication, cancellation, disconnect, redaction,
and joined cleanup when negotiated options are empty. Contract 034 assertions
cover exact request-plan-capability-lifecycle agreement and effective
confirmation when reasoning is requested.

## Preserved Boundaries

- no container, sandbox, or implicit containment claim
- no generic provider configuration surface
- no provider-specific option record in core or runtime
- no executable search, installation, update, downgrade, or ambient fallback
- no live authentication in default tests
- no Nucleus or Soundcheck changes

## Validation

- focused Kimi, runtime, and testkit tests: passed
- focused warnings-denied clippy: passed
- full repository QA: 606 inventoried, 602 passed, four gated probes ignored
- doctor: inherited 19 findings, 12 warnings and seven errors

The shared local Cargo target remains structurally pathological, with 65,535
entries in its dependency directory. Validation uses an isolated temporary
target. Repository output paths and runtime behavior are unchanged.

## Continuation

Roadmap 043 and cards 129-131 are complete. Roadmap 044 and card 132 return to
provider-coverage evidence without preselecting a provider or transport.
