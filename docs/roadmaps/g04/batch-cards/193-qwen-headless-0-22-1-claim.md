# 193 Qwen Headless 0.22.1 Claim

Status: completed
Owner: Tom
Milestone: [g04.069 Qwen Headless 0.22.1 Useful Newer](../069-qwen-headless-0-22-1-useful-newer.md)
Created: 2026-08-26

## Task

Raise the Qwen headless `qwen-code.package` qualified ceiling from
`0.21.15` to official `0.22.1` after identity card 192 confirms
compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-qwen/src/selection.rs`:

- Change latest-qualified constant from `"0.21.15"` to `"0.22.1"`
- Keep claim id `qwen-code.headless.package-window-2`
- Keep `AllowUnverified`
- Keep baseline `0.19.11`
- Keep exact `0.21.15` on
  `qwen-code.headless.v0.21.15-reasoning-control`
- Add Maintained `0.22.0..=0.22.1` on the same revision
- Mark `0.21.0..=0.21.14` catalogue-filter `Deprecated`
- Keep `0.19.11..=0.20.1` `Deprecated`
- Keep unpublished `0.20.2` and `0.21.16` incompatible
- Unit tests: `0.21.15`, `0.22.0`, and `0.22.1` qualified Maintained;
  catalogue-filter Deprecated; `0.21.16` rejected; synthetic
  `UnverifiedNewer` is `0.22.2`

In tests:

- Add `0.22.1` identity corpus assertions
- Keep the `0.21.15` specimen and decoder corpus `qwen-code-v0.19.11`
- Keep reasoning and budget fixtures exact `0.21.15`
- Prove ordinary `0.22.0`/`0.22.1` operations reject selected reasoning
  and selected turn/tool budgets
- Move synthetic later-stable UnverifiedNewer to `0.22.2`

In docs:

- Update Qwen headless prepared-integration guide
- Update Qwen route + feature matrix rows, including Contract 029
  support labels
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Index family research, logs, g04 README, batch-cards README, and
  generation-index
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Keep g04 open

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Acceptance

- Official `0.22.1` classifies as Qualified Maintained
- Published `0.22.0` classifies as Qualified Maintained
- `0.21.15` remains Qualified on reasoning-control
- `0.21.0..=0.21.14` classifies as Qualified Deprecated
- `0.19.11..=0.20.1` remains Qualified Deprecated
- `0.21.16` remains incompatible
- `0.22.2` remains permitted UnverifiedNewer
- Reasoning and budgets stay exact `0.21.15`
- Ordinary `0.22.0`/`0.22.1` operations reject selected reasoning and
  selected turn/tool budgets
- Decoder specimens remain
- Named adapter gates pass
- Milestone and cards are indexed

Auto-continuation: No. Next Task stays on the generation's actual work.

## Out Of Scope

- Extending reasoning or budgets past exact `0.21.15`
- Gemini
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Provider work
- Next Task changes, architecture, or contract edits
