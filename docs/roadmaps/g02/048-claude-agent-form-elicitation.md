# 048 Claude Agent Form Elicitation

Status: completed
Owner: Tom
Created: 2026-07-30
Depends on: g02.047
Vision tags: callbacks, typed user input, ACP
Contract refs: 009, 012, 015, 029, 037, 041
Planning state: cards 162-164 compiled

## Problem

Claude Agent ACP suppresses `AskUserQuestion` unless the client advertises
form elicitation. Swallowtail neither advertised the capability nor handled
the resulting client request, leaving an already-portable runtime callback
unreachable.

## Generation Runway Goal

Make Claude choice questions reachable through the same typed callback
surface used by Codex, Kimi, OpenCode, and Pi without claiming arbitrary ACP
form rendering.

## Goals

- [x] revalidate current ACP and claude-agent-acp form behavior
- [x] settle the proposed context field against end-to-end evidence
- [x] promote the capability and lossless-subset boundary
- [x] add provider-neutral ACP fixtures
- [x] realize Claude typed question exchange
- [x] prove response, failure, cancellation, version, and cleanup behavior
- [x] close with one clear next task

## Non-Goals

- URL elicitation
- arbitrary MCP forms
- refusal-fallback model policy
- provider option previews
- invented question context
- consumer UI or repository edits
- live provider calls, publication, or candidate replacement

## Execution Plan

### Batch 48.1 — Evidence, Contract, And Corpus

- [x] Execute card 162.
- [x] freeze capability and wire method
- [x] freeze historical and current AskUserQuestion schema revisions
- [x] record why context cannot cross this bridge

### Batch 48.2 — Adapter Exchange

- [x] Execute card 163.
- [x] advertise form support
- [x] project supported forms into typed callbacks
- [x] translate typed answers into elicitation content
- [x] decline richer forms

### Batch 48.3 — Acceptance And Closeout

- [x] Execute card 164.
- [x] prove focused source and extracted-package behavior
- [x] update architecture and currentness
- [x] record remaining unstable-protocol and presentation gaps

## Decision Gates

- Do not claim arbitrary ACP form rendering.
- Do not parse context from question prose.
- Do not expose raw form payloads or answers in diagnostics.
- Stop if capability advertisement cannot agree with callback availability.
- Do not run the full workspace suite.
