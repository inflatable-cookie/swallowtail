# 073 Qwen Headless 0.21.15 Claim

Status: completed
Owner: Tom
Milestone: [g04.026 Qwen Headless 0.21.15 Useful Newer](../026-qwen-headless-0-21-15-useful-newer.md)
Created: 2026-08-21

## Task

Raise the Qwen qualified ceiling from `0.21.14` to official `0.21.15`
after identity card 070 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-qwen/src/selection.rs`:

- Change `QWEN_CODE_LATEST_QUALIFIED_VERSION` from `"0.21.14"` to
  `"0.21.15"`
- Keep claim id `qwen-code.headless.package-window-2`
- Keep `AllowUnverified`
- Keep Deprecated `0.19.11..=0.20.1`
- Extend Maintained `0.21.0..=0.21.15` on
  `qwen-code.headless.v0.21.0-catalogue-filter`
- Unit test: `0.21.15` qualified; synthetic `UnverifiedNewer` is
  `0.21.16`

In tests:

- Add `0.21.15` identity corpus assertions
- Keep `0.21.13` and `0.21.14` specimens
- Keep decoder corpus `qwen-code-v0.19.11`

In docs:

- Update Qwen prepared-integration guide
- Update route + feature matrices
- Update architecture if it names this ceiling
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Update research/log/roadmap/g04/batch-card indexes
- Keep Next Task on g04.023; do not keep the generation open for
  currentness

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Acceptance

- Official `0.21.15` classifies as Qualified Maintained
- Exact `0.21.14` remains Qualified
- Unpublished `0.20.2` remains incompatible
- `0.21.16` remains permitted UnverifiedNewer
- Decoder specimens remain
- Named gates pass

Auto-continuation: No. Next Task returns to g04.023.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
