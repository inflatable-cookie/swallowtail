# 090 Pi SDK Sidecar Fresh Driver

Status: completed
Owner: Tom
Created: 2026-08-21
Milestone: `../033-pi-sdk-sidecar-route.md`
Depends on: card 089

## Goal

Add a separate production Rust driver for fresh Pi SDK sidecar sessions with
the existing Pi route's useful operation surface and exact identity truth.

## Scope

1. Add `swallowtail.pi.sdk-sidecar` and route `pi.sdk-sidecar` inside the Pi
   adapter without changing `swallowtail.pi.rpc`.
2. Bind the host-approved launch recipe, exact Node runtime, source-tagged
   sidecar, `swallowtail-pi-sdk-jsonl-v1`, exact SDK package, provider, model,
   cwd, environment, credential, and execution host during preflight.
3. Add a qualified-only one-point `pi.sdk-sidecar.package` claim for `0.84.2`;
   do not inherit the RPC range or unverified-newer posture.
4. Project fresh-session prompt, steer, follow-up, ordered output, stop, usage,
   provider failure, transport failure, abort, and joined close.
5. Preserve explicit read-only tool and `AmbientHost` posture.
6. Expose model catalogue only from the explicitly constructed runtime and
   retain exact provider/model identities.
7. Prove bounds, backpressure, unknown-message failure, redaction, deadline,
   cancellation, process join, and credential cleanup ordering.

## Out Of Scope

- stored-session new/load/resume (card 091)
- connection lifecycle and production route admission (card 092)
- write tools, shell, ambient configuration, retry, fallback, or containment
- changing or removing the RPC driver

## Acceptance Criteria

- the SDK sidecar is an independently selected driver and route
- fresh operations preserve the named Pi parity surface without SDK types
  crossing the adapter boundary
- missing or mismatched runtime, sidecar, wire, package, resource, provider,
  model, or host identity fails before provider work
- cancellation and close join the sidecar before leases are released
- all default tests use frozen fixtures

## Validation

- `effigy validate:focused swallowtail-adapter-pi swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-pi swallowtail-runtime swallowtail-testkit`
- `effigy qa:routes`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

Yes, into card 091.

## Stop Conditions

- Stop if another route is selected implicitly on failure.
- Stop if a sidecar or SDK task can outlive operation close.
- Stop if the driver needs raw secret access outside the approved host seam.
- Stop if `AmbientHost` would be presented as containment.
