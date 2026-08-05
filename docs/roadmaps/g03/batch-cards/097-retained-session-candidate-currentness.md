# 097 Retained Session Candidate Currentness

Status: planned
Owner: Tom
Created: 2026-08-05
Milestone: `../037-retained-session-recovery-promotion.md`
Depends on: card 096

## Goal

Revalidate Pi RPC and Alibaba Conversations against their exact retained-session
recovery gates and select each route independently.

## Scope

1. Refresh maintained public Pi switching, effective-cwd, and ordered-history
   evidence.
2. Refresh Alibaba conversation retrieval, item listing, continuation,
   retention, and cleanup evidence.
3. Test each candidate against exact host, resource, access, model, version,
   provider-state, replay, and readiness requirements.
4. Record supported or blocked outcomes without route-family inheritance.
5. Use no authenticated provider mutation by default.

## Validation

- `effigy validate:focused swallowtail-adapter-pi swallowtail-adapter-alibaba-model-studio`

## Stop Conditions

- keep Pi blocked if effective cwd cannot be caller-bound and corroborated
- keep Alibaba blocked if retention cannot be separated from operation-owned
  deletion
- stop rather than infer authority from private storage or copied ids

## Auto-Continuation

Continue independently to cards 098-100 only for gates this card opens. Card
101 closes the milestone even if every candidate remains blocked.
