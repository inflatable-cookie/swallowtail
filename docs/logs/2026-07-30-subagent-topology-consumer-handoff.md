# Subagent Topology Consumer Handoff

Date: 2026-07-30

Card 161 closes roadmap g02.047 without editing a consumer.

## Nucleus Path

Nucleus can project browseable child work from the same ordered activity
stream it already consumes:

1. inspect exact prepared activity evidence
2. create one bounded `SubagentDirectoryProjection` per runtime operation
3. route primary and child-authored activity through `ActivityActor`
4. replace child snapshots by operation-local child id
5. retain operation, nested, and unknown parentage separately
6. end projection on terminal operation truth without inventing child status

The public `observable_activity_nucleus` example carries this reducer shape.
No native provider event switch is required.

Final local proof covers 155 focused core and runtime tests, warnings-denied
clippy, the public Nucleus example, public-API declarations, route truth, docs,
and Doctor at 144 warnings with zero errors.

## Ownership

Swallowtail owns:

- bounded identity, parentage, lifecycle, metadata, attribution, and order
- exact prepared profile truth
- redacted formatting
- visible provider-owned collaboration actions

Nucleus owns:

- durable graph and transcript records
- child selection and navigation
- labels, grouping, tabs, badges, collapse state, and retention
- operator policy and recovery workflows

Poodle owns reusable presentation components and local view state. No Poodle
contract or component hierarchy is imported into Swallowtail.

## Unsupported

No selected route exposes direct operator child spawn, steering,
interruption, resume, wait, close, or deletion. Visible Codex collaboration
actions are observations only. Whole-turn cancellation and main-agent
messaging are not substitutes.

No Nucleus, Poodle, Soundcheck, provider, candidate, publication, tag, or
release state changed.

## Next

Return to the operator checkpoint. Nucleus adoption requires explicit consumer
authorization; publication and provider-session binding persistence remain
deferred.
