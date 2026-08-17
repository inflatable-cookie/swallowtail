# 222 DeepSeek Harness Web Artifact And `/api` Corpus

Status: ready
Owner: Tom
Created: 2026-08-17
Milestone: `../070-deepseek-harness-web-api-foundation.md`
Depends on: Research 125; Spec 009

## Goal

Freeze exact DeepSeek Harness `@deepseek-ai/dsh@0.1.0-rc.6` web artifact,
loopback trust fence, method allowlist, unary RPC, mux, history, and failure
evidence before production Rust behavior exists.

## Scope

1. Record CLI identity, npm pin, spawn (`dsh web`), loopback bind, and
   Cordis patch admission from Research 125.
2. Freeze redacted fixtures for list, search, create, history, models,
   prompt, cancel, fork, archive, denied methods, and malformed carrier
   cases.
3. Define stream rules for POST `/api/<method>`, WebSocket mux, HTTP
   carrier status versus business errors, and control-free history paging.
4. Name the exact qualified-only compatibility and protocol-facade
   revisions on axis `deepseek-harness.web`.

## Out Of Scope

- production driver, prepared facade, or live selector
- JSON-RPC, ACP, headless CLI, or `deepseek.continuation` changes
- credentials, settings, llm configuration, directory picker, ZIP export
- committing private probe transcripts

## Acceptance Criteria

- [ ] fixtures contain no credentials, account identifiers, private paths,
      prompts, reasoning bodies, tool bodies, or export bytes
- [ ] history fixtures show inspect-without-resume and keep page bounds
- [ ] denied methods fail closed without calling provider work
- [ ] random identities are sanitized without weakening correlation
- [ ] the pin is CLI/npm identity, not JSON-RPC payload digest and not
      `host.describe`

## Validation

- focused package-independent fixture/parser tests introduced by this card
- `effigy qa:northstar`

## Stop Conditions

- stop if `/api` cannot bind loopback, allowlisted unary methods, and mux
      events without booting a browser
- stop if `session.history` resumes or publishes an Agent
- stop if the pin would use launcher `-V` or `host.describe` as the
      compatibility axis
- stop if fixtures would require credentials, settings, or ZIP export

## Auto-Continuation

Continue to card 223 once the shared fixture tree is ready for the Rust
driver.
