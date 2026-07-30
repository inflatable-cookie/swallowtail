# 142 Grok Authenticated Evidence And Contract

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../043-grok-build-maintained-acp-route.md`

## Goal

Resolve the held Grok delegated-authentication gate and make the production
lane contract-ready.

## Scope

1. Revalidate official xAI, npm, and ACP registry evidence.
2. Match the installed authenticated executable to one exact public artifact.
3. Run one no-prompt activation and session-allocation probe.
4. Prove credential-file stability and classify provider-owned state changes.
5. Promote the narrow Contract 015 activation rule.
6. Archive Spec 003 and freeze sanitized deterministic fixtures.

## Acceptance Criteria

- [x] exact `0.2.114` executable and signed artifact match
- [x] `cached_token` is advertised and selected without fallback
- [x] activation succeeds without login, API key, prompt, or model request
- [x] existing authentication file remains unchanged
- [x] provider-private authentication metadata is absent from repository state
- [x] durable empty-session creation is recorded honestly
- [x] Research 070 and Contract 015 hold the durable decision
- [x] focused protocol fixture and docs validation pass

## Validation

- focused Grok ACP protocol fixture tests
- docs QA
- `git diff --check`

## Stop Conditions

- Stop if the current executable does not match the public signed artifact.
- Stop if activation invokes an interactive flow or changes access mechanism.
- Do not delete ambient Grok state by path convention.

## Auto-Continuation

Yes. Continue to card 143 after focused evidence validation.

## Evidence

- [Research 070](../../../research/070-grok-build-0-2-114-authenticated-acp-qualification.md)
- exact installed and npm artifact digests
- bounded no-prompt activation and `session/new` probe
- sanitized `0.2.114` ACP fixture corpus
- 10 focused Grok protocol tests pass
- Effigy docs QA passes
