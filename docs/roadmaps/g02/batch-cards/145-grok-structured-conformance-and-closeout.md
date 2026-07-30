# 145 Grok Structured Conformance And Closeout

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../043-grok-build-maintained-acp-route.md`

## Goal

Close the Grok route through exact structured projection, cross-host
conformance, package assembly, and public integration truth.

## Governing Refs

- Research 070
- Contracts 011, 023, 029, 037, 039, and 044
- roadmap g02.043
- cards 142-144

## Scope

1. Apply the shared ACP single-turn assertion pack.
2. Add one operation-private structured projection with explicit durable local
   retention and no deletion claim.
3. Prove baseline, unverified-newer, exclusion, cancellation, deadline,
   disconnect, provider request, malformed protocol, cleanup, and redaction.
4. Prove local and remote-authoritative execution.
5. Refresh the route guide, provider-solution matrix, package topology,
   changelog, release notes, backlog, roadmaps, and one closeout log.
6. Assemble the affected packages without publishing.

## Acceptance Criteria

- [x] interactive and structured roles remain distinct
- [x] exact `0.2.114` passes both host topologies
- [x] unverified-newer posture never becomes guaranteed support
- [x] structured retention is durable and provider-owned
- [x] provider permissions remain separate from isolation and sandboxing
- [x] feature-matrix `Yes` values map to public prepared operations
- [x] affected package archives assemble and compile
- [x] one clear next task remains

## Validation

- focused Grok and shared conformance suites
- affected package assembly and extracted compile
- docs QA
- doctor delta review
- `git diff --check`

## Stop Conditions

- Stop if structured projection needs prompt replay or hidden cleanup.
- Stop if cross-host behavior differs.
- Do not publish or edit consumer repositories.

## Auto-Continuation

No. Return to the g02 stabilization checkpoint.

## Evidence

- 19 focused adapter tests pass: five unit, eleven ACP, and three installed
  discovery tests.
- Exact `0.2.114` runs interactive and structured operations on local and
  remote-authoritative hosts. `0.2.115` remains unverified newer; older,
  prerelease, malformed, and wrong-revision observations reject.
- Structured success, cancellation, deadline, provider request, disconnect,
  malformed protocol, redaction, and joined cleanup pass deterministic
  fixtures plus the shared ACP assertion pack.
- The prepared facade exposes distinct interactive and structured operations.
  Structured execution retains its private provider-owned session and makes no
  lifecycle, deletion, sandbox, or ambient-permission-approval claim.
- Public truth now records 27 production routes, 23 solutions, and 20
  structured-run `Yes` cells.
- The 41-file Grok archive assembles without publication. Its final SHA-256 is
  `d8bf760eca39c7c5d8f924ce61daef69e96c0c8ca5d2952507f6c02af59e62ff`;
  extracted all-target check and test compilation pass.
- Focused warnings-denied clippy, formatting, docs QA, package metadata,
  public-API, route/activity matrix, and diff checks pass.
- Effigy doctor still reports 136 existing size findings: 104 warnings and 32
  errors. All four Grok findings are warnings; this card adds no Grok
  error-level finding.

## Closeout

Roadmap g02.043 is complete. No live prompt, consumer edit, publication, or
provider-session lifecycle authority was added. The next step is the explicit
g02 stabilization checkpoint, not automatic execution.
