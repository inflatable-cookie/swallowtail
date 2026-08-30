# Roadmap Status Grammar

Status: active
Audience: agents editing generation indexes and batch cards

`effigy qa:docs:roadmaps:status` runs
`scripts/check-roadmap-status-drift.py`. That checker owns the accepted
Status buckets and the generation-index census phrases. Prefer matching this
grammar over rewriting the parser.

## Status Buckets

The first token of a card or milestone `Status:` line must be one of:

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
Status: planned; gated behind card 016
Status: ready; one authorized live turn
```

Do not write `Status: gated`.

## Generation-Index Census

The active generation's census paragraph in `generation-index.md` must carry
these exact shapes (live regexes in the checker):

| Claim | Required phrase |
| --- | --- |
| Completed count | `N completed milestones` |
| Honest stops | `honest evidence stops at …` (id list), or `no honest evidence stops` |
| Ready set | `ready milestones at 003` / `ready milestone at 003`, or `one ready milestone at 003` |

Examples that pass:

```text
g05 now has 2 completed milestones, honest evidence stops at 001, and one
ready milestone at 003.
```

```text
g05 has 4 completed milestones, no honest evidence stops, and ready
milestones at 003, 007.
```

Card-ready prose elsewhere in the generation index uses
`card 011 is ready` / `cards 011-012 are ready` and must match frontmatter.
