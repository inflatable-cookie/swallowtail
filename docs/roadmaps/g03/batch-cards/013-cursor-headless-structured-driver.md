# 013 Cursor Headless Structured Driver

Status: completed
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
5. Keep optional Cursor sandboxing separate and project only exact
   provider-disclosed thinking events.

## Acceptance Criteria

- [x] the headless route is distinct from ACP
- [x] model and authority selection are explicit with no fallback
- [x] streamed tool and result activity is ordered and correlated
- [x] usage and errors remain typed, bounded, and sanitized
- [x] optional sandboxing is not required by the ambient profile
- [x] no thinking activity is invented
- [x] focused structured-run and activity conformance passes

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

Completed. Continue to card 014.

## Result

Research 077 corrects the earlier print-mode assumption from exact installed
source: plain `stream-json` emits provider-disclosed thinking deltas and a
completion marker. `CursorHeadlessDriver` now projects those exact deltas,
assistant output, correlated tool lifecycle, terminal result, and usage while
keeping raw tool payloads and stderr private.

Read-only runs add `--mode plan`; explicit read-write runs omit that mode.
Both send the prompt through stdin and select neither force flags, optional
sandboxing, nor partial-output duplication. Non-zero exit, incomplete stream,
malformed stream, cancellation, deadline, local and remote-authoritative
topology, and joined cleanup are deterministic.

`effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor`
passed 116 tests across seven binaries plus warnings-denied checking. The
Cursor-only subset contains 27 tests across four binaries. No live Cursor
prompt ran.
