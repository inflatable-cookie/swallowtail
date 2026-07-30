# Grok Structured Conformance And Closeout

Date: 2026-07-30
Status: completed

## Changed

- added a distinct Grok structured-run role beside interactive sessions
- projected one operation-private ACP `session/new` plus one prompt into the
  shared structured-run lifecycle
- retained ordered activity, terminal output, cancellation, deadlines,
  provider requests, safe failures, and joined cleanup
- added prepared structured input, planning, discovery, and execution surfaces
- applied the shared ACP single-turn assertions
- refreshed route, feature, activity, package, release, backlog, and roadmap
  truth

## Boundary

The structured projection allocates a normal provider-owned durable Grok
session. Swallowtail closes its connection, process, task, credential, and
resource leases; it does not delete or hide that provider state.

Interactive and structured operations remain separate. The route claims no
load, resume, archive, restore, delete, native close, sandbox, read-only
harness, ambient permission approval, login, API-key reuse, or executable
search.

Exact `0.2.114` is the sole guaranteed release. Exact later stable observations
may execute only as unverified newer. Older, prerelease, malformed, and
wrong-revision observations reject.

## Conformance

- 19 focused Grok tests pass: five unit, eleven ACP, and three installed
  discovery tests
- exact interactive and structured operations pass local and
  remote-authoritative host topologies
- structured success, cancellation, deadline, provider request, disconnect,
  malformed protocol, diagnostic redaction, and cleanup remain distinct
- provider permissions stop execution and never imply sandbox or approval
  authority
- focused all-target warnings-denied clippy passes

## Package And Public Truth

The final 41-file `swallowtail-adapter-grok` archive is 60,383 bytes:

`d8bf760eca39c7c5d8f924ce61daef69e96c0c8ca5d2952507f6c02af59e62ff`

Its extracted all-target check and test compilation pass with local unpublished
Swallowtail dependencies. Package metadata and public-API gates now cover 24
crates. Public matrices record 27 production routes, 23 solutions, and 20
structured-run `Yes` cells.

Formatting, docs QA, route and activity matrix checks, and `git diff --check`
pass. Effigy doctor still reports 136 existing size findings: 104 warnings and
32 errors. Its four Grok findings are warnings; this batch adds no Grok
error-level finding.

No live Grok prompt, consumer edit, crates.io publication, release-candidate
replacement, or provider-session persistence work ran.

## Risks

- only exact `0.2.114` carries guaranteed support
- later stable releases still need milestone evidence before guarantee widening
- model choices remain session-negotiated, not a route-free catalogue
- provider-owned sessions remain durable without lifecycle management
- permission requests remain observation-and-stop events

## Next

Hold at the g02 stabilization checkpoint. Decide whether to widen Grok through
separately evidenced stable milestones or select another stabilization lane.
