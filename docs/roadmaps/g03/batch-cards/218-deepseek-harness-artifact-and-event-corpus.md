# 218 DeepSeek Harness Artifact And Event Corpus

Status: ready
Owner: Tom
Created: 2026-08-17
Milestone: `../069-deepseek-harness-jsonrpc-foundation.md`
Depends on: Research 124; Spec 008

## Goal

Freeze exact DeepSeek Harness runtime-bin `0.1.0rc6` artifact, JSON-RPC,
session-log, correlation, terminal, and failure evidence before production
Rust behavior exists.

## Scope

1. Record wheel, executable, spawn-helper, and `serverInfo` identity from
   Research 124.
2. Freeze handshake, text-success, tool-success, tool-error, missing-key, and
   unknown-event fixtures with private content redacted.
3. Define stream rules for JSON-RPC frames, `session.event` versus durable
   JSONL packing, usage, reasoning progress, text, tools, idle, and unknown
   types.
4. Name the exact qualified-only compatibility and protocol-facade revisions.

## Out Of Scope

- production driver, prepared facade, package topology, or live selector
- ACP, Web `/api`, headless CLI, or `deepseek.continuation`
- committing private probe transcripts

## Acceptance Criteria

- [ ] fixtures contain no credentials, account identifiers, private paths,
      prompts, reasoning bodies, or tool input/result bodies
- [ ] random identities are consistently sanitized without weakening
      correlation evidence
- [ ] live-stream cardinality is recorded as distinct from durable JSONL
      packing
- [ ] malformed, oversized, post-terminal, and mismatched-model records fail
      safely
- [ ] unknown event types remain namespaced observations

## Validation

- focused package-independent fixture/parser tests introduced by this card
- `effigy qa:northstar`

## Stop Conditions

- stop if JSON-RPC cannot bind one initialize, prompt receipt, idle, and
      terminal
- stop if sanitized fixtures cannot retain exact lifecycle meaning
- stop if the pin would use `serverInfo.version` as the compatibility axis

## Auto-Continuation

Continue to card 219 once the shared fixture tree is ready for the Rust
driver.

## Evidence

- Research 124
- Spec 008
- isolated probe captures remain outside the repository until redacted
  fixtures land
