# g04.033 Pi SDK Sidecar Route Closeout

Date: 2026-08-21
Milestone: g04.033
Cards: 089, 090, 091, 092
Status: complete

## Result

`pi.sdk-sidecar` is the eighth realized addable route and Pi's second
production route, on a source-tagged Node sidecar over Pi's public SDK:

- Card 089 froze the source-tagged sidecar asset
  (`sidecar/pi-sdk-sidecar.mjs`, only public
  `@earendil-works/pi-coding-agent@0.84.2` exports) and the private
  strict LF-JSON wire `swallowtail-pi-sdk-jsonl-v1` with its bounded,
  correlated, fail-closed corpus. Ambient configuration, catalogue
  refresh, update checks, retry, and fallback stay suppressed by
  construction.
- Card 090 added the separate `swallowtail.pi.sdk-sidecar` driver with four
  qualified-only one-point claims (SDK package `0.84.2`, Node `22.23.2`,
  wire, sidecar source tag), fresh-session prompt/steer/follow-up/events/
  abort/joined-close parity with the RPC surface, catalogue from the
  explicitly constructed offline runtime, and bootstrap identity
  verification (wire, behavior, SDK, Node, leased cwd, provider, model,
  read-only tools) before any provider work.
- Card 091 realized Contract 017 persistent new, load, and resume: new
  sessions return the opaque SDK session reference and exact restart
  binding; load attaches through `session_switch` with the expected-cwd
  gate and substitution check, then completes bounded typed replay (1,024
  messages, 4 MiB) before readiness; resume attaches without replay.
  Durable provider state in the application-provisioned session directory
  is preserved on close; persisted bindings round-trip only under exact
  dimensions through the runtime's versioned opaque record.
- Card 092 admitted the route through Contract 057 with an
  adapter-local addable descriptor (installed topology, opaque launch
  recipe, environment, and delegated credential references, no sign-in
  action), the `from_admitted` prepared handoff into
  `prepare_pi_sdk_sidecar_session`, deterministic admission and drift
  tests, a compile-tested example, and the prepared-integration guide.

RPC disposition: retain both routes. The SDK sidecar is a feature superset
for session continuity (Research 180's attachment gate still blocks RPC
load/resume), but not an operational superset: `pi.rpc` needs only one
installed upstream executable and an upstream-owned wire, while
`pi.sdk-sidecar` requires the application to provision an exact Node
runtime, the source-tagged sidecar, and the exact SDK package over a
Swallowtail-owned private wire. Each route keeps a distinct useful posture;
neither substitutes for the other. The route matrix, feature matrix, both
Pi guides, the triage note, and this log record the decision.

No Node paths, session paths, environment values, or credential bytes
entered portable records or diagnostics.

## Validation

- Cards 089-092 each ran `effigy validate:focused` and
  `effigy package:verify-affected` over their affected packages, plus
  `effigy qa:routes`, `effigy package:api` after unreleased baseline
  refreshes, and `git diff --check` — all passed.
- Card 092 also ran `effigy qa:guides`, `effigy qa:northstar`,
  `effigy qa:docs`, and `effigy check:examples` — all passed. The
  consumer front-door route comparison keeps the tagged `v0.3.3` set
  immutable and admits exactly that set plus `pi.sdk-sidecar`; unrelated
  source-route additions fail the gate.
- Review remediation replaced portable session paths with opaque Pi session
  ids resolved uniquely and canonically inside the approved session directory,
  rolled back active-turn and attachment state when deadline-task creation
  fails, and made disposal or non-zero process exit fail cleanup. The final
  focused four-package run passed 456 tests; package verification, semantic
  API, route, guide, Northstar, docs, Node syntax, and diff checks also passed.
- `effigy doctor` keeps the inherited repository baseline: 41 god-file
  error findings, identical to `main`; no new error-level file.

## Next

Merge reality: worker PR pending; cards 089-092 are committed on the
`g04-033-pi-sdk-sidecar` branch (through `5e8a0ec8` plus this card's
working tree) and NOT merged. Next lane: Gemini CLI `0.56.0` enterprise
API-key requalification across ACP and headless, then the serial per-route
feature completion programme. Contract 029 currentness remains standing.
