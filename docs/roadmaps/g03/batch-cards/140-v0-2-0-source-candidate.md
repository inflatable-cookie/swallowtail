# 140 v0.2.0 Source Candidate

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../046-v0-2-0-muse-and-rust-floor-source-release.md`
Depends on: card 139

## Goal

Produce one complete local `v0.2.0` source candidate with all deterministic
release gates passing.

## Scope

1. Promote Contract 036, architecture, front-door, route, package, and semantic
   API truth from unreleased Muse evidence to the `v0.2.0` candidate.
2. Promote the unified Rust `1.95.0` floor into Contract 036, architecture,
   release baselines, source-consumer proof, and CI truth.
3. Write changelog and `v0.2.0` release notes with explicit breaking-MSRV,
   upgrade, and rollback guidance.
4. Deliberately refresh the dependency lock within the declared Rust floor
   and record every advance or retain decision.
5. Use Effigy to prepare version `0.2.0`; synchronize internal requirements,
   lock entries, examples, and release baselines exactly.
6. Run all 11 configured local release gates against the final prepared tree.

## Acceptance

- [x] workspace version, 28 package manifests, internal requirements, lock,
      changelog, release notes, and source examples agree on `0.2.0`
- [x] old 27 package APIs remain compatible and Muse joins the released API
      baseline
- [x] 34 released routes and 28 released packages are exact
- [x] all 28 packages pass the exact unified Rust `1.95.0` floor
- [x] all release gates and exact-revision source-consumer proof pass
- [x] no tag, registry, GitHub Release, consumer, or provider mutation runs

## Validation

- Effigy release simulation, status, and prepare plan
- `effigy release prepare --yes --check-gates`
- complete configured release gates after final synchronized state

## Stop Conditions

- stop on dependency, API, MSRV, source-consumer, security, or release-gate
  failure
- stop before commit, push, workflow dispatch, or tag mutation

## Auto-Continuation

No. Return one complete local candidate worktree for operator review and exact
commit authorization.

## Completion Evidence

- Contract, architecture, front-door, package, route, semantic API, dependency,
  toolchain, changelog, and release-note truth now describe `v0.2.0`.
- Bedrock explicitly inherits the workspace Rust version; all 28 packages
  declare Rust `1.95` metadata and use coordinated `^0.2.0` internal edges.
- Muse joins the candidate package, route, and semantic API baselines without
  rewriting immutable `v0.1.x` evidence.
- The dependency refresh advances only the 28 workspace package identities.
  Six available third-party updates remain outside declared ranges or the
  selected graph and are deliberately retained.
- Effigy prepared `v0.2.0` after all 11 gates passed. Narrowing the temporary
  cross-minor transition requirements to exact `^0.2.0` makes that preparation
  fingerprint stale by design; all 11 gates pass again on the final tree.
- No authenticated provider, consumer, commit, push, workflow, tag, registry,
  or GitHub Release mutation ran.
