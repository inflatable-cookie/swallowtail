# 2026-08-24 g04.053 Qoder Maximum Turns Closeout

Status: paused for operator claim reconciliation; PR 54 review response
Owner: Tom
Milestone: g04.053
Cards: 148-150
Branch: `t3code/qoder-headless-max-turns`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-8e7b4cb9`
Base: restacked on `origin/main` after PR 52/53 (`a2ebf148` lineage)
PR: https://github.com/inflatable-cookie/swallowtail/pull/54

## Result

Card 148 froze exact `@qoder-ai/qodercli@1.1.25` evidence and promoted
Research 200 with an empty deliver-now set, then **paused**: the same evidence
contradicts existing qualified-route max-turns claims. Card 148 is not
complete. Cards 149 and 150 remain blocked. The Qoder adapter, prepared
facade, fixtures, guide, and unreleased API baseline are unchanged. No
install, login, credential or account inspection, provider prompt, or paid
operation was used.

## Evidence And Claim Contradiction

Exact package registers `--max-turns <count>` as a raw string (no Commander
`argParser`) and copies it onto Config `maxSessionTurns`. The selected CLI
headless QueryEngine factory (`entrypoint: "cli"`, headless transcript writer)
hardcodes `maxTurns: kN` (`1000`). QueryEngine.driveQuery passes
`maxTurns: this.config.maxTurns ?? kN`. `getMaxSessionTurns()` is only used
for the text-output error line. Other `maxTurns: kN` literals are ACP,
remote-control, and TUI paths — not this route's decisive site.

Therefore CLI `--max-turns N` is not proven to enforce N AgentLoop turns, and
upstream flag omission is not AgentLoop-unbounded on this path. That
contradicts Research 151 Authority, `command.rs` ("Required positive CLI turn
bound"), the guide ("Must pass `--max-turns 8`"), and fixture fields such as
`require_max_turns` / `omit-max-turns-unbounded`.

Decoder mapping of synthetic `error_max_turns` (`limit.jsonl` `num_turns: 1`)
remains distinct from proving argv `8` produces a limit at turn 8.

Operator must choose reconciliation (corpus rewrite vs qualified-route change)
before card 148 closes. This lane does not preserve the invalidated
assumption or silently edit production surfaces.

## Changed Route-Local Surfaces

- `docs/research/200-qoder-headless-max-turns-evidence.md`: promoted evidence,
  precision fixes, contradiction table, empty deliver-now, operator pause
- `docs/roadmaps/g04/batch-cards/148-qoder-headless-max-turns-evidence.md`:
  blocked; awaiting operator claim reconciliation
- `docs/roadmaps/g04/batch-cards/149-qoder-headless-max-turns-binding.md`:
  blocked
- `docs/roadmaps/g04/batch-cards/150-qoder-headless-max-turns-acceptance.md`:
  blocked
- `docs/roadmaps/g04/053-qoder-headless-max-turns.md`: paused
- this closeout log

No production `crates/swallowtail-adapter-qoder/src/**` change. Guide, example,
fixtures, and public API baseline unchanged pending operator direction.

## Shared Closeout Delta

Reserved for the orchestrator after review and directed reconciliation:

- architecture and route/feature matrices: no new capability row until claims
  settle
- Contract 029 disposition: unchanged; exact `1.1.25` qualified-only remains
  until a route revision is authorized
- programme and research/log/roadmap/g04/batch-card indexes: refresh status for
  Research 200, cards 148-150, milestone 053, and this closeout
- changelog and unreleased public-API baseline: unchanged unless reconciliation
  edits the route
- the sole Next Task and merge identity: move only after reviewed pause /
  reconciliation direction
- g04 remains open until explicit operator direction

## Validation

Card 148 gates: `effigy qa:docs:index:research`,
`effigy qa:docs:index:roadmaps:g04`,
`effigy qa:docs:index:roadmaps:batch-cards`,
`effigy qa:docs:index:logs`, `git diff --check`.

Inherited doctor baseline unchanged: 376 god-file findings (330 warnings, 46
errors); one generated-in-src warning.

## Next

Evidence PR: https://github.com/inflatable-cookie/swallowtail/pull/54.
Do not merge from this lane until review clears the restack and claim pause.
Operator (or orchestrator with operator direction) chooses reconciliation
plan, then either closes card 148 as docs/corpus-only or authorizes a
qualified-route follow-on.
