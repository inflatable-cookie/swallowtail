# 226 ZCode App-Server Artifact And Event Corpus

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../071-zcode-app-server-foundation.md`
Depends on: Research 126; Spec 010

## Goal

Freeze exact ZCode runtime `0.16.3` artifact, app-server framing, create
handshake, session-log, correlation, terminal, and failure evidence before
production Rust behavior exists.

## Scope

1. Record npm packaging, `zcode.cjs` digest, runtime identity, spawn
   (`node zcode.cjs app-server`), and non-axis launcher/desktop versions
   from Research 126.
2. Freeze handshake, text-success, tool-success, tool-error,
   missing-credential, and unknown-event fixtures with private content
   redacted.
3. Define stream rules for line-delimited JSON, server→client
   `session/requestRuntimePreferences`, `session/event` versus result
   snapshots, usage, reasoning progress, text, tools, idle, and unknown
   types.
4. Name the exact qualified-only compatibility and protocol-facade
   revisions on axis `zcode.runtime`.

## Out Of Scope

- production driver, prepared facade, package topology, or live selector
- `--print`, TUI, desktop GUI, community ACP, or OpenCode
- committing private probe transcripts

## Acceptance Criteria

- [x] fixtures contain no credentials, account identifiers, private paths,
      prompts, reasoning bodies, tool input/result bodies, or raw session
      ids
- [x] random identities are consistently sanitized without weakening
      correlation evidence
- [x] create-without-preferences-reply is a fail-closed fixture
- [x] malformed, oversized, post-terminal, and mismatched-runtime records
      fail safely
- [x] unknown event types remain namespaced observations
- [x] the pin is runtime `0.16.3` plus payload digest, not launcher
      `3.7.7-13` and not desktop About `3.7.7`

## Validation

- focused package-independent fixture/parser tests introduced by this card
- `effigy qa:northstar`

## Stop Conditions

- stop if app-server cannot bind one create, preferences reply, send, idle,
      and terminal
- stop if sanitized fixtures cannot retain exact lifecycle meaning
- stop if the pin would use launcher or desktop About as the compatibility
      axis

## Auto-Continuation

Continue to card 227 once the shared fixture tree is ready for the Rust
driver.

## Evidence

- package-independent corpus validation: 13 tests passed
- `python3 scripts/check-zcode-app-server-corpus.py -v`
- Research 126
- Spec 010
- handshake create-plus-preferences is probe-proven; prompt/tool/terminal
  JSONL is reconstructed protocol shape, not a live transcript
