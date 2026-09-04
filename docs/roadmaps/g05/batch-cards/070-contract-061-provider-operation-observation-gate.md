# 070 Contract 061 Provider-Operation Observation Public-Baseline Gate

Status: complete
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061; the Kimi active-observation gate note; the candidate I audit note; completed cards 022, 033, and 066; operator decision of 2026-09-04

## Goal

Fix the exact shared vocabulary that lets a completed provider operation
which opens no session publish observed truth honestly, so the recurring
`control.provider-session-catalogue` and `control.provider-session-history`
rows on `deepseek-harness.local-server`, `kimi-code.acp`, and `opencode.http`
can be projected without reinterpreting the session-scoped names. Planning
only: one triage gate note, zero Rust.

## Why Now

Three candidates carry the same gap. The Kimi gate stopped candidate F on
it, card 066 stopped candidate I on it with two rows, and the census places
the row on `opencode.http` in candidate L. The operator chose on 2026-09-04
to settle it once with at least two proving consumers rather than withhold
the rows on all three routes.

## Scope

1. Read the three session-scoped names on current `main`:
   `ConsumerRouteProjectionSourceKind::ActiveSessionObservation`
   (`identity.rs`), `ConsumerRouteLifecycle::PostOpenObservationOnly`
   (`semantics/posture.rs`), and `ConsumerRouteActiveSessionState`
   (`views.rs`). Leave their meaning untouched.
2. Propose exact additive shared names for a provider-operation observation:
   one source kind, one lifecycle band, and one view or view extension, with
   Rustdoc-grade one-line definitions, in `swallowtail-runtime` only. Name
   the fixed maximum each new collection needs, derived from the three
   carrier routes' high-water marks plus explicit headroom, following the
   Batch 9.1 method.
3. Define admission: which prepared operation shapes may anchor the new
   source kind (`ProviderSessionCatalogue`, `ProviderSessionHistory`, and no
   other on this gate), what the retained evidence must be (the completed
   outcome, never the prepared plan), and the exact failure when a session
   shape or prepared record tries to use it.
4. Define composition: how the new view composes with the three existing
   views, cross-access and source-identity agreement, replacement semantics,
   and why a prepared contribution and a provider-operation contribution for
   the same route never merge.
5. Name the two proving consumers and their exact rows:
   `deepseek-harness.local-server` `list_sessions` and `page_history`
   (`web/driver.rs`) for rows 44 and 45, and
   `KimiPreparedSessionCatalogue::list_sessions` for the Kimi row. Record
   `opencode.http` as the third carrier with its anchor, without auditing
   candidate L.
6. Draft the Contract 061 amendment text: the new source dimension, the
   lifecycle band, the view, and the fail-closed point it adds. Do not edit
   the contract; Chatterbox promotes it.
7. Draft the testkit assertions the runtime card must add: session-shape
   rejection, prepared-record rejection, bound enforcement, and honest
   descriptor-only state for the observed rows.
8. Apply the Batch 9.1 readiness rubric to the drafted baseline and state
   which decisions remain open for the operator, if any.
9. Write exactly one new triage note
   `docs/triage/YYYYMMDD-HHMMSS-contract-061-provider-operation-observation-gate.md`
   holding items 2-8. Fill this card's `## Result`.

## Out Of Scope

Rust, contract, architecture, census, or fixture edits; widening the gate to
per-turn, acknowledgement, negotiated-option, or catalogue-feature rows;
candidate L's audit; the compound acknowledgement shape on `kimi-code.acp`;
provider contact.

## Acceptance Criteria

- every proposed name is additive, one-line defined, and leaves the three
  session-scoped names byte-identical in meaning
- admission, composition, failure, and maxima are exact enough to write the
  runtime card without a second decision
- both proving consumers are anchored with code references
- the Contract 061 amendment and testkit assertions are drafted verbatim
- open operator decisions, if any, are listed as questions, not defaults
- one triage note exists; zero Rust

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the gate widens vocabulary additively and never by reinterpreting
an existing name. Smallest counterexample: a proposal that changes what
`ActiveSessionObservation`, `PostOpenObservationOnly`, or
`ConsumerRouteActiveSessionState` means for a route already using it, or a
prepared plan admitted as observation.

## Auto-Continuation

No. Stop after the note for coordinator closeout and Chatterbox promotion
into Contract 061 and the runtime baseline card.

## Result

Complete. The strict-ready planning gate is
[`docs/triage/20260904-161224-contract-061-provider-operation-observation-gate.md`](../../../triage/20260904-161224-contract-061-provider-operation-observation-gate.md),
resolved against current `main` at
`13df1599c96a455689ddea564c45d5a4ffbd4e9a`.

- Additive vocabulary: source kind `ProviderOperationObservation`, lifecycle
  `PostOperationObservationOnly`, fourth view
  `ConsumerRouteProviderOperationState`, typed successful-outcome reference,
  and separate `ConsumerRouteProviderOperationObservation` input.
- Admission accepts completed `ProviderSessionCatalogueOutcome` and
  `ProviderSessionHistoryPage` only. Prepared-only, session-shaped,
  mismatched, failed, and authority-widening evidence fails closed through
  `ProviderOperationObservationInvalid`.
- `MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS = 4`: carrier high-water 2 plus
  two rows / 100% headroom. Existing source, extension-count, and text maxima
  remain sufficient.
- Composition keeps prepared and provider-operation contributions in separate
  row collections and source identities under exact applicability and
  cross-access agreement. Equal rows with a new outcome source ID replace the
  snapshot.
- Proving anchors: DeepSeek Harness local-server rows 44/45 at
  `web_prepared.rs:301-437`, `:840-897`, `:1082-1140`, and
  `web/driver.rs:384-674`; Kimi ACP catalogue at
  `prepared_profile/provider_session_catalogue.rs:22-74` and
  `driver/session_catalogue.rs:49-115`.
- OpenCode HTTP is recorded only as the third catalogue carrier at
  `provider_sessions/catalogue.rs:16-65` and
  `driver/provider_session_import.rs:29-69`; candidate L is not audited.
- Draft Contract 061 replacement/addition text and six exact runtime/testkit
  assertions are included. All six readiness categories pass. Open decisions:
  none.
- Zero Rust and no edits outside this card and the one new triage note.
