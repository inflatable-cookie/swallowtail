# 067 v0.3.2 Source Patch Release

Status: active
Owner: Tom
Created: 2026-08-11
Depends on: g03.059-g03.066; operator package selection
Vision tags: source release, compatibility maintenance, consumer hardening
Contract refs: 001, 009, 022-023, 029, 036-037, 039, 044-045, 051-056

## Problem

Current source adds Command Code, idioms, Codex spawn admission, Claude
response-only execution, and two host/tooling fixes over immutable `v0.3.1`.
The complete local candidate is prepared and green. Exact commit, canonical CI,
and tag identity remain operator-gated.

## Generation Runway

Advance g03's source-release and consumer-proven hardening goals. Publish the
reviewed 30-package, 36-route compatible source shape without changing the
source-only distribution boundary.

## Execution Plan

- [x] card 210: align the `v0.3.2` contract, changelog, 30-package release
      order, metadata, and separate unreleased API baselines
- [x] card 211: write release notes, prepare the exact local candidate, and
      pass all configured credential-free gates
- [ ] card 212: after operator acceptance, commit and push the candidate and
      require canonical CI at the exact SHA
- [ ] card 213: after separate exact authorization, create and push one
      annotated immutable `v0.3.2` tag

## Goals

- [x] prepare Command Code and idioms as separately selectable packages
- [x] prepare 36 production routes including `command-code.headless` and
      `claude-code.response-only`
- [x] preserve the `v0.3.1` public API, Rust `1.95.0` floor, verified target,
      source-only distribution, and exact provider boundaries
- [ ] bind version, changelog, release notes, package graph, API evidence, CI,
      and tag to one exact source identity

## Boundaries

- no crates.io publication, GitHub Release object, binary, installer, model
  artifact, consumer mutation, or authenticated provider work
- no release-gate bypass, workflow edit, or timeout inflation
- no candidate mutation before card 211 and no remote or tag mutation without
  the later cards' explicit operator authorization
- existing tags remain immutable

## Acceptance Criteria

- [x] all 30 packages share `0.3.2` and resolve in release dependency order
- [x] old 28 package APIs remain compatible and both new packages receive
      exact first-release API baselines
- [x] all configured local gates and isolated source-consumer proof pass
- [ ] canonical CI passes at the exact candidate commit
- [x] release notes contain upgrade, rollback, payload, limits, and unchanged
      distribution posture
- [ ] annotated `v0.3.2` resolves locally and remotely to the green commit

## Planning Checkpoint

Stop on API incompatibility, provider-behavior drift, dependency or MSRV
regression, gate failure, source-identity drift, or missing authorization.
