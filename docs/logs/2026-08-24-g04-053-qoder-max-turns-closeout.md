# 2026-08-24 g04.053 Qoder Maximum Turns Closeout

Status: stopped after evidence; awaiting review and merge
Owner: Tom
Milestone: g04.053
Cards: 148-150
Branch: `t3code/qoder-headless-max-turns`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-8e7b4cb9`
Base: `f7593490ccf6f7f1f3cf741c04eb20b8112c09e5` (planning); worker started from
pushed `origin/main` `8a5415fd684246204c9d8ef2a537499bee2a705d`
PR: https://github.com/inflatable-cookie/swallowtail/pull/54

## Result

Card 148 completed an exact evidence pass against
`@qoder-ai/qodercli@1.1.25`. Research 200 admits no deliver-now row. Cards
149 and 150 are blocked and were not executed. The Qoder adapter, prepared
facade, fixtures, guide, and unreleased API baseline remain unchanged except
for Research 200, cards, milestone, and this closeout. No install, login,
credential or account inspection, provider prompt, or paid operation was used.

## Evidence Stop

Exact package registers `--max-turns <count>` and copies a parsed value onto
Config `maxSessionTurns`. QueryEngine.driveQuery passes
`maxTurns: this.config.maxTurns ?? kN` with headless factories hardcoding
`maxTurns: kN` (`kN = 1000`). `getMaxSessionTurns()` is only used for the text
output error line. CLI `--max-turns N` is therefore not proven to enforce N
AgentLoop turns on the selected stream-json print path.

Research 200's deliver-now table is empty. Existing fixed argv `--max-turns 8`
and `error_max_turns` → `swallowtail.qoder.headless.max_turns` fixture truth
remain the claim. No new Contract 029 point. The guide stays unchanged because
card 150, which owns guide capability claims, did not execute.

## Changed Route-Local Surfaces

- `docs/research/200-qoder-headless-max-turns-evidence.md`: promoted exact
  official/package evidence, parser-versus-public domains, counter/wiring gap,
  terminal truth, and empty deliver-now table
- `docs/roadmaps/g04/batch-cards/148-qoder-headless-max-turns-evidence.md`:
  complete; evidence stop
- `docs/roadmaps/g04/batch-cards/149-qoder-headless-max-turns-binding.md`:
  blocked
- `docs/roadmaps/g04/batch-cards/150-qoder-headless-max-turns-acceptance.md`:
  blocked
- `docs/roadmaps/g04/053-qoder-headless-max-turns.md`: evidence stop
- this closeout log

No production `crates/swallowtail-adapter-qoder/src/**` change. Guide, example,
fixtures, and public API baseline unchanged.

## Shared Closeout Delta

Reserved for the orchestrator after review and merge:

- architecture and route/feature matrices: keep Qoder max-turns as not
  deliverable / evidence stop; no new capability row
- Contract 029 disposition: unchanged; exact `1.1.25` qualified-only remains
- programme and research/log/roadmap/g04/batch-card indexes: refresh status for
  Research 200, cards 148-150, milestone 053, and this closeout
- changelog and unreleased public-API baseline: unchanged; nothing shipped
- the sole Next Task and merge identity: move after reviewed stop; no next
  feature family selected by this lane
- g04 remains open until explicit operator direction

## Validation

Card 148 gates: `effigy qa:docs:index:research`,
`effigy qa:docs:index:roadmaps:g04`,
`effigy qa:docs:index:roadmaps:batch-cards`, `git diff --check`.

Inherited doctor baseline unchanged: 376 god-file findings (330 warnings, 46
errors); one generated-in-src warning.

## Next

Evidence PR open: https://github.com/inflatable-cookie/swallowtail/pull/54.
Do not merge from this lane.
Orchestrator reviews, merges, then applies shared closeout and reassesses
remaining inventory.
