# 191 Pi RPC 0.84.3 Claim

Status: completed
Owner: Tom
Milestone: [g04.068 Pi RPC 0.84.3 Useful Newer](../068-pi-rpc-0-84-3-useful-newer.md)
Created: 2026-08-26

## Task

Raise the Pi RPC `pi.package` qualified ceiling from `0.84.2` to official
`0.84.3` after identity card 190 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-pi/src/selection.rs`:

- Change latest-qualified constant from `"0.84.2"` to `"0.84.3"`
- Keep claim id `pi.rpc.package-window-2`
- Keep `AllowUnverified`
- Keep baseline `0.80.10`
- Keep behavior `pi.rpc.strict-lf-v0.84.0-message-update-delta`
- Extend Maintained `0.84.0..=0.84.2` to `0.84.0..=0.84.3`
- Unit tests: `0.84.2` and `0.84.3` qualified; synthetic
  `UnverifiedNewer` is `0.84.4`

In tests:

- Add `0.84.3` identity corpus assertions
- Keep the `0.84.2` specimen and decoder corpus `pi-rpc-0.80.10`
- Move synthetic later-stable UnverifiedNewer to `0.84.4`

In docs:

- Update Pi RPC prepared-integration guide
- Update Pi RPC feature-matrix version column
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Index family research and logs
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Update the g04 milestone/checkpoint and batch-card indexes

## Validation

```sh
cargo fmt -p swallowtail-adapter-pi
effigy validate:focused swallowtail-adapter-pi
effigy package:verify-affected swallowtail-adapter-pi
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Acceptance

- Official `0.84.3` classifies as Qualified Maintained
- `0.80.10` through `0.84.2` remain Qualified at their existing
  revisions
- `0.83.1` remains incompatible
- `0.84.4` remains permitted UnverifiedNewer
- Decoder specimens remain
- Named adapter gates pass

Auto-continuation: No. Next Task stays on the generation's actual work.

## Out Of Scope

- SDK sidecar
- Oh My Pi
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
- Next Task changes, architecture, or contract edits
