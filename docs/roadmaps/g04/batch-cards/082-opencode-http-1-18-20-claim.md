# 082 OpenCode HTTP 1.18.20 Claim

Status: completed
Owner: Tom
Milestone: [g04.029 OpenCode HTTP 1.18.20 Useful Newer](../029-opencode-http-1-18-20-useful-newer.md)
Created: 2026-08-21

## Task

Raise OpenCode HTTP qualified ceiling from `1.18.18` to `1.18.20` after
identity card 081 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-opencode/src/selection.rs`:

- Change `OPENCODE_LATEST_QUALIFIED_VERSION` from `"1.18.18"` to `"1.18.20"`
- Extend `surface-19` through `1.18.20`
- Unit tests: `1.18.19` and `1.18.20` qualified
- Synthetic `UnverifiedNewer`: `1.18.21`

In tests and frozen corpora:

- Raise latest-qualified, release rows, and later-stable fixtures
- Keep decoder specimen `opencode-1.14.48`
- Import / reconcile / history / detach still reject unverified-newer

In docs:

- Update OpenCode prepared-integration guide
- Update this family's route-matrix and feature-matrix rows
- Update architecture OpenCode ceiling mentions
- Add `CHANGELOG.md` Unreleased entry
- Create the claim log
- Index research 176 and the family logs
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Update the g04 milestone/checkpoint, batch-card index, and standing
  currentness pointer without moving the Next Task

## Validation

```sh
cargo fmt -p swallowtail-adapter-opencode
effigy validate:focused swallowtail-adapter-opencode
effigy package:verify-affected swallowtail-adapter-opencode
```

## Acceptance

- Production claim raised to `1.18.20`
- Published intermediate `1.18.19` qualified
- Tests pass
- Family docs updated
- Named gates pass

Auto-continuation: No. Next Task stays on the generation's actual work.

## Out Of Scope

- Gemini requalification (deferred)
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
- Next Task changes or Kimi Platform implementation
