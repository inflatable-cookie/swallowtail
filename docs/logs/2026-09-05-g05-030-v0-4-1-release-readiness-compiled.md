# g05.030 v0.4.1 Release Readiness Compiled

Date: 2026-09-05
Roadmap: `../roadmaps/g05/030-v0-4-1-release-readiness.md`

## Why

Card 080 delivered the Claude SDK read-write session and permission policy
through PRs 221 and 224, with card 089's preflight scoping underneath. That
is the content Bovine Desktop waits on, and the operator named `v0.4.1` as
its carrier. Since `v0.4.0` the tree also carries OpenCode `1.18.28`, the
Contract 061 tranches for candidates C, E, F, I, and J with two shared
baselines, and the Contract 013 and 061 amendments. All are believed
additive or widening; card 090 proves it.

## Shape

Three serial cards on the g05.021 precedent: 090 audit, 091 prepare and
exact-SHA CI after separate authorization, 092 consumer proof and tag gate
after the operator's smoke packet. Chatterbox recommends a Bovine editing
session on `claude-agent.sdk` as the smoke. A feature freeze holds from this
promotion until card 092 stops.

## Authorizations

On 2026-09-05 the operator granted card 091's one-shot prepare authorization,
contingent on card 090's acceptance, and accepted Bovine Desktop on
`claude-agent.sdk` as the card 092 smoke application. The exact Bovine
checkout, command, and retry budget are still to be supplied before card 092
is ready.

## First Prepare

The first card 091 prepare passed nine gates and failed `floor` with no
retained output; Effigy rolled back cleanly. Chatterbox reproduced the floor
gate green on the same tree and CI's pinned-MSRV job was green on card
090's head, so the failure was transient. The operator renewed the
authorization with a standing grant for further transient failures; every
attempt now captures each gate's output.

## Next

Coordinator resumes card 091 on the same worker.
