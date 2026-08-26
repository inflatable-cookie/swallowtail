# 2026-08-26 Qwen Headless 0.22.1 Claim

## Result

Card 191 raised `qwen-code.package` through official `0.22.1` as a
compatible extension of
`qwen-code.headless.v0.21.15-reasoning-control`. Keep exact `0.21.15`.
Add same-revision `0.22.0..=0.22.1`. Unpublished `0.20.2` and `0.21.16`
stay incompatible. Later stables stay AllowUnverified; the synthetic
later point is now `0.22.2`. Reasoning and budgets stay exact
`0.21.15`. Unused 0.22 extras stay unmapped. Decoder specimen
`qwen-code-v0.19.11` stays.

g04.068 is standing currentness, completed. Next Task pointer and g04
generation status were left alone.

## Validation

- `cargo fmt -p swallowtail-adapter-qwen` passed
- `bash scripts/validate-focused-packages.sh swallowtail-adapter-qwen` passed (56 tests, clippy `-D warnings`)
- `bash scripts/verify-affected-packages.sh swallowtail-adapter-qwen` passed
- Official `latest` stayed `0.22.1` through the run

No `effigy` binary on this host; the named scripts are the task equivalents.

## Next

Resume the generation's actual Next Task. This family is done.
