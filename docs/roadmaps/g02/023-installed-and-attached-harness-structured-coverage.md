# 023 Installed And Attached Harness Structured Coverage

Status: completed
Owner: Tom
Created: 2026-07-27
Depends on: g02.022
Vision tags: harness interaction, transport diversity, prepared facade
Contract refs: 004-015, 017, 023, 028-029, 032-034, 037, 039
Planning state: cards 074-076 completed

## Problem

Claude ACP, Pi RPC, Gemini CLI, and OpenCode HTTP can all complete one bounded
agent task. Consumers should not need to manually open a session, execute one
turn, translate callbacks, and close transport work when they want a
structured harness run.

## Goals

- [x] Add the provider-neutral projection assertion pack.
- [x] Add Claude Agent ACP one-turn structured execution.
- [x] Add Pi RPC one-turn structured execution.
- [x] Add OpenCode attached HTTP one-turn structured execution and session
      deletion.
- [x] Add a separately qualified Gemini CLI headless JSONL route.

## Non-Goals

- a universal prompt facade
- changing provider-native approval, question, or retention semantics
- sandboxing as an implicit prerequisite
- editing Nucleus or Soundcheck
- unqualified Claude Code print support

## Execution Plan

### Batch 23.1 — ACP Projection Foundation

- [x] Execute card 074 after card 073 closes.

### Batch 23.2 — RPC And Attached HTTP

- [x] Execute card 075 after the assertion pack passes.

### Batch 23.3 — Gemini Headless

- [x] Execute card 076 after exact headless range evidence is frozen.

## Acceptance Criteria

- [x] every route retains `HarnessInteraction`
- [x] callbacks remain correlated through `RunHandle`
- [x] cancellation or deadline stops the active turn, then joins session and
      transport cleanup
- [x] retention matches exact provider behavior
- [x] OpenCode deletes only the session created by the run
- [x] Gemini ACP and headless retain separate driver and transport identities
- [x] qualified version ranges and unverified-newer posture remain visible

## Decision Gates

- Stop a route if one-turn completion cannot be distinguished from transport
  close.
- Do not auto-approve provider requests that lack a qualified callback path.
- Do not group routes in the solution CSV until one public solution facade
  exposes explicit typed selection.

## Next Planning Checkpoint

Continue to Kimi-specific currentness and retained execution in g02.024.
