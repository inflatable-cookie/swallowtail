# 2026-08-04 Anthropic Managed Run Reconciliation And Cleanup

Roadmap: `../roadmaps/g03/033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Card: `../roadmaps/g03/batch-cards/085-anthropic-managed-run-reconciliation-and-cleanup.md`

## Changed

- added an opt-in recoverable Managed Agents run profile while preserving the
  ordinary profile's delete-on-close behavior
- emitted separate run-reconciliation and recovered-cleanup records after the
  exact environment and session exist, before submitting the user message
- restored those opaque records only against the exact prepared route
- added exact session retrieval plus at most eight persisted-event pages and
  2,048 events, with bounded cursors, cycle rejection, and contradiction checks
- mapped active, waiting, completed, failed, cancelled, inactive unresolved,
  and unknown truth from exact ordered events
- kept reconciliation read-only: no message, callback answer, retry, resume,
  interruption, stream attachment, or cleanup effect
- rechecked inactivity before cleanup, confirmed session deletion before
  environment deletion, and preserved partial or uncertain effect truth
- split recovery implementation and tests by concern; no new Rust source file
  exceeds 300 lines

## Evidence

Deterministic fixtures cover checkpoint-before-message ordering, single- and
two-page history, active-resource preservation, exact completion, interruption,
ambiguous termination, ordered cleanup, uncertain session deletion,
cancellation, and elapsed deadlines.

Validation:

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`
  — 188 tests passed; focused package check passed
- `cargo fmt --all`
- `git diff --check`

No authenticated provider work, paid inference, consumer edit, or live resource
deletion ran.

## Next Move

Execute card 086. Reconcile public route truth and consumer guidance, then run
focused, docs, and independently extracted-package acceptance.
