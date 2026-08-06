# 046 v0.2.0 Muse And Rust-Floor Source Release

Status: active
Owner: Tom
Created: 2026-08-06
Depends on: g03.045
Vision tags: source release, Muse Code, breaking MSRV, deterministic CI
Contract refs: 001, 023, 029, 032-033, 036-037, 044-045, 051-052

## Problem

Muse Code is complete on canonical `main`, but consumers can select it only by
commit revision because immutable `v0.1.0` and `v0.1.1` predate the package.
The route and package are additive. Current source also intentionally raises
the unified workspace Rust floor to `1.95.0` and removes Bedrock's separate
floor. Contract 036 classifies an MSRV raise as breaking, so the candidate is
`v0.2.0`, not the previously selected patch. Muse also introduced two new
high-severity structural-size findings that should not enter the tag.

## Generation Runway

Advance g03's source-release and high-value installed-harness goals. Publish
the reviewed 28-package, 34-route source shape and one honest Rust `1.95.0`
floor without changing the source-only distribution boundary or reopening
unrelated structural debt.

## Execution Plan

- [x] card 139: split the two new Muse structural-error files without behavior
      or public API change
- [x] card 140: prepare the complete `v0.2.0` source candidate and pass all
      local release gates
- [ ] card 141: push the exact release commit and require canonical GitHub CI
- [ ] card 142: after separate exact authorization, create and push one
      annotated immutable `v0.2.0` tag

## Goals

- [ ] give Muse consumers one stable exact source tag
- [x] preserve all existing package APIs and guaranteed route behavior
- [x] promote Muse from separate unreleased evidence into the coordinated
      release package, route, and semantic API baselines
- [x] publish the intentional unified Rust `1.95.0` floor as a breaking minor
      release with explicit upgrade and rollback guidance
- [ ] leave `v0.1.0` and `v0.1.1` immutable
- [x] avoid carrying the two new Muse structural errors into the tag

## Boundaries

- no crates.io publication, GitHub Release object, binary, sidecar, installer,
  model artifact, or consumer mutation
- no live or authenticated provider work
- no cleanup of the 22 structural errors inherited from earlier tags
- no release-gate bypass, workflow edit, or timeout inflation
- no tag before the exact release commit passes canonical GitHub CI
- no tag mutation without a separate exact operator authorization

## Acceptance Criteria

- [x] Muse contributes no error-severity structural-size finding
- [x] all 28 packages share `0.2.0` and remain source-only
- [x] all 11 source-release gates pass locally
- [x] every package passes at the exact unified Rust `1.95.0` floor
- [x] the isolated Git-source consumer resolves the 28-package candidate
- [ ] all five GitHub CI jobs pass against the exact release commit
- [x] release notes describe Muse, upgrade, rollback, limits, and unchanged
      publication posture
- [ ] annotated `v0.2.0` resolves locally and remotely to the green commit

## Planning Checkpoint

Stop on public API incompatibility, provider-behavior drift, dependency-floor
regression, local gate failure, or CI failure. A failed or partially published
tag is never moved or recreated.
