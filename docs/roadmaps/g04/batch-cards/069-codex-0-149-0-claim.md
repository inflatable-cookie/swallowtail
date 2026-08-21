# 069 Codex 0.149.0 Claim

Status: completed
Owner: Tom
Milestone: [g04.025 Codex 0.149.0 Useful Newer](../025-codex-0-149-0-useful-newer.md)
Created: 2026-08-21

## Task

Raise Codex qualified ceiling from `0.148.0` to `0.149.0` after identity
card 068 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-codex/src/selection.rs`:

- Change `CODEX_LATEST_QUALIFIED_VERSION` from `"0.148.0"` to `"0.149.0"`
- Extend latest segments in both claims through `0.149.0`
- Add unit test: `0.149.0` qualified
- Synthetic `UnverifiedNewer`: `0.149.1`

In tests:

- Update discovery/foundation table fixtures if they name the ceiling
- Keep existing decoder corpora (no adapter mapping change)

In docs:

- Update `docs/guides/provider-solution-feature-matrix.csv` codex column
- Update `docs/guides/provider-route-matrix.md` if it names the ceiling
- Update `docs/architecture/system-architecture.md` if it names this ceiling
- Add `CHANGELOG.md` Unreleased entry
- Create the claim log
- Update `docs/logs/README.md`
- Update `docs/research/README.md` to index 172
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Update `docs/roadmaps/g04/README.md` Current Checkpoint and Next Planned
- Update `docs/roadmaps/g04/batch-cards/README.md` Active and Completed

## Validation

```sh
cargo fmt -p swallowtail-adapter-codex
effigy validate:focused swallowtail-adapter-codex
effigy package:verify-affected swallowtail-adapter-codex
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
```

## Acceptance

- Production claims raised to `0.149.0`
- Tests pass
- Docs updated
- All named gates pass

Auto-continuation: No. Next Task returns to the generation's actual work.

## Out Of Scope

- Gemini requalification (deferred)
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
