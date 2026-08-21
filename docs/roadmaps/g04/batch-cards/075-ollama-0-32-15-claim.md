# 075 Ollama 0.32.15 Claim

Status: completed
Owner: Tom
Milestone: [g04.027 Ollama 0.32.15 Useful Newer](../027-ollama-0-32-15-useful-newer.md)
Created: 2026-08-21

## Task

Raise the Ollama qualified ceiling from `0.32.14` to official `0.32.15`
after identity card 072 confirms compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-ollama/src/selection.rs`:

- Change `OLLAMA_LATEST_QUALIFIED_VERSION` from `"0.32.14"` to
  `"0.32.15"`
- Keep claim id `ollama.native-runtime-window-2`
- Keep `AllowUnverified`
- Keep baseline `0.14.0` and behavior `ollama.native-text-v1`
- Keep exclusions `0.32.2` and `0.32.10`
- Unit test: `0.32.15` qualified; synthetic `UnverifiedNewer` is
  `0.32.16`

In tests:

- Add `0.32.15` identity corpus assertions
- Keep the `0.32.14` specimen
- Keep decoder corpus `ollama-native-v0.14.0-v0.32.1`
- Move synthetic later-stable UnverifiedNewer to `0.32.16`

In docs:

- Update Ollama prepared-integration guide
- Update route + feature matrices
- Update architecture and contracts that name this ceiling
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Update research/log/roadmap/g04/batch-card indexes
- Keep Next Task on g04.023; do not keep the generation open for
  currentness

## Validation

```sh
cargo fmt -p swallowtail-adapter-ollama
effigy validate:focused swallowtail-adapter-ollama
effigy package:verify-affected swallowtail-adapter-ollama
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

- Official `0.32.15` classifies as Qualified Maintained
- Exact `0.32.14` remains Qualified
- `0.32.2` and `0.32.10` remain incompatible
- `0.32.16` remains permitted UnverifiedNewer
- Decoder specimens remain
- Named gates pass

Auto-continuation: No. Next Task returns to g04.023.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Qwen
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Flattening llama.cpp onto Ollama
- Provider work
