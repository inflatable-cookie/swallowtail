# 098 Alibaba Retained Conversation Contract And Corpus

Status: complete
Owner: Tom
Created: 2026-08-05
Milestone: `../037-retained-session-recovery-promotion.md`
Depends on: card 097 selecting Alibaba

## Goal

Define and freeze a separate retained Alibaba conversation profile without
widening the existing operation-owned delete-on-close route.

## Scope

1. Promote exact retention, ownership, attachment, replay, and cleanup rules.
2. Freeze bounded conversation and ordered-item retrieval before readiness.
3. Bind workspace, region, endpoint, deployment, credential, model, and
   conversation identity.
4. Cover foreign, missing, deleted, malformed, oversized, stale, and uncertain
   conversations.
5. Preserve explicit cleanup as separate authority.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit swallowtail-adapter-alibaba-model-studio`

## Stop Conditions

- stop if retained and operation-owned profiles cannot coexist explicitly
- stop if list or retrieval mutates state or cannot prove replay completion

## Auto-Continuation

Continue to card 100 only when the contract and corpus pass.

## Completion

- [x] Contract 017 admits exact resource-free resume-binding persistence and
      load without weakening resource-bound resume or operation checkpoints
- [x] Contract 025 separates retained preservation from operation-owned
      delete-on-close
- [x] Contract 038 keeps retained deletion behind separate management
      authority; the persisted resume record grants none
- [x] Alibaba retrieval and ordered pagination requests are frozen
- [x] replay parsing enforces exact identity, role, content, sequence, page,
      item, and byte bounds
- [x] foreign, missing, deleted, malformed, oversized, stale, and uncertain
      dispositions produce no readiness or fallback
- [x] common retained-conversation conformance coexists with the existing
      delete-on-close profile
- [x] focused acceptance passed with no authenticated provider work

Card 100 is ready. It may implement only the separate retained prepared
profile. The current Alibaba prepared conversation remains delete-on-close.
