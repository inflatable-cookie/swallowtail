# 2026-09-01 g05.009 Kimi Active-Observation Gate Stop

Status: complete; evidence stop; gate incomplete; candidate F not promoted
Owner: Tom
Date: 2026-09-01
Contracts: 037, 047, 057, 061

## Result

The operator answered all five `kimi-code.acp` decisions card 033 returned.
Four are realizable route-locally. The fifth is not, so the gate stops.

`KimiPreparedSessionCatalogue::list_sessions` runs a `ProviderSessionCatalogue`
operation and opens no session. Publishing its completed query as
`control.provider-session-catalogue` needs
`ConsumerRouteProjectionSourceKind::ActiveSessionObservation`,
`ConsumerRouteLifecycle::PostOpenObservationOnly`, and
`ConsumerRouteActiveSessionState` — and current `swallowtail-runtime` defines
all three as post-open **session** semantics. Exact
`ProviderSessionCatalogue` applicability stops cross-operation mixing, but it
does not make the source kind, lifecycle band, or view true. Restating those
shared names in adapter documentation would widen Contract 061 semantics for
every route already using them, without a runtime baseline change or an
operator decision. The blocker is missing shared vocabulary, so no arrangement
of adapter-local types resolves it. The projected catalogue seam is withdrawn.

A second, narrower item is also unresolved. The compound
`feature.active-session-reasoning-and-plan-ack` row has no fixed shape: encoding
the halves as `reasoning=`/`plan=` domain entries with row-level union flags
loses half-to-state association for a generic Contract 061 consumer, and typed
adapter accessors do not repair that without the downcast the contract forbids.
The earlier draft also set `with_pending()` for a Plan half that reasoning had
terminally rejected, but runtime documents that flag as proven pending
acknowledgement state and no Plan request was ever dispatched.

## What The Gate Still Fixes

Retained as evidence, not authorization: the bounded adapter-local
`KimiProviderValue` and its private 128-byte admission; the shared private open
lifecycle and the ordered cases 1-4; the two disjoint foreign/unretainable
branches; exact provider-effective effort under requested `"on"`; the
`open_session_with_projection` outcome and failure shapes minus the
acknowledgement accessors; the negotiated model-option observation; and the
operation-shape-scoped persistence split.

## Corrected Ledger Evidence

Re-derived from the reviewed census plus current `main`, with three defects
found in re-review repaired:

| Route | Census | Emitted | Withheld | Undecided |
| --- | ---: | ---: | ---: | ---: |
| `kimi-code.acp` | 25 | 21 | 3 | 1 |
| `kimi-code.headless` | 20 | 10 | 10 | 0 |
| `kimi-code.local-server` | 31 | 31 | 0 | 0 |
| `kimi-platform.chat` | 13 | 12 | 1 | 0 |
| **Total** | **89** | **74** | **14** | **1** |

The undecided row is `kimi-code.acp` `control.provider-session-catalogue`:
emitted and 75/14 if a provider-operation observation baseline is authorized,
withheld and 74/15 otherwise.

Two emitter corrections, neither changing totals. `control.load-session` and
`control.resume-session` are profile-conditional: `load_request` and
`resume_request` both call `reject_attachment_options`, so a profile that bound
reasoning or Plan cannot construct either request and must omit both controls.
`kimi-platform.chat` `control.model-selection` comes from
`KimiPlatformPreparedInferenceAttempt` alone, because `prepare_catalogue`
builds its plan with no model route.

## Current State

- g05.009 is `strict-paused`
- card 034 is `blocked`; no card is ready
- coverage remains 249 proved and 518 remaining; none of candidate F's 89 rows
  is proved
- candidate F is not promoted and no 89-row package completion is authorized
- no Rust, manifest, release baseline, contract, architecture, census, or
  provider claim changed
- no provider was contacted and no live probe ran

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Move

Two operator decisions. First, whether to broaden the shared Contract 061
vocabulary so a completed provider-operation query can be projected honestly,
or to leave `kimi-code.acp` `control.provider-session-catalogue` withheld as
unrepresentable. Second, an exact compound-acknowledgement shape that preserves
each half's state generically without adapter downcasts or invented pending
state. Decision 1 gates candidate F as a whole; decision 2 gates any
implementation card. Do not implement Kimi, promote another candidate, contact
a provider, or compile Batch 9.5 before both land.

## Authority

- [stopped gate](../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [blocked card 034](../roadmaps/g05/batch-cards/034-contract-061-kimi-package-completion.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
