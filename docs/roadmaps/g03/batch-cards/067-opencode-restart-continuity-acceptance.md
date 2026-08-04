# 067 OpenCode Restart Continuity Acceptance

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../025-durable-session-resume-binding-persistence.md`
Depends on: card 066

## Goal

Prove the OpenCode prepared attachment resumes one exact provider session after
a consumer process boundary without identity rotation or fresh-session fallback.

## Scope

1. Add a deterministic open, export, reconstruct, resume, and prompt case.
2. Assert the HTTP trace contains one session create and exact-id continuation.
3. Freeze same-session `session.compacted` as lifecycle-only identity evidence.
4. Keep foreign-session lifecycle quarantined and never adopted.
5. Reconcile the integration guide, Nucleus handoff, roadmap, and closeout log.

## Out Of Scope

- live OpenCode, provider prompt, credential, or consumer acceptance
- automatic title recovery, catalogue import, synchronization, or replacement
- explicit OpenCode fork support or qualified-range extension
- consumer repository edits

## Acceptance Criteria

- [x] simulated restart resumes and prompts `ses_fixture`
- [x] no second `POST /session` occurs
- [x] compaction retains the existing session binding
- [x] corrupted or drifted records never reach provider work
- [x] focused runtime/OpenCode and affected-package verification pass
- [x] the sole Next Task returns to the g03 evidence gate

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-runtime swallowtail-adapter-opencode`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. Return to the g03 evidence gate after closeout.
