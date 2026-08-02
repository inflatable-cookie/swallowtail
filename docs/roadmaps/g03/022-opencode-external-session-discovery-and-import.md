# 022 OpenCode External Session Discovery And Import

Status: completed
Owner: Tom
Created: 2026-08-01
Depends on: g03.021
Vision tags: attached harness, HTTP session catalogue, external import
Contract refs: 009-011, 014, 017, 029, 037-038, 044, 046
Planning state: cards 058-060 completed

## Problem

OpenCode's attached HTTP server exposes session list, lookup, status, messages,
and continuation, but Swallowtail only loads or resumes sessions from existing
bindings. The complete list-to-import chain needs exact evidence across the
maintained server range.

## Goals

- [x] freeze exact list, lookup, status, message, load, and resume closures
- [x] preserve attached endpoint, directory, access, and version identity
- [x] expose bounded resource-scoped session catalogue and explicit import
- [x] reuse existing ordered replay and prompt continuation
- [x] retain server ownership, deletion, and SSE lifecycle boundaries
- [x] close with focused and extracted-package evidence

## Execution Plan

### Batch 22.1 — Exact OpenCode Range Corpus

- [x] Execute card 058.
- [x] freeze the selected OpenAPI closures and runtime behavior at every
  maintained segment
- [x] record pagination, directory scoping, child-session, status, and message
  differences without widening the selected profile

### Batch 22.2 — HTTP Driver And Prepared Facade

- [x] Execute card 059 after the corpus passes.
- [x] implement list and exact candidate lookup through the approved endpoint
- [x] bind directory/resource, access, server revision, model, and policy
- [x] issue an imported binding and reuse existing load/resume behavior

### Batch 22.3 — Conformance And Acceptance

- [x] Execute card 060 after card 059 passes.
- [x] prove local and remote-authoritative host identity, attached preservation,
  Basic-auth lease cleanup, stale targets, deadlines, and cancellation
- [x] update facade guidance, matrix truth, and extracted package evidence

## Boundaries

- no OpenCode server start, stop, update, or ownership
- no project scan or cross-directory listing outside approved scope
- no child-session import unless separately selected and qualified
- no session share, fork, rename, revert, summarize, archive, or implicit delete
- no raw HTTP body or message content in diagnostics
- no provider prompt, live network selector, consumer edit, or broad suite

## Acceptance Criteria

- [x] every guaranteed OpenCode segment proves the complete import chain
- [x] attached endpoint and directory identity cannot drift between list/import
- [x] imported replay preserves exact message/part ordering and bounds
- [x] deletion and provider-session management remain separate operations
- [x] attached servers remain running after success, cancellation, and failure
- [x] focused OpenCode and affected-package validation pass

## Next Planning Checkpoint

After card 060, enter g03.023 for remaining-route classification, public truth,
and the bounded Nucleus adoption handoff.
