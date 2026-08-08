# Provider History Page Guide

Date: 2026-08-08
Roadmap: g03.057
Card: 178

## Outcome

Contract 054 history pages are documented for consumers and operators.

`docs/guides/provider-session-history.md` covers newest-first page shape,
cursors, Exact/AtLeast/Unknown totals, authority limits, and the Codex
app-server synthetic `thread/read` mapping. Key concepts, the Codex prepared
guide, route matrix, and integration-guide map point at it. No feature-CSV
column was added; support remains Codex-only and route-advertised.

Milestone g03.057 is complete. The generation returns to its evidence gate.

## Local Validation

- `effigy qa:docs`: passed
- `effigy qa:docs:index:logs`: covered by `qa:docs`
- `effigy qa:docs:index:roadmaps:batch-cards`: covered by `qa:docs`

## Boundaries

No additional adapter mappings, Nucleus transcript paging, live provider work,
tag, or release.
