# Codex Discovery Exit Diagnostics

Date: 2026-07-26
Card: `../roadmaps/g02/batch-cards/045-codex-discovery-exit-diagnostics.md`

## Outcome

Soundcheck exposed an opaque Codex installed-version failure after its PATH
selected a cmux wrapper. Swallowtail reported the stable failure code but lost
the process evidence needed to diagnose the wrapper exit.

Codex discovery now retains numeric exit status and a bounded sanitized stderr
excerpt under `swallowtail.codex.discovery_exit_failed`. Executable selection
remains host-owned. Swallowtail adds no cmux or generic wrapper rejection.

## Bounds

- stderr capture stops at 1 KiB
- the stable diagnostic exposes at most 240 sanitized characters plus a
  truncation marker
- control and ANSI material is removed
- path, assignment, credential, email-like, and long-token shapes are redacted
- exact-version stdout parsing, cleanup, and preparation-stage classification
  are unchanged

## Evidence

All 93 Codex adapter tests pass. New deterministic cases cover numeric status,
useful safe stderr, redaction, empty stderr, and capture-bound truncation. Full
`effigy qa` passes across all 22 production routes and the workspace.

## Current State

Card 045 is complete. Card 042 remains operator-paused. No provider call,
consumer mutation, workspace write, publication, push, tag, or release was
performed.

## Next

Await operator direction for the next application-proof batch. Do not resume
card 042 implicitly.
