# 160 Subagent Topology Route Matrix And Package Proof

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../047-subagent-topology-acceptance-and-consumer-handoff.md`
Depends on: card 159

## Goal

Account for exact child-topology truth across production activity routes and
prove the positive packages from independent archives.

## Scope

1. Extend the provider activity matrix with exact columns for:
   - child identity and lifecycle
   - parent and metadata fidelity
   - child-authored activity attribution
   - visible provider collaboration actions
   - direct operator control
2. Populate every production activity route and operation shape.
3. Link every positive cell to a prepared profile and deterministic fixture.
4. Keep generic provider-tool activity distinct from child topology.
5. Machine-check allowed values, ordering, identities, and evidence links.
6. Assemble and audit core, runtime, Codex, and Kimi archives independently,
   then compile them through one shared extracted target.

## Acceptance Criteria

- [x] every production activity row has exact topology and control truth
- [x] only qualified Codex and Kimi routes claim child topology
- [x] no route claims direct operator child control
- [x] every positive cell maps to prepared evidence and deterministic fixtures
- [x] generic tool names do not create positive topology
- [x] selected archives assemble, audit, and compile independently
- [x] route, public-API, and docs gates pass
- [x] no provider call, consumer edit, candidate replacement, or publication
  occurs

## Validation

- provider activity matrix checker
- `effigy qa:routes`
- `effigy package:api`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime
  swallowtail-adapter-codex swallowtail-adapter-kimi`
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- Stop if matrix vocabulary flattens observation, collaboration, and control.
- Stop if a positive row lacks public prepared evidence.
- Do not replace independent archive assembly with source-workspace proof.
- Do not run live provider probes or the full workspace test suite.

## Auto-Continuation

Yes. Continue to card 161 after matrix and package proof pass.

## Evidence

- the 57-row activity matrix now carries exact observation, parentage,
  child-attribution, provider-collaboration, and operator-control columns
- four Codex or Kimi operation rows claim qualified topology; zero rows claim
  direct operator control
- route QA checks every topology value against exact route and operation
  identity
- core, runtime, Codex, and Kimi archives assemble and audit independently,
  then compile through one shared target in eight seconds
- the first interdependent-package run exposed a temporary workspace identity
  collision; the selector now checks each archive as its own root while sharing
  compilation artifacts and using local unpublished dependencies
- selector argument tests, public-API declarations, routes, docs, and diff
  hygiene pass
- no provider call, consumer edit, candidate replacement, or publication ran
