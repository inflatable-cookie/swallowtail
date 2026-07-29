# Provider Retention Audit And Corpus Gate

Date: 2026-07-28
Cards: g02 099-100

## Changed

- classified all 75 archive, restore, delete, and owned-cleanup `No` cells
- found 58 operation-shape non-applicabilities and twelve exact absences
- found the stale OpenCode cleanup false negative
- qualified Gemini CLI stored-transcript deletion across exact
  `0.51.0..=0.52.0` source
- promoted optional Claude Agent operation-private cleanup
- promoted OpenAI terminal background-response cleanup
- froze three deterministic offline corpus manifests

## Key Evidence

Gemini's tagged `0.51.0` documentation omits the delete flag, but exact source
contains the same implementation and source digests as `0.52.0`. The command
can exit zero after rejection and its storage helper catches some unlink
errors. Swallowtail may claim only `HistoryRemoved`, after one bounded
`--list-sessions` reconciliation proves the exact bound id absent.

OpenAI's current official Responses API exposes deletion by response id.
Background response data is temporarily retained even with `store=false`.
This is operation-owned response cleanup, not provider-session management or
ZDR.

Kimi deletion remains unsupported.

## Validation

- JSON corpus syntax passed
- provider route and 22-solution matrix gate passed
- Effigy docs QA passed
- Effigy Northstar QA passed
- diff check passed

## Next

Card 101 implements four selected runtime cells. Card 102 then corrects the
OpenCode false negative, 58 non-applicable values, and final matrix counts.
