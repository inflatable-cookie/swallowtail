# g04.024 Kimi Platform Chat Closeout

Date: 2026-08-21
Milestone: g04.024
Cards: 076, 077, 078
Status: complete

## Result

`kimi-platform.chat` is the seventh realized addable route, on the proved
hosted API-key shape:

- `kimi_platform_chat_addable_route_descriptor` exposes the Contract 057
  descriptor: hosted topology, the existing
  `swallowtail.kimi-platform.direct-chat` driver identity, one secret
  Platform API-key field `api_key` with no environment name, and one opaque
  host-owned `endpoint` config field. Availability reports the missing
  Credential host service by name.
- Contract 057 admission writes the record with an opaque endpoint
  `ConfigFieldRef`; API-key collection stores only the submitted
  `CredentialRef` and needs no URL-open, loopback, or device-code ports.
- `KimiPlatformPreparationInput::from_admitted` retypes the admitted refs
  into `prepare_kimi_platform_direct` and fails closed on route,
  endpoint-ref, credential-ref, audience, or host drift.
- `refresh_readiness` writes host-supplied access status without touching
  enablement; the authenticated subject stays `Absent`.
- The existing catalogue and one explicit K3 attempt prepare only after
  admission, keeping exact `moonshot` / `kimi-k3` identity, explicit
  reasoning selection, and the output bound. The consumer-assembled 047
  snapshot retains exact instance, route, provider, model, facade, and
  access truth; the overlay cannot change `Ready` / `NotReady`.

No secret bytes or endpoint values entered portable records or diagnostics.
Route inventory, Contracts 047 and 057, and `release-baselines/public-api-0.3.3`
are unchanged. The connection-lifecycle and Kimi Platform prepared guides
name the realized path; a compile-tested `connection_lifecycle` example
mirrors it.

## Validation

- Card 076: `effigy validate:focused swallowtail-adapter-kimi-platform
  swallowtail-runtime` — 247 tests passed; `git diff --check` — passed;
  `effigy package:api` — passed after adding
  `release-baselines/public-api-unreleased/swallowtail-adapter-kimi-platform.txt`.
- Card 077: `effigy validate:focused swallowtail-adapter-kimi-platform
  swallowtail-runtime swallowtail-host-local` — 299 tests passed;
  `git diff --check` — passed; `effigy package:api` — passed.
- Card 078: `effigy validate:focused swallowtail-adapter-kimi-platform
  swallowtail-runtime swallowtail-host-local swallowtail-testkit` — 396
  tests passed; `effigy package:verify-affected` for the same four packages
  — passed; `effigy qa:routes` — passed; `effigy qa:guides` — passed;
  `effigy qa:northstar` — passed; `effigy check:examples` — passed;
  `effigy package:api` — passed; `git diff --check` — passed.
- `effigy qa:docs:index:logs` — passed.
- Restacked head `a08c89a1`: GitHub Stable, documentation and semantic API,
  pinned MSRV, dependency policy, and external Git-source consumer jobs all
  passed. The first restacked run exposed only Rust 1.98 formatting drift in
  the new lifecycle test; commit `a08c89a1` contains the formatting-only fix.

The `effigy doctor` orientation keeps the inherited repository baseline:
348 god-file findings (40 errors, 308 warnings) and one generated-in-src
warning. No new finding appeared in this lane.

## Next

PR 31 fast-forwarded `main` to `a08c89a1`; merge reality is recorded. Resolve
the Pi RPC resource-bound session attachment decision from Research 180 before
compiling an implementation card. Gemini enterprise API-key currentness and
the per-route feature programme remain next in sequence.
