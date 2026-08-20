# g04.017 Cline Stable Clippy Result Large Err

Date: 2026-08-20
Roadmap: `../roadmaps/g04/017-cline-stable-clippy-result-large-err.md`
Cards: `../roadmaps/g04/batch-cards/048-cline-box-start-session-err.md`,
`../roadmaps/g04/batch-cards/049-cline-clippy-result-large-err-proof.md`
Handoff: `../handoffs/20260820-180328-g04-017-cline-stable-clippy.md`

## Result

ACP `start_session` helpers on Cline, Goose, Copilot CLI, Gemini, Kiro,
and Deep Agents now return
`Result<Handle, Box<(RuntimeFailure, ResourceLease)>>`. Each
`open_session` still destructures the pair and releases the
working-resource lease on failure. DeepSeek and g04.016 files are
unchanged. `public-api-0.3.3` is unchanged.

The ACP papercut stays open until CI Stable clippy is green. After
boxing, 1.98.0 next failed `chunks_exact_to_as_chunks` on the one
`chunks_exact(2)` in ACP lifecycle fixtures; that call is now
`as_chunks::<2>().0`. Orchestrator records the merge SHA at closeout.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-8d619c23`
Worker branch: `t3code/follow-cline-stable-clippy-handoff`
Rebased onto `c779cc4b`.

Validation:

- Card 048: `cargo test --locked -p swallowtail-adapter-goose -p
  swallowtail-adapter-copilot-cli -p swallowtail-adapter-gemini -p
  swallowtail-adapter-kiro -p swallowtail-adapter-deepagents` (exit 0),
  `git diff --check`
- Card 049: `cargo clippy --workspace --all-targets --all-features
  --locked -- -D warnings` (rustc 1.97.1, exit 0), `git diff --check`

Local clippy is 1.97.1; CI Stable is rust-clippy 1.98.0. Boxing is the
Clippy-suggested fix.

PR: https://github.com/inflatable-cookie/swallowtail/pull/14

## Next

Await review. Do not merge without operator authorisation. After merge,
restack g04.016 / PR 13. Do not start Claude Agent ACP or llama.cpp.
