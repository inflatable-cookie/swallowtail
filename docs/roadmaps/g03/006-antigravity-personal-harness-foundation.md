# 006 Antigravity Personal Harness Foundation

Status: planned
Owner: Tom
Created: 2026-07-31
Depends on: g03.005
Vision tags: harness breadth, Google personal access, structured execution, continuation
Contract refs: 005-006, 011, 020, 023, 029, 032-033, 037, 039, 043-045
Planning state: cards 015-018 planned

## Problem

Google moved personal Google AI subscription access from Gemini CLI to
Antigravity CLI. Swallowtail retains Gemini for enterprise and paid API-key
postures, but lacks the installed harness now intended for personal Google
accounts.

Antigravity exposes model listing, structured and streamed headless output,
schema-constrained results, tool and subagent activity, usage, and explicit
conversation-id continuation. It is a separate integration family, not a
Gemini transport alias.

## Generation Runway Goal

Add Google's current personal-account harness without weakening access,
permission, sandbox, activity, or integration identity boundaries.

## Goals

- [ ] reconcile the official version page, tags, installed artifact, and source
- [ ] add separate catalogue and headless routes behind `prepare_antigravity`
- [ ] preserve Google personal and enterprise access as distinct profiles
- [ ] project stream steps, tools, subagents, usage, and results faithfully
- [ ] support exact conversation-id continuation without global latest-session
  selection
- [ ] keep permission bypass prohibited and sandboxing optional
- [ ] accept through deterministic and focused package evidence

## Non-Goals

- renaming, replacing, or falling back from Gemini
- claiming a machine-readable interactive TUI callback protocol
- automatic approval of provider tool requests
- dangerous permission bypass
- implicit sandboxing
- credential extraction or cross-use between Google access postures
- provider-owned archive or deletion without exact evidence
- consumer edits or publication

## Execution Plan

### Batch 6.1 — Artifact, Discovery, And Catalogue Corpus

- [ ] Execute card 015 after roadmap g03.005 closes.
- [ ] reconcile `1.1.8`/`1.1.9` evidence and freeze an exact artifact
- [ ] capture version, authentication posture, and model-list fixtures
- [ ] add identity-safe discovery and catalogue behavior

### Batch 6.2 — Structured Headless Execution

- [ ] Execute card 016.
- [ ] implement JSON, stream-JSON, schema, model, effort, usage, and activity
  projection
- [ ] preserve permission and optional-sandbox truth

### Batch 6.3 — Exact Turn-Scoped Continuation

- [ ] Execute card 017.
- [ ] bind continuation to the returned conversation id
- [ ] keep each turn a joined owned process with explicit cancellation and
  deadline handling

### Batch 6.4 — Prepared Facade And Acceptance

- [ ] Execute card 018.
- [ ] expose explicit catalogue, headless, and continuation operations
- [ ] reconcile public matrices and validate the extracted package

## Acceptance Criteria

- [ ] every guarantee names an exact reconciled Antigravity artifact
- [ ] personal Google and enterprise access do not imply one another
- [ ] invalid model selection remains a visible failure without fallback
- [ ] structured schema and stream activity are bounded and safely mapped
- [ ] tool and subagent payloads cannot leak through stable diagnostics
- [ ] continuation uses an exact conversation id, never ambient `--continue`
- [ ] permission-required tools remain denied unless provider-approved
- [ ] optional sandboxing is explicit and capability-gated
- [ ] Gemini's existing routes and guarantees remain unchanged
- [ ] focused and package evidence passes without a live provider prompt

## Decision Gates

- Stop if the installed artifact cannot be matched to an authoritative release.
- Stop if Google Sign-In requires Swallowtail to acquire or export credentials.
- Stop if continuation silently selects an ambient latest conversation.
- Stop if a stable machine protocol is required for callbacks but unavailable.

## Next Planning Checkpoint

After card 018, reassess Qwen account readiness and the paused standalone
Claude range extension. Keep Gemini maintenance paused unless its supported
access posture has fresh consumer value.

