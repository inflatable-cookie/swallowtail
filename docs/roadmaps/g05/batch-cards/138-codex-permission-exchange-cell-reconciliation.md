# 138 Codex Permission Exchange Cell Reconciliation

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: the 2026-09-07 Desktop reconciliation capsule; card 129's matrix classification

## Defect

Two Swallowtail statements about the same capability disagree, and a consumer
acting on either one gets a different answer.

- The feature matrix marks `codex.app-server; codex.exec`
  `permission_exchange` as `No`, kind `provider_limitation`, citing
  "no frozen provider or harness support on this route".
- The reconciliation capsule sent to Bovine Desktop said Codex permissions are
  merged and answerable, naming `codex_approval_request_extension()`,
  `codex_user_input_request_extension()`,
  `CodexSessionProfileInput::with_user_input_exchange()`, and
  `CallbackResponder::respond`, and told Desktop its `ObservedOnly` posture is
  its own ceiling rather than the producer's.

`codex_approval_request_extension()` exists in
`crates/swallowtail-adapter-codex/src/session_access.rs`, and the same row
carries `question_exchange: Yes`. So the flat `provider_limitation` reading is
at best imprecise: something approval-shaped is exposed.

## Question To Settle

On the qualified app-server range, can a consumer **answer** an approval
request and have that answer reach Codex, or can it only observe one? If it
can answer, `provider_limitation` is wrong. If it can only observe, the cell
is right but its reason text must say that approvals are observable through a
bounded namespaced extension and not answerable, so nobody reads it as
"Codex has no approvals".

Also settle whether combining `codex.app-server` and `codex.exec` into one row
hides a difference: if app-server can answer and exec cannot, the combined
cell must not report the weaker of the two without saying so.

## Scope

1. Determine the answer from the frozen app-server corpus and the adapter
   source, with anchors; no live Codex.
2. Correct whichever surface is wrong: the matrix cell and its reason, or the
   capsule's claim (in which case Chatterbox re-relays a correction to
   Desktop).
3. If the two Codex routes differ, state how the combined row represents that,
   or split the reporting.

## Out Of Scope

Building an answerable exchange that does not exist; live Codex; Claude and
Grok routes.

## Acceptance Criteria

- [ ] one answer, anchored, for observe-only versus answerable
- [ ] matrix cell, kind, and reason text agree with the adapter source
- [ ] any combined-row ambiguity between app-server and exec is stated
- [ ] if the capsule was wrong, the correction to Desktop is drafted for Chatterbox

## Validation

- `effigy qa:routes`; `effigy qa:docs`; `git diff --check`

## Review Oracle

Invariant: the matrix and what we tell consumers say the same thing about the
same capability. Smallest counterexample: a cell that reads
`provider_limitation` for a capability the adapter exposes.

## Auto-Continuation

No. Stop for exact-head review.
