# Grok Build Artifact And Authentication Gate

Date: 2026-07-24

## Outcome

Card 137 is complete. Research 031 freezes the exact Grok Build publication,
artifact, discovery, ACP, model, authentication, configuration, permission,
and state evidence. No release is qualified.

The first route is rebaselined from restrictive read-only execution to an
explicit `AmbientHost` harness relay. Provider permissions remain useful, but
they do not establish filesystem containment or a sandbox. Sandboxing stays a
separate opt-in capability.

## Exact Evidence

- launcher and darwin-arm64 registries contain 111 matching `0.2.x` versions:
  `0.2.0..=0.2.47` and `0.2.49..=0.2.111`
- `0.2.48` is unpublished
- launcher tags mark `0.2.111` as `latest` and `alpha`; platform tags instead
  mark `0.1.220` as `latest` and `0.2.111` as `alpha`
- exact `0.2.0` and `0.2.111` launcher and platform artifacts were inspected
  without installation
- both exact agents negotiate ACP wire version 1
- `0.2.0 --version` mutates empty local state and is not a safe discovery
  command
- direct `0.2.111 --no-auto-update --version` leaves empty local state
  unchanged and is the sole discovery candidate
- the exact agents advertise only `grok.com` without credentials, while
  maintained public ACP guidance demonstrates `cached_token` or `xai.api_key`
- unauthenticated `session/new` fails but still creates durable local session
  state

Exact transition releases for discovery behavior, bundled ACP SDK, model, and
reasoning options remain unknown. No continuous compatibility interval is
inferred.

## Contract Decision

Spec 003 records the unresolved activation-only delegated-authentication
boundary. Existing contracts do not authorize a generic ACP `authenticate`
call that might launch sign-in, refresh or replace credentials, call an
external helper, or switch to API-key access.

Card 138 is blocked pending either:

1. explicit operator authorization for one separately gated no-prompt probe
   against an already authenticated exact `0.2.111` state, or
2. maintained xAI documentation matching the exact artifact.

The probe must send no prompt or model request and must not expose token,
account, or raw credential material.

## Validation

- `cargo test -p swallowtail-protocol-acp` — 44 passed, including seven Grok
  corpus tests
- `cargo clippy -p swallowtail-protocol-acp --all-targets -- -D warnings` —
  passed
- formatting, docs, Northstar, and diff checks passed
- `effigy doctor` remains at the inherited 19 oversized-file findings:
  12 warnings and 7 errors

## Continuation

- card 138: blocked delegated-authentication and ambient-access qualification
- cards 139-141: planned discovery, production driver, and portability
  closeout
- sole next task: resolve card 138 without changing credential mechanism
