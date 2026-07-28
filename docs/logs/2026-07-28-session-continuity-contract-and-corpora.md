# 2026-07-28 Session Continuity Contract And Corpora

## Changed

- confirmed Contracts 009, 017, and 038 cover the first five continuity cells
- froze Codex app-server load/resume behavior across six exact segments
- froze Claude Agent ACP load/resume across ten qualified releases
- froze OpenCode load/resume across seven wire surfaces and twelve published
  segments
- added ordered load, replay-free resume, mismatch, ordering, bounds,
  cancellation, disconnect, redaction, and cleanup fixtures
- split stale Claude fixture capability truth into load, resume, close,
  delete, list, and fork
- promoted Research 052 and made card 094 ready

## Current State

No shared contract or production behavior changed. All selected routes have
deterministic corpus coverage. Codex `excludeTurns` starts at `0.129.0`;
older qualified resumes remain replay-free at the public boundary by bounding
and ignoring returned turns. Claude native close still retains provider
history. OpenCode attached cleanup still does not stop the server or delete
the session.

## Validation

- 60 focused corpus, compatibility, lifecycle, and adjacent protocol tests
  passed
- docs, Northstar, route-matrix, and diff-integrity checks passed
- no live account, credential, executable, provider request, container, or
  model server was used

## Next

Execute card 094: implement the five selected prepared continuity paths.
