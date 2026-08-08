# 051 Validation Machinery And Index Closure

Status: planned
Owner: Tom
Created: 2026-08-08
Generation: g03
Depends on: g03.050
Vision tags: validation, repository health, release discipline
Contract refs: 001, 036, 052
Planning state: cards 152-155 completed; card 156 ready

## Problem

A verified deep audit found the machine-checked docs system has gaps and
drift:

- `docs/logs/README.md` is not machine-checked and has drifted: four log files
  are missing from the index
  (`2026-07-26-nucleus-native-pilot-clean-launch-four.md`,
  `2026-07-26-nucleus-native-pilot-clean-launch-three.md`,
  `2026-07-26-nucleus-native-pilot-closeout.md`,
  `2026-08-02-acp-stable-session-list-codec.md`)
- the roadmaps index policy exists (`effigy.toml:13-17`) but no task runs it;
  g01 misses three handoff files and g02 misses one closeout plus links two
  nonexistent files
- `qa:docs`, `qa:northstar`, `package:msrv`, `package:release-floor`, and
  `validate:selectors:test` never run in CI
  (`.github/workflows/ci.yml`)
- the 34-route inventory is duplicated five ways: a heredoc
  (`scripts/check-provider-route-matrix.sh:19-54`), CSV-derived ids
  (`scripts/provider_route_matrix/assertions.py:299-305`), two regex parsers,
  and a frozen baseline (`release-baselines/production-routes-0.2.0.txt`);
  every route touch is a multi-file edit
- retired release scripts hold stale facts (`1.93.0` MSRV,
  `scripts/verify-packages-local.sh:216`; 26-route expectations,
  `verify-candidate-provider-lifecycle.sh:125-128`) and
  `scripts/check-muse-code-corpus.py` is fully unwired dead weight
- release constants (MSRV, tag, package and route counts) are hardcoded in
  roughly fifteen files, so a release bump is a multi-file chore

## Goals

- [ ] make the logs, research, and roadmaps indexes machine-checked and repair
      the verified drift
- [ ] run the doc-policy and MSRV gates in CI
- [ ] single-source the route inventory
- [ ] retire or re-home stale scripts and consolidate release constants where
      cheap

## Execution Plan

- [x] Execute card 152 (docs index checks and drift repair).
- [x] Execute card 153 (doc-policy and tooling gate disposition).
- [x] Execute card 154 (route inventory single-sourcing).
- [x] Execute card 155 (retired-script and constant consolidation).

## Boundaries

- no provider, route, public API, or consumer behavior change
- no change to which facts are authoritative; indexes and gates only enforce
  what the spine already claims
- no tag, release, registry publication, or live provider work

## Acceptance Criteria

- [ ] every `.md` file under `docs/logs/`, `docs/research/`, and
      `docs/roadmaps/` is indexed, and the check runs in CI
- [ ] `qa:docs`, `qa:northstar`, `package:msrv`, `package:release-floor`, and
      `validate:selectors:test` run in CI and pass
- [ ] adding or renaming one route touches exactly one inventory source
- [ ] retired scripts and the unwired corpus validator are archived or deleted
      with their disposition recorded

## Next Planning Checkpoint

The suite planning checkpoint here: reassess evidence-gate posture before the
scaffolding extraction tranches.
