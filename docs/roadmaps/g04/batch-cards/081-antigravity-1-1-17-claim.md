# 081 Antigravity 1.1.17 Claim

Status: completed
Owner: Tom
Milestone: [g04.030 Antigravity 1.1.17 Useful Newer](../030-antigravity-1-1-17-useful-newer.md)
Created: 2026-08-21
Depends on: card 080; Research 177

## Task

Raise the `antigravity-cli.release` qualified ceiling from `1.1.15` to
`1.1.17` on both catalogue and headless claims after identity card 080
confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-antigravity/src/selection.rs`:

- Change `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` from `"1.1.15"` to `"1.1.17"`
- Keep claim ids `release-window-1` and both behavior revisions
- Keep `AllowUnverified`
- Unit tests: `1.1.16` and `1.1.17` qualified
- Synthetic `UnverifiedNewer`: `1.1.18`
- Keep `1.1.8` incompatible

In tests:

- Refresh discovery/foundation table: official `1.1.17` qualified,
  synthetic `1.1.18` unverified
- Add `1.1.17` identity corpus assertions
- Keep decoder specimen `antigravity-cli-1.1.9`
- Keep `1.1.14` and `1.1.15` fixtures

In docs:

- Update Antigravity prepared-integration guide
- Update this family's route-matrix and feature-matrix rows
- Update architecture Antigravity ceiling mentions
- Add `CHANGELOG.md` Unreleased entry
- Create the claim log
- Index the family log
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Do not edit `docs/roadmaps/g04/README.md`

## Validation

```sh
cargo fmt -p swallowtail-adapter-antigravity
effigy validate:focused swallowtail-adapter-antigravity
effigy package:verify-affected swallowtail-adapter-antigravity
```

## Acceptance

- [x] official `1.1.17` classifies as Qualified Maintained
- [x] published intermediate `1.1.16` classifies as Qualified Maintained
- [x] `1.1.8` remains incompatible
- [x] `1.1.18` remains permitted UnverifiedNewer
- [x] decoder specimen remains `antigravity-cli-1.1.9`
- [x] `1.1.15` specimens remain
- [x] focused Antigravity proof and package verify named
- [x] family guide and matrix rows name the new release ceiling

## Stop Conditions

- stop if card 080 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `1.1.17` is no longer the official stable point

## Auto-Continuation

No. Next Task stays on the generation's actual work.

## Out Of Scope

- Gemini requalification (deferred)
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping `mcp`, `--input-format`, or Gemini API-key sign-in
- Flattening onto `antigravity-acp` `1.0.0`
- Provider work
- Next Task / generation status edits

## Evidence

- Research 177
- `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.17/`
- Decoder specimen remains `antigravity-cli-1.1.9`
- latest qualified = `1.1.17`
- catalogue behavior = `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`
- headless behavior = `antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`
- synthetic later-stable UnverifiedNewer is `1.1.18`
