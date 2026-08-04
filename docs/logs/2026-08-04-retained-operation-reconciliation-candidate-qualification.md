# 2026-08-04 Retained Operation Reconciliation Candidate Qualification

Roadmap: `../roadmaps/g03/032-retained-operation-reconciliation-candidate-gate.md`

## Changed

- qualified exact Gemini CLI `0.51.0..=0.52.0` retained transcript and listing
  behavior
- found `--list-sessions` runs summary generation, may invoke Gemini, and may
  mutate retained transcript metadata before returning metadata without
  terminal truth
- corrected Contracts 038 and architecture: that operation is not a read-only
  post-delete confirmation and cannot support confirmed `HistoryRemoved`
- qualified Anthropic Managed Agents exact session retrieval plus bounded
  persisted-event history as the strongest remaining read-only run candidate
- added provider-input wait to the contracted run-state vocabulary
- separated an Anthropic reconciliation checkpoint from explicit cleanup of
  the exact recovered driver-owned session and environment
- promoted Research 103 and compiled g03.033 cards 083-086

## Current State

Roadmap g03.032 and cards 080-082 are complete. Gemini remains supported for
headless runs but blocked for reconciliation. Its management truth repair is
the sole ready card.

Anthropic recovery is contracted but unrealized. The next batch removes the
Gemini false claim, adds portable recovered-resource cleanup authority, emits
the Managed Agents checkpoint before work can be lost, and realizes bounded
session/event reconciliation plus inactive-only cleanup.

No authenticated provider work, provider prompt, deletion, or consumer edit
ran during qualification.

## Validation

- `effigy qa:docs`
- `git diff --check`

## Next Move

Execute g03 card 083. Remove the stateful Gemini confirmation path and reconcile
runtime, prepared evidence, tests, and public route truth before implementing
Anthropic recovery.
