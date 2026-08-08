# Paged Provider Session History Planning

Date: 2026-08-08
Roadmap: g03.057

## Outcome

Promoted newest-first provider-session history paging into Swallowtail
authority without collapsing load or reconciliation:

- Research 114
- Spec 005 (promoted)
- Contract 054 plus amendments to 017 and 048
- Architecture note
- Milestone g03.057 with ready cards 176-178

Shared substrate: `SessionReplayItem`, bounds, opaque cursors, honest totals.
Separate operations: load stays complete-before-ready; reconciliation stays
observe-only replacement snapshots; history pages are read-only browse.

Codex first proof uses synthetic pages over bounded `thread/read` until native
turn pagination is separately qualified.

## Validation

Docs planning surfaces only; no runtime package validation in this closeout.

## Next

Card 176 — runtime history-page vocabulary and role.
