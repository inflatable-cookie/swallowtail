# 088 Kimi Code 0.38.0 Claim

Status: completed
Owner: Tom
Milestone: [g04.032 Kimi Code 0.38.0 Useful Newer](../032-kimi-code-0-38-0-useful-newer.md)
Created: 2026-08-21

## Task

Raise the three Kimi qualified ceilings from `0.37.2` to official
`0.38.0` after identity card 087 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-kimi/src/selection.rs` and
`local_server/selection.rs`:

- Change latest-qualified constants from `"0.37.2"` to `"0.38.0"`
- Keep claim ids and `AllowUnverified`
- Keep ACP / headless / local-server baselines and behavior revisions
- Unit tests: `0.38.0` qualified; synthetic `UnverifiedNewer` is `0.38.1`

In tests:

- Add `0.38.0` identity corpus assertions
- Keep the `0.37.2` specimen
- Keep decoder corpora
- Move synthetic later-stable UnverifiedNewer to `0.38.1`

In docs:

- Update Kimi prepared-integration guide
- Update Kimi route + feature matrix rows
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Index family research and logs
- Do not edit Next Task, `docs/roadmaps/README.md`, g04 front-door
  text, architecture, or contracts

## Validation

```sh
cargo fmt -p swallowtail-adapter-kimi
effigy validate:focused swallowtail-adapter-kimi
effigy package:verify-affected swallowtail-adapter-kimi
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Acceptance

- Official `0.38.0` classifies as Qualified Maintained on all three
  routes
- Exact `0.37.2` remains Qualified
- `0.38.1` remains permitted UnverifiedNewer
- Decoder specimens remain
- Named adapter gates pass

Auto-continuation: No. Do not change Next Task.

## Out Of Scope

- Gemini requalification (deferred)
- Sibling currentness PRs
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Flattening Python `kimi-cli` or Kimi Platform Chat
- Provider work
- Next Task, generation status, g04 front-door, architecture, or
  contract edits
