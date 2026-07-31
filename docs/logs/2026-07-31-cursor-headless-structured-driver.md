# 2026-07-31 Cursor Headless Structured Driver

## Changed

- promoted Research 077 from exact installed Cursor source
- corrected the earlier assumption that print mode suppresses thinking
- added a distinct headless descriptor, compatibility claim, discovery role,
  structured driver, command builder, parser, activity projection, and joined
  cancellation handle
- bound explicit model, host-approved executable, workspace, read-only versus
  read-write authority, ambient configuration, durable provider retention, and
  deadline before process launch
- projected assistant text, provider-disclosed reasoning summaries, correlated
  tool lifecycle, result, and usage without raw tool payloads or stderr
- added deterministic read, write, cancellation, deadline, failure, malformed,
  local-authoritative, and remote-authoritative fixtures

## Boundary

The route is qualified only for installed Cursor Agent
`2026.07.01-41b2de7`. Later releases remain visible as unverified newer.

Read-only runs use `--mode plan`. Explicit write runs use Cursor's default
mode. Neither route selects force flags, optional sandboxing, nor partial
assistant output. No containment claim is made. No live provider prompt ran.

## Validation

`effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor`
passed 116 tests across seven binaries plus warnings-denied checking in one
second. The Cursor-only subset contains 27 tests across four binaries.

## Current State

Card 013 is complete. Cursor now has three distinct production drivers. Card
014 is the sole next task and owns their explicit prepared facade, package
acceptance, matrix reconciliation, and roadmap closeout.
