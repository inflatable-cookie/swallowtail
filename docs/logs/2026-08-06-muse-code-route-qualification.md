# Muse Code Route Qualification

Date: 2026-08-06
Roadmap: g03.045
Research: 112

## Decision

Promote exact Muse Code `0.1.0-R708.1` to a dedicated installed headless route
milestone. Do not treat every new harness as route-worthy; Muse qualifies as
Meta's first-party Spark harness with an explicit event JSONL protocol and a
deterministic echo provider.

## Evidence

- installed launcher and exact signed Apple Silicon payload inspected
- direct payload version and echo execution passed
- `exec --json` exposed stable envelope schema version 1, correlated session,
  run and task streams, output deltas, task lifecycle, and explicit terminal
- one operator-authorized Meta-account probe completed `muse-spark-1.2` at low
  reasoning with one model step, no tools, no writes, no shell, no web, and no
  retained session log
- the launcher is auto-updating; the route therefore binds the exact versioned
  payload instead of inheriting launcher mutation

## Current State

Roadmap g03.045 and cards 135-138 define the route runway. Card 135 is ready.
No Rust package, production route, consumer file, tag, or release identity
changed in this qualification batch.

## Next Move

Execute card 135: freeze sanitized exact artifact and JSONL corpora before
writing the driver.
