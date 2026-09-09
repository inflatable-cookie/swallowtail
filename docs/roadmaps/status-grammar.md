# Roadmap Status Grammar

Status: active
Audience: agents editing generation indexes and task files

`effigy qa:docs:roadmaps:status` runs
`scripts/check-roadmap-status-drift.py`. That checker owns the accepted
Status buckets and the generation-index census phrases. Prefer matching this
grammar over rewriting the parser.

## Status Buckets

The first token of a task `Status:` line must be one of:

| Bucket | Accepted primary tokens |
| --- | --- |
| planned | `planned` |
| ready | `ready` |
| blocked | `blocked` |
| stopped | `stopped` |
| complete | `complete`, `completed`, `done` |

Anything after the first `;` is free-form detail. Index annotations may use the
same primary tokens, plus complete aliases `evidence stop` and
`identity stop`.

`gated` is not a status bucket. Write a gate as detail after an accepted
bucket:

```text
Status: planned; gated on a non-empty Research 256 deliver-now disposition
Status: ready; one authorized live turn
```

Do not write `Status: gated`.

The active generation README lists each task once under ``### Planned``,
``### Ready``, ``### Blocked``, ``### Stopped``, or ``### Completed`` beneath
its ``## Tasks`` section. ``stopped`` Status maps only to ``### Stopped``.

## Generation-Index Census

The active generation's census paragraph in `generation-index.md` must carry
these exact shapes (live regexes in the checker):

| Claim | Required phrase |
| --- | --- |
| Completed count | `N completed tasks` |
| Honest stops | `honest evidence stops at …` (id list), or `no honest evidence stops` |
| Ready set | `ready tasks at 003` / `ready task at 003`, or `one ready task at 003` |
| Planned set | `one planned task at 035`, `planned tasks at 035, 039`, or `no planned tasks` |

Examples that pass:

```text
g05 now has 2 completed tasks, honest evidence stops at 001, and one
ready task at 003.
```

```text
g05 has 4 completed tasks, no honest evidence stops, ready
tasks at 003, 007, and one planned task at 035.
```

Task-ready prose elsewhere in the generation index uses
`task 011 is ready` / `tasks 011-012 are ready` and must match frontmatter.

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
