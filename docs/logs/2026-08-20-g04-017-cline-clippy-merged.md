# g04.017 Cline Clippy Merged

Date: 2026-08-20
Roadmap: `../roadmaps/g04/017-cline-stable-clippy-result-large-err.md`
PR: https://github.com/inflatable-cookie/swallowtail/pull/14

## Result

PR 14 fast-forwarded onto `main` at
`47b94efc707476a90752d8e9cc5ba8a48a3efec4`. Review comment
https://github.com/inflatable-cookie/swallowtail/pull/14#issuecomment-5360253253
is the canonical verdict. `v0.3.3` still peels to `51d18620`.

ACP `start_session` Err pairs are boxed on Cline, Goose, Copilot CLI,
Gemini, Kiro, and Deep Agents. The ACP fixture uses `as_chunks::<2>().0`.
Stable Clippy 1.98.0 is green. The papercut is closed.

## Next

Restack g04.016 / PR 13 onto `47b94efc`. Do not start Claude Agent ACP
addable or llama.cpp.
