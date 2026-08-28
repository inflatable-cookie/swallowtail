# 2026-08-28 Qwen Headless 0.22.3 Claim

## Result

Card 013 raised `qwen-code.package` through official `0.22.3` as a
compatible extension of
`qwen-code.headless.v0.21.15-reasoning-control`. Keep exact `0.21.15`.
Extend same-revision `0.22.0..=0.22.3`. Qualify published intermediate
`0.22.2`. Catalogue-filter `0.21.0..=0.21.14` stays Deprecated.
Unpublished `0.20.2` and `0.21.16` stay incompatible. Later stables stay
AllowUnverified; the synthetic later point is now `0.22.4`. Reasoning
and budgets stay exact `0.21.15`. Unused 0.22 extras stay unmapped.
Decoder specimen `qwen-code-v0.19.11` stays.

g05.004 is standing currentness, completed. Next Task pointer was left
alone. g05.001-g05.003 stay as they were.

## Validation

Restack onto `main` `70badf07` as Research 258 / g05.004 / cards 012-013:

- `cargo fmt -p swallowtail-adapter-qwen` passed
- `bash scripts/validate-focused-packages.sh swallowtail-adapter-qwen` passed (65 tests, clippy `-D warnings`)
- `bash scripts/verify-affected-packages.sh swallowtail-adapter-qwen` passed
- `bash scripts/check-provider-route-matrix.sh` passed
- Index-equivalent: research, logs, g05 README, g05 batch-cards
- Official `latest` stayed `0.22.3` through closeout

`check-roadmap-status-drift.py` still fails on current `main`'s g05
runway: cards 004 and 007 frontmatter are complete/done while the card
index still lists them Ready. This restack did not rewrite that runway.

No `effigy` binary on this host; the named scripts are the task equivalents.

## Next

Resume the generation's actual Next Task. This family is done.
