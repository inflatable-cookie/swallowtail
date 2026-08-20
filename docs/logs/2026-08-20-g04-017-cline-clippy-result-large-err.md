# g04.017 Cline Stable Clippy Result Large Err

Date: 2026-08-20
Roadmap: `../roadmaps/g04/017-cline-stable-clippy-result-large-err.md`
Cards: `../roadmaps/g04/batch-cards/048-cline-box-start-session-err.md`,
`../roadmaps/g04/batch-cards/049-cline-clippy-result-large-err-proof.md`
Handoff: `../handoffs/20260820-180328-g04-017-cline-stable-clippy.md`

## Result

`ClineAcpDriver::start_session` now returns
`Result<ClineSessionHandle, Box<(RuntimeFailure, ResourceLease)>>`.
`open_session` still destructures the pair and releases the
working-resource lease on failure. DeepSeek and g04.016 files are
unchanged. `public-api-0.3.3` is unchanged.

The Cline papercut is closed in `PAPERCUTS.md`. Orchestrator records the
merge SHA at closeout.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-8d619c23`
Worker branch: `t3code/follow-cline-stable-clippy-handoff`

Validation:

- Card 048: `effigy validate:focused swallowtail-adapter-cline` (49 tests
  passed), `git diff --check`
- Card 049: `cargo clippy -p swallowtail-adapter-cline --all-targets
  --all-features -- -D warnings` (rustc 1.97.1, exit 0),
  `effigy validate:focused swallowtail-adapter-cline`, `git diff --check`

Local clippy is 1.97.1; CI Stable is rust-clippy 1.98.0. Boxing is the
Clippy-suggested fix.

PR: https://github.com/inflatable-cookie/swallowtail/pull/14

## Next

Await review. Do not merge without operator authorisation. After merge,
restack g04.016 / PR 13. Do not start Claude Agent ACP or llama.cpp.
