# 152 Codex 0.149.1 Claim

Status: completed
Owner: Tom
Milestone: [g04.054 Codex 0.149.1 Useful Newer](../054-codex-0-149-1-useful-newer.md)
Created: 2026-08-24

## Task

Raise Codex qualified ceiling from `0.149.0` to `0.149.1` after identity
card 151 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-codex/src/selection.rs`:

- Change `CODEX_LATEST_QUALIFIED_VERSION` from `"0.149.0"` to `"0.149.1"`
- Extend latest segments in both claims through `0.149.1`
- Add unit test: `0.149.1` qualified
- Synthetic `UnverifiedNewer`: `0.149.2`

In tests:

- Update discovery/foundation table fixtures if they name the ceiling
- Keep existing decoder corpora (no adapter mapping change)

In docs:

- Update `docs/guides/provider-solution-feature-matrix.csv` Codex column
- Update `docs/guides/provider-route-matrix.md` if it names the ceiling
- Update `docs/guides/codex-prepared-integration.md`
- Add `CHANGELOG.md` Unreleased entry
- Create the claim log
- Update `docs/research/README.md` to index 201
- Update `docs/logs/README.md`
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Update the g04 milestone/checkpoint and batch-card indexes

## Validation

```sh
cargo fmt -p swallowtail-adapter-codex
effigy validate:focused swallowtail-adapter-codex
effigy package:verify-affected swallowtail-adapter-codex
```

## Acceptance

- Production claims raised to `0.149.1`
- Tests pass
- Family docs updated
- Named gates pass

Auto-continuation: No. Next Task stays on the generation's actual work.

## Out Of Scope

- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
