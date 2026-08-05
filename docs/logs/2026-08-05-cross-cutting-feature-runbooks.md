# 2026-08-05 Cross-Cutting Feature Runbooks

Roadmap: `../roadmaps/g03/042-complete-integration-guide-system.md`
Card: `../roadmaps/g03/batch-cards/122-cross-cutting-feature-runbooks.md`

## Changed

- added provider selection and preparation guidance for consumer-assembled
  configured-instance catalogues, exact model binding, immutable evidence,
  readiness, and route selection
- added one ordinary run/session lifecycle for event, callback, terminal,
  usage, cost, cancellation, cleanup, and restart ordering
- added generation-control and input-authority guidance for output limits,
  reasoning, schemas, attachments, tools, permissions, typed questions,
  resources, writes, and search
- added provider-state and resource lifecycle guidance for continuation,
  import, restoration, management, remote cleanup, and attached versus owned
  runtimes
- deepened observable activity with plan-mode selection and typed task-list
  replacement semantics
- deepened portable failure and validation ownership boundaries
- moved all 14 feature-family rows to complete; every one of the 34 matrix
  feature columns and each named portable surface now has a canonical guide
  owner

No generic provider router, fallback, retry, credential workflow, prompt API,
or consumer persistence policy was added.

## Validation

- `effigy check:examples` — passed
- `effigy qa:docs` — passed
- `effigy qa:routes` — passed
- `git diff --check` — passed

No live or authenticated provider work ran.

## Next Move

Execute card 123: add deterministic coverage over production routes, feature
headers, guide owners, and examples; reconcile indexes; close g03.042.
