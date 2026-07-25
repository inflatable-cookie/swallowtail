# 140 Grok ACP Production Driver

Status: backlog
Owner: Tom
Updated: 2026-07-24
Milestone: `../047-grok-build-maintained-acp-range.md`

## Objective

Implement one separately registered ambient Grok Build interactive driver over
ACP v1 stdio.

## Governing Refs

- Research 030 and 031
- Spec 003 or its promoted contract
- Contracts 009, 010, 012, 013, 015, 017, 023, 029, 033, and 034
- roadmap g01.047
- cards 137-139

## Scope

1. Add `swallowtail-adapter-grok`.
2. Bind exact executable, environment, working resource, delegated OAuth,
   subscription audience, model, version, ambient configuration, durable
   retention, permission posture, and `AmbientHost` before process start.
3. Launch the exact no-update ACP stdio argv and perform only the qualified
   delegated-auth activation sequence.
4. Map initialization, model selection, new session, prompt, updates,
   cancellation, provider requests, disconnect, and explicit close for only the
   frozen subset.
5. Join process, reader, callback, working-resource, state, and credential work.

## Boundaries

- no xAI direct API reuse or API-key injection
- no login, logout, auth-status, update, installation, or registry execution
- no bounded read-only claim; provider tools and ambient configuration remain
  visible capability and policy evidence
- no Swallowtail-owned write, shell, web, subagent, memory, plugin, hook, or
  MCP mechanism
- no sandbox or containment claim
- no load, resume, list, delete, or newer v1 method unless card 137 explicitly
  qualifies it
- no model or behavior fallback

## Acceptance Criteria

- [ ] driver identity remains distinct from direct `xai`
- [ ] every provider behavior is version-qualified
- [ ] provider permissions remain distinct from `AmbientHost` and containment
- [ ] delegated auth bytes and provider state never enter stable records
- [ ] cancellation, disconnect, and close retain exact session and cleanup truth
- [ ] no task or process detaches

## Validation

- focused Grok production-driver tests
- unchanged ACP and persistent-session tests
- warnings-denied focused clippy
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- exact descriptor, preflight, argv, protocol, lifecycle, and cleanup fixtures
- access, configuration, retention, permission, and isolation mismatch tests
- version dispatch, redaction, cancellation, deadline, and disconnect results

## Stop Conditions

- the production artifact differs from the frozen corpus
- delegated activation cannot remain separate from sign-in or mechanism change
- authentication or retained-state use requires uncontracted host authority

## Auto-Continuation

Yes, once card 141 is ready and the production driver passes focused
validation.

## Generation Disposition

This card remains behind cards 138-139 in the shared
[roadmap backlog](../../backlog/grok-build-maintained-acp-range.md). It stays
with its source generation and is not ready.
