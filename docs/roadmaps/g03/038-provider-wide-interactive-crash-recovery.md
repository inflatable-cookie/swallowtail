# 038 Provider-Wide Interactive Crash Recovery

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.035
Vision tags: restart continuity, exact attachment, usable recovery
Contract refs: 017, 037, 042, 048, 050
Planning state: completed; g03.036 card 093 restored as next

## Problem

The common restoration facade covers seven strong routes but leaves four
connected harness routes without any usable post-crash action. Cursor and Grok
can establish exact live sessions while failing the stronger complete-replay
gate. Other interactive harnesses can safely create a replacement session but
cannot restore provider context.

## Generation Runway Goal

Give every prepared interactive harness route one truthful restart action
without retrying one-shot work or flattening recovery strength.

## Goals

- [x] add exact attachment recovery with non-authoritative replay discarded
- [x] add fresh-session replacement with explicit provider-context loss
- [x] map Cursor and Grok to exact attachment recovery
- [x] map Antigravity, Gemini ACP, Pi, and Qwen to replacement
- [x] keep stronger routes and one-shot route behavior unchanged

## Execution Plan

- [x] card 102: realize the portable contract, binding, outcomes, and common
      conformance
- [x] card 103: implement Cursor and Grok exact attachment recovery
- [x] card 104: implement fresh replacement for the remaining interactive
      harness routes
- [x] card 105: qualify whether Gemini ACP can advance from replacement to
      exact attachment recovery
- [x] card 106: close provider-wide route, guide, and package acceptance

## Boundaries

- no prompt, transcript, tool, or side-effect replay
- no dynamic fallback after dispatch
- no complete-replay claim from discarded ACP updates
- no terminal-state inference for the interrupted turn
- no automatic retry for one-prompt headless routes
- no provider, model, credential, route, resource, or access substitution

## Acceptance Criteria

- [x] all 11 prepared interactive harness routes expose one exact method
- [x] reattachment returns the exact provider session and no replay
- [x] replacement returns a new session and explicit context-loss truth
- [x] persisted bindings support exact model-less preparation without a
      synthetic model
- [x] foreign, malformed, oversized, late, cancelled, and disconnected attach
      paths return no handle
- [x] focused and affected-package validation pass without authenticated work

## Lane Runway

Milestone complete. Resume g03.036 card 093; g03.037 remains planned behind
g03.036.
