# 022 OpenCode External Session Discovery And Import

Status: planned
Owner: Tom
Created: 2026-08-01
Depends on: g03.021
Vision tags: attached harness, HTTP session catalogue, external import
Contract refs: 009-011, 014, 017, 029, 037-038, 044, 046
Planning state: cards 058-060 planned

## Problem

OpenCode's attached HTTP server exposes session list, lookup, status, messages,
and continuation, but Swallowtail only loads or resumes sessions from existing
bindings. The complete list-to-import chain needs exact evidence across the
maintained server range.

## Goals

- [ ] freeze exact list, lookup, status, message, load, and resume closures
- [ ] preserve attached endpoint, directory, access, and version identity
- [ ] expose bounded resource-scoped session catalogue and explicit import
- [ ] reuse existing ordered replay and prompt continuation
- [ ] retain server ownership, deletion, and SSE lifecycle boundaries
- [ ] close with focused and extracted-package evidence

## Execution Plan

### Batch 22.1 — Exact OpenCode Range Corpus

- [ ] Execute card 058.
- [ ] freeze the selected OpenAPI closures and runtime behavior at every
  maintained segment
- [ ] record pagination, directory scoping, child-session, status, and message
  differences without widening the selected profile

### Batch 22.2 — HTTP Driver And Prepared Facade

- [ ] Execute card 059 after the corpus passes.
- [ ] implement list and exact candidate lookup through the approved endpoint
- [ ] bind directory/resource, access, server revision, model, and policy
- [ ] issue an imported binding and reuse existing load/resume behavior

### Batch 22.3 — Conformance And Acceptance

- [ ] Execute card 060 after card 059 passes.
- [ ] prove local and remote-authoritative host identity, attached preservation,
  Basic-auth lease cleanup, stale targets, deadlines, and cancellation
- [ ] update facade guidance, matrix truth, and extracted package evidence

## Boundaries

- no OpenCode server start, stop, update, or ownership
- no project scan or cross-directory listing outside approved scope
- no child-session import unless separately selected and qualified
- no session share, fork, rename, revert, summarize, archive, or implicit delete
- no raw HTTP body or message content in diagnostics
- no provider prompt, live network selector, consumer edit, or broad suite

## Acceptance Criteria

- [ ] every guaranteed OpenCode segment proves the complete import chain
- [ ] attached endpoint and directory identity cannot drift between list/import
- [ ] imported replay preserves exact message/part ordering and bounds
- [ ] deletion and provider-session management remain separate operations
- [ ] attached servers remain running after success, cancellation, and failure
- [ ] focused OpenCode and affected-package validation pass

## Next Planning Checkpoint

After card 060, enter g03.023 for remaining-route classification, public truth,
and the bounded Nucleus adoption handoff.
