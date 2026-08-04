# 2026-08-04 Durable Session Resume-Binding Persistence

## Outcome

The reported T3 Code failure was consumer mapping loss, not OpenCode session
rotation. Three new OpenCode roots existed, the prior roots remained intact,
and the consumer's provider-session mapping was null. OpenCode compaction keeps
the same session id.

Swallowtail now exposes a provider-neutral durable form for one ordinary
`SessionResumeBinding`:

- `PersistedSessionResumeBinding`
- `SessionResumeBinding::export_persisted`
- `SessionResumeBinding::restore_persisted`
- `SessionResumeBindingPersistenceFailure`
- `SessionResumeBindingPersistenceFailureKind`

The record is bounded to 8 KiB, versioned, opaque under default formatting,
and protected against stored-record corruption. It carries one bounded opaque
provider-session reference, binding origin, and an exact attachment
fingerprint. The fingerprint covers adapter, transport, configured instance,
revision, target, host, facade, policy, access profile, credential mechanism,
audience, model route, model, provider, interface versions, working resource,
session access, provider state, and harness configuration.

Restore reconstructs authority only from the current matching preflight plan,
working resource, and access policy. Malformed, oversized, unsupported,
corrupted, or drifted records fail before provider work. There is no raw-id
constructor path, lookup, import, route switch, or fresh-session fallback.

## OpenCode Acceptance

The deterministic restart case opens `ses_fixture`, exports and reconstructs
the record across an ownership boundary, restores under a freshly supplied
exact plan, resumes the exact provider session, and submits the next prompt to
`/session/ses_fixture/prompt_async`. The trace contains one session create.

Access and interface-version drift plus record corruption issue no HTTP work.
The frozen `session.compacted` lifecycle retains the existing session identity;
the protocol still quarantines the same envelope under a foreign session id.

Consumers own atomic storage with their local thread mapping. On restore
failure they keep local history readable and require explicit refresh or
re-import. They do not create a replacement session implicitly.

Provider-session archive, restore, and delete bindings remain same-process
authority under their separate deferred persistence gate.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-opencode` —
  210 tests passed
- `effigy package:verify-affected swallowtail-runtime swallowtail-adapter-opencode`
  — both extracted packages compiled
- `effigy qa:docs`
- `git diff --check`
- no authenticated provider work or live provider operation

## Current State

Cards 066-067 and roadmap g03.025 are complete. The sole Next Task has returned
to the g03 evidence gate.
