# 097 OpenCode Session Identity And Resume-Binding Persistence

Status: promoted
Owner: Tom
Date: 2026-08-04

## Question

Does OpenCode replace a durable session identity during compaction, and can a
Swallowtail consumer safely continue the exact provider session after its own
process restarts?

## Evidence

The confirmed T3 Code reproduction contains three separate OpenCode root
sessions for one continuous consumer thread. The older sessions remain intact,
no compaction event occurred, and T3 Code persisted no provider session id. It
recovered by mutable title and created another root session when recovery
failed. This is consumer binding loss, not provider-planned identity rollover.

OpenCode `1.18.10` and current `1.18.13` compaction source retain the supplied
`sessionID` for compaction messages, processor work, replay, automatic
continuation, and the final `session.compacted` event. Explicit fork is a
separate operation which creates another session. `prompt_async` targets the
session id in its request path and passes that same id into prompt execution.

Swallowtail already retains one exact OpenCode session id in the live handle,
targets it for prompt, load, resume, abort, replay, and deletion, and
quarantines SSE events owned by other sessions. It does not silently adopt a
replacement id.

The remaining gap is process restart. Contract 017 permits consumer
persistence of an opaque `SessionResumeBinding`, but the public runtime has no
stable persistence form. The Nucleus import handoff therefore requires
explicit catalogue/import again after restart. A consumer which calls new
instead can recreate T3 Code's fresh-root history loss.

## Decision

Promote a versioned provider-neutral persisted form for ordinary
`SessionResumeBinding` under Contract 017.

The persisted form:

- carries one bounded opaque provider-session reference
- fingerprints the exact adapter, transport, instance, target, host, facade,
  access, model route, interface versions, working resource, and session policy
- preserves binding origin
- rejects malformed, oversized, unsupported-version, corrupted, and
  attachment-drifted records
- reconstructs a usable binding only from the current matching preflight plan
- exposes no credential, endpoint secret, working-resource value, prompt,
  transcript, title, or provider payload
- never creates, discovers, imports, or substitutes a provider session

This is not provider-session management-binding persistence. Archive, restore,
and delete authority remain under their deferred gate.

## Validation Needs

- provider-neutral round trip, corruption, version, bounds, redaction, and
  route/access/resource drift cases
- OpenCode open, persisted-record reconstruction, exact resume, and prompt
  against the original provider session without a second create call
- same-session `session.compacted` lifecycle acceptance
- foreign-session lifecycle quarantine without identity adoption
- explicit proof that invalid persistence never falls back to session creation

## Sources

- [T3 Code confirmed reproduction](https://github.com/pingdotgg/t3code/issues/2343#issuecomment-5173056639)
- [OpenCode `1.18.10` compaction](https://github.com/anomalyco/opencode/blob/v1.18.10/packages/opencode/src/session/compaction.ts)
- [OpenCode `1.18.13` compaction](https://github.com/anomalyco/opencode/blob/v1.18.13/packages/opencode/src/session/compaction.ts)
- [OpenCode `1.18.13` session HTTP handlers](https://github.com/anomalyco/opencode/blob/v1.18.13/packages/opencode/src/server/routes/instance/httpapi/handlers/session.ts)
