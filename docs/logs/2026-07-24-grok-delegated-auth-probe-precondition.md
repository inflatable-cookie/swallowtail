# Grok Delegated Auth Probe Precondition

Date: 2026-07-24

## Outcome

The operator authorized card 138's narrow no-prompt delegated-auth probe. The
probe did not start because the required pre-existing authenticated Grok state
is absent from this host.

This is a precondition failure, not an authentication failure. No Grok release
is qualified.

## Evidence

- the exact direct `0.2.111` executable remains available
- its SHA-256 matches the card-137 frozen artifact
- no installed `grok` command is available
- the documented default Grok state directory is absent
- no API-key environment fallback is present or selected
- no credential file was read
- no agent, authentication, session, provider, model, or paid request ran

## Boundary

The authorization covered activation of an existing subscription credential.
It did not cover package installation, login, browser or device flow, API-key
access, or credential-mechanism switching. Creating state to satisfy the probe
would cross that boundary.

Spec 003 remains provisional. Card 138 remains blocked.

## Validation

- exact artifact and state precondition check stopped before process launch
- docs, Northstar, formatting, and diff checks passed
- doctor remains at the inherited 19 oversized-file findings:
  12 warnings and 7 errors

## Continuation

Resume card 138 only after either:

1. the operator independently installs exact `0.2.111` and signs in through
   the intended Grok subscription route, or
2. maintained xAI ACP authentication documentation matches the exact
   artifact.

Cards 139-141 remain planned and must not start early.
