# Roadmap Generation Index

Status: active
Owner: Tom
Updated: 2026-08-19

## Generations

| Generation | Status | Focus |
| --- | --- | --- |
| `g01` | completed | Standalone authority, runtime kernel, integration landscape, representative drivers, consumer adoption, compatibility foundations, and broad provider coverage. |
| `g02` | completed | API stabilization, prepared facades, packaging, consumer upgrade proof, lifecycle management, feature-matrix closure, observable activity, and structural/validation hardening. |
| `g03` | completed | Compatibility maintenance across real interface ranges and consumer-proven hardening without release chasing. |
| `g04` | active | Route availability, connection admission, credential and sign-in descriptors, readiness refresh, and consumer-driven model selection without a Swallowtail server. |

## Generation Size

A generation normally contains 30-50 numbered roadmap files. Batch cards do
not count toward that range.

g01 closed at 49 roadmaps: 48 completed milestones and one backlog move. g02
closed at 49 roadmaps: 48 completed roadmaps, including roadmap 049's
disposition and cutover, plus one backlog move. Roadmap g02.029 and cards
097-098 remain recoverable as shared backlog evidence behind the Pi cwd-bound
attachment gate.

g03 closed at 106 roadmaps after operator-authorized rollover. Compatibility
maintenance, consumer-proven hardening, recovery, source tags, harness
expansion, and the currentness sweep are complete or rehomed. Aider headless,
Kiro headless, and OpenHands production wiring join the shared backlog.
Gemini requalification, Pi continuity, and binding persistence stay deferred.

g04 resets roadmap and batch-card numbering. Its sequencing baseline is route
readiness and connection admission for consuming applications. It begins with
a repository-local inventory of existing Swallowtail records against the
consumer connection lifecycle. It does not imply a connection server,
credential store, UI, router, or implementation before evidence and operator
decisions agree.

## Rollover Rule

g04 should normally run for 30-50 roadmaps. Provider releases, consumer
defects, contract additions, or completed readiness tranches do not create a
new generation by themselves. Rollover requires a substantial run, clean
disposition of unfinished work, and an explicit sequencing reset.


## Next Task

Implement the active roadmap card named by the
[roadmaps front door](README.md#next-task). This index records generation
status; the batch pointer stays in the front door.
