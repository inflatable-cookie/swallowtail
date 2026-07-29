# 2026-07-29 Codex App-Server Activity Projection

## Changed

- Added the production Codex app-server observable-activity projector.
- Published exact activity capability and profile evidence from prepared
  read-only and bounded-workspace sessions.
- Added provider-unspecified assistant phase for older qualified messages.
- Kept callback, provider request, approval, and tool completion separate.

## Current State

- Native item ids own started, updated, and completed observations.
- Commentary and final-answer deltas stay distinct from legacy unclassified
  assistant items.
- Readable reasoning summaries are portable; raw reasoning is excluded.
- Command output retains command, cwd, exit status, and duration. File changes
  retain current replacement diffs and turn diff snapshots.
- Unknown semantic items use bounded Codex namespaces without raw payloads.
- Stable Codex `0.146.0` remains an allowed unverified attempt on the
  `0.145.0` activity guarantee.
- Access, workspace, cancellation, deadline, session, and cleanup behavior is
  unchanged.

## Evidence

- three projector corpus tests
- fifteen app-server integration tests
- nineteen prepared facade tests across activity milestones
- fourteen runtime activity and event-buffer tests
- complete 120-test Codex adapter suite
- workspace compile and lint gates
- public API declaration baseline
- doctor unchanged at 111 known structural findings after projector and
  prepared-session module splits

## Next

Card 124 maps the separate Codex exec JSONL lifecycle and closes roadmap
g02.036.
