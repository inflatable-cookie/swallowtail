# 062 Kimi Local Server Lifecycle Driver

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../020-kimi-code-local-server-route.md`

## Objective

Implement native Kimi local-server archive and restore through explicit
attached and owned-foreground host composition.

## Governing Refs

- card 061
- Contracts 010, 014, 017, 029, 037-038

## Scope

1. Add attached preparation from one approved loopback endpoint, exact server
   metadata, Kimi state-root resource, and opaque bearer credential lease.
2. Add owned-foreground preparation from one approved executable and
   `kimi web --no-open`, with explicit loopback binding, readiness, metadata,
   and joined shutdown.
3. Keep provider login credentials separate from the local-server bearer.
4. Add native inactive-session archive and restore requests, plans, outcomes,
   and prepared bound operations.
5. Preserve the external server for attached execution and join only the child
   owned by Swallowtail.
6. Reject delete before dispatch.

## Acceptance Criteria

- [x] no implicit port, sibling instance, state root, token, or topology
- [x] authentication is never disabled
- [x] health alone cannot satisfy metadata or compatibility
- [x] archive and restore cross one exact effect boundary without retry
- [x] cancellation and deadline preserve before-effect versus unconfirmed
      after-effect truth
- [x] attached cleanup preserves the server; owned cleanup joins it
- [x] credential release follows joined transport work; owned child cleanup
      joins before its handle returns
- [x] no filesystem or descendant-process containment claim

## Evidence

- `swallowtail.kimi.local-server` now registers only
  `ProviderSessionManagement`.
- Attached preparation requires one host-approved loopback HTTP endpoint,
  exact executable/server metadata, one opaque Kimi state-root identity, and
  a secret lease under `kimi-code/local-server-bearer`.
- Owned preparation starts one approved executable with exactly
  `kimi web --no-open --host 127.0.0.1 --port <approved> --log-level info`.
  It parses only the readiness origin shape and port, then requires both
  health and authenticated metadata. It never reads Kimi's token file or
  passes `--dangerous-bypass-auth`.
- Archive and restore use native POST routes. Provider rejection is
  `FailedBeforeEffect`; transport, server, malformed-success, cancellation,
  or deadline uncertainty after dispatch is `UnconfirmedAfterEffect`.
- Delete remains unsupported before dispatch and is absent from the
  configured-instance capability profile.
- Deterministic fixtures cover attached local and remote-authoritative hosts,
  owned remote-authoritative startup, readiness mismatch cleanup, exact
  command arguments, provider rejection, and cancellation/deadline truth.

## Validation Evidence

Passed on 2026-07-27:

- `cargo clippy -p swallowtail-adapter-kimi --all-targets -- -D warnings`
- `cargo test -p swallowtail-adapter-kimi` — 39 passed, one live installed
  probe ignored
- `effigy check:rust`
- `effigy format:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes`

`effigy doctor` returns the pre-existing 32 oversized-file findings
(23 warnings, 9 errors). This batch introduced no finding.

`effigy package:api` returns the expected held-candidate diff. The new Kimi
prepared lifecycle API joins the unbaselined lifecycle surfaces from cards
046-061. Card 059 still owns baseline replacement after canonical source
history exists.

## Validation

- focused lifecycle driver and prepared-facade tests
- local and remote-authoritative host fixtures where applicable
- owned-child readiness, failure, and cleanup matrix
- Kimi ACP regression

## Stop Conditions

- the only usable path requires `--dangerous-bypass-auth`
- token acquisition requires exposing it through diagnostics or public values
- owned startup cannot bind one exact endpoint and child
- provider effects cannot be classified under Contract 038

## Auto-Continuation

Yes. Continue to card 063 after lifecycle driver validation passes.
