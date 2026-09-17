# Roadmap Status Grammar

Status: active
Audience: agents editing generation indexes and task files

`effigy qa:docs:roadmaps:status` runs
`scripts/check-roadmap-status-drift.py`. That checker owns enforcement; this
note records the model it enforces. Prefer matching this grammar over
rewriting the parser.

## Lifecycle-Owned Terminal State

Since g05.057, terminal task state is lifecycle-owned. The canonical records
live in `.northstar/lifecycle/v1/tasks/` (compacted closed generations live
in `.northstar/lifecycle/v1/generations/`), and the generated projection
blocks in the front doors are the only rendered status. Task files carry no
hand-maintained `Status:` line once a task is dispatched through the queue
lifecycle, the generation README's `## Tasks` section is a flat link registry
with no status buckets, and the generation index carries no hand-maintained
status census. The retired status-bucket model (status sections under
`## Tasks`, index census phrases) is a migration defect, like any nested card
level: do not reintroduce it.

## Planning Status Lines

A planned, pre-dispatch task file may carry one `Status:` line whose first
token is one of:

| Bucket | Accepted primary tokens |
| --- | --- |
| planned | `planned` |
| ready | `ready` |
| blocked | `blocked` |
| stopped | `stopped` |
| complete | `complete`, `completed`, `done` |

Anything after the first `;` is free-form detail. `gated` is not a status
bucket. Write a gate as detail after an accepted bucket:

```text
Status: planned; gated on a non-empty Research 256 deliver-now disposition
Status: ready; one authorized live turn
```

Do not write `Status: gated`.

## Enforcement

The checker validates: exactly one active generation is declared in
`generation-index.md` (`| gNN | active |`), every task file in the active
generation is indexed exactly once under the README's `## Tasks`, every
indexed link resolves to a task file, no status buckets return under
`## Tasks`, and no nested dispatch level (`batch-cards/`) is reintroduced in
current planning surfaces.

## No Nested Dispatch Level

There is no card, milestone-wrapper, or `batch-cards/` level. The checker
rejects any `batch-cards/` directory or link, `## Batch Cards` section,
`Milestone:` pointer, `execute card` verb, card-budget or allowed-runway
table, or `First milestone` / `Next milestone` column in current planning
surfaces. Past-tense card evidence citations (which unit proved a frozen
outcome) are history, not dispatch, and stay.

## Next Task

Implement the active task named by the [roadmaps front door](README.md#next-task).
This file records status grammar; the sole actionable pointer stays in the
front door.
