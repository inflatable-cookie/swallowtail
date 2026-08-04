# 2026-08-04 OpenAI Background Run Reconciliation And Detachment

Roadmap: `../roadmaps/g03/030-openai-background-run-reconciliation-and-detachment.md`
Cards: 076-078

## Changed

- added the distinct provider-run checkpoint and reconciliation capability,
  role, plan, request, observation, outcome, and runtime registration slot
- added bounded, versioned, integrity-checked persisted checkpoints binding the
  runtime run, provider run, opaque adapter cursor, and exact prepared route
- added prepared OpenAI background reconciliation through one exact read-only
  response retrieve request
- added opt-in structured-run detachment after checkpoint availability
- preserved default close, cancellation, deadline, and terminal response
  deletion behavior
- promoted exact-run recovery in Contracts 048-049, architecture, research,
  route guides, and the existing provider-managed-recovery matrix column

## Evidence

Deterministic tests prove:

- checkpoint opacity, integrity, version, size, and cross-route rejection
- exact active and completed response reconciliation with bounded output and usage
- foreign response and recovered-output-bound rejection
- cancellation and elapsed-deadline rejection before provider observation
- checkpoint-bearing events remain semantic even on a coalescible event kind
- idempotent detach plus cancellation and terminal rejection
- no cancel or delete request after detachment
- unchanged ordinary terminal response deletion
- frozen retained-execution request and lifecycle truth

Validation:

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-openai`
  — 235 tests passed; Clippy and package checks passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-openai`
  — all three extracted packages passed
- `effigy qa:docs` — passed
- `effigy qa:routes` — passed
- `cargo fmt --all -- --check` — passed
- `git diff --check` — passed

Official OpenAI documentation was rechecked. No authenticated provider work,
API request, or paid inference ran.

## Next

Execute g03 card 079. Qualify Claude Agent and Kimi ACP retained-history
reconciliation without treating retained history as a surviving live turn.
