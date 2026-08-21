# 083 Oh My Pi 17.4.0 Claim

Status: completed
Owner: Tom
Milestone: [g04.031 Oh My Pi 17.4.0 Useful Newer](../031-oh-my-pi-17-4-0-useful-newer.md)
Created: 2026-08-21
Depends on: card 082; Research 178

## Task

Raise the `oh-my-pi.package` qualified ceiling from exact `17.3.8` to
exact `17.4.0` after identity card 082 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-oh-my-pi/src/selection.rs`:

- Change `OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION` from `"17.3.8"` to
  `"17.4.0"`
- Keep baseline `17.2.9`, claim id, behavior
  `oh-my-pi.rpc-v2-v17.2.9`, and AllowUnverified
- Synthetic `UnverifiedNewer`: `17.4.1`

In tests:

- Add 17.4.0 identity/claim coverage
- Keep decoder specimen `oh-my-pi-rpc-17.2.9`
- Keep frozen `17.3.7` and `17.3.8` corpora

In docs:

- Oh My Pi prepared-integration guide
- This family's route + feature matrix rows
- Architecture and Contract 029 ceiling lines that name this bound
- `CHANGELOG.md` Unreleased line
- Research 178 index, identity/claim logs
- Family cards only. Do not rewrite Next Task or g04 front-door status.

## Validation

```sh
cargo fmt -p swallowtail-adapter-oh-my-pi
effigy validate:focused swallowtail-adapter-oh-my-pi
effigy package:verify-affected swallowtail-adapter-oh-my-pi
```

## Acceptance

- [x] `17.2.9..=17.4.0` classifies as Qualified Maintained
- [x] `17.2.8` remains incompatible
- [x] `17.4.1` remains permitted UnverifiedNewer
- [x] unpublished `17.3.6` stays unpublished
- [x] decoder specimen remains `oh-my-pi-rpc-17.2.9`
- [x] `17.3.7` and `17.3.8` specimens remain
- [x] focused Oh My Pi proof and package verify pass
- [x] matrices and the Oh My Pi guide name the new package ceiling

Auto-continuation: No.

## Out Of Scope

- Gemini requalification (deferred)
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
- Flattening onto Pi RPC
- Next Task / g04 README / generation status edits
