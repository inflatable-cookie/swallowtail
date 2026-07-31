# Roadmap Generation Index

Status: active
Owner: Tom
Updated: 2026-07-31

## Generations

| Generation | Status | Focus |
| --- | --- | --- |
| `g01` | completed | Standalone authority, runtime kernel, integration landscape, representative drivers, consumer adoption, compatibility foundations, and broad provider coverage. |
| `g02` | completed | API stabilization, prepared facades, packaging, consumer upgrade proof, lifecycle management, feature-matrix closure, observable activity, and structural/validation hardening. |
| `g03` | active | Compatibility maintenance across real interface ranges and consumer-proven hardening without release chasing. |

## Generation Size

A generation normally contains 30-50 numbered roadmap files. Batch cards do
not count toward that range.

g01 closed at 49 roadmaps: 48 completed milestones and one backlog move. g02
closed at 49 roadmaps: 48 completed roadmaps, including roadmap 049's
disposition and cutover, plus one backlog move. Roadmap g02.029 and cards
097-098 remain recoverable as shared backlog evidence behind the Pi cwd-bound
attachment gate.

g03 resets roadmap and batch-card numbering. Its sequencing baseline is
compatibility maintenance and consumer-proven hardening. It begins with a
repository-local inventory, then current authoritative evidence, then a
bounded implementation-tranche decision. It does not imply provider
selection, consumer edits, publication, or a release per upstream version.

## Rollover Rule

g03 should normally run for 30-50 roadmaps. Provider releases, consumer
defects, contract additions, or completed maintenance tranches do not create a
new generation by themselves. Rollover requires a substantial run, clean
disposition of unfinished work, and an explicit sequencing reset.
