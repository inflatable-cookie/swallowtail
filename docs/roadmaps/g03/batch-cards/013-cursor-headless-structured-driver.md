# 013 Cursor Headless Structured Driver

Status: planned
Owner: Tom
Created: 2026-07-31
Milestone: `../005-cursor-installed-dual-route-foundation.md`
Depends on: card 012

## Goal

Implement Cursor headless stream JSON as a separate structured-run driver with
explicit model, authority, activity, usage, and cleanup behavior.

## Scope

1. Build exact headless invocations from host-approved executable, model,
   operation mode, workspace, and output configuration.
2. Map system, user, assistant, tool-call, usage, error, and terminal events.
3. Support bounded read-only and explicit workspace-write profiles from exact
   Cursor controls.
4. Implement output schema where the exact corpus proves it, plus cancellation,
   deadlines, and joined process cleanup.
5. Keep optional Cursor sandboxing separate and never synthesize thinking
   events suppressed by print mode.

## Acceptance Criteria

- [ ] the headless route is distinct from ACP
- [ ] model and authority selection are explicit with no fallback
- [ ] streamed tool and result activity is ordered and correlated
- [ ] usage and errors remain typed, bounded, and sanitized
- [ ] optional sandboxing is not required by the ambient profile
- [ ] no thinking activity is invented
- [ ] focused structured-run and activity conformance passes

## Validation

- `effigy validate:focused swallowtail-adapter-cursor`
- focused structured, schema, authority, activity, usage, cancellation,
  deadline, and cleanup tests
- no broad workspace suite or live Cursor prompt

## Stop Conditions

- Stop if write authority cannot be bounded before process launch.
- Stop if the event stream cannot identify one terminal outcome exactly.
- Do not use dangerous or undocumented permission flags.

## Auto-Continuation

Yes. Continue to card 014 after focused headless validation passes.

